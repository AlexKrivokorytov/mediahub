use tauri::State;
use serde::{Deserialize, Serialize};
use crate::services::database::{Database, UserCredentials};
use chrono::{Utc, Duration};

const AUTH_URL: &str = "https://accounts.spotify.com/authorize";
const TOKEN_URL: &str = "https://accounts.spotify.com/api/token";

#[derive(Serialize, Deserialize)]
pub struct SpotifyAuthRequest {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

#[derive(Default)]
pub struct SpotifyAuthMemory {
    pub code_verifier: std::sync::Mutex<Option<String>>, 
    pub redirect_uri: std::sync::Mutex<Option<String>>, 
}

fn build_auth_url(client_id: &str, redirect_uri: &str, state: &str, code_challenge: &str) -> String {
    let scope = [
        "user-read-email",
        "user-read-private",
        "playlist-read-private",
        "user-library-read",
    ].join(" ");
    let params = vec![
        ("response_type", "code"),
        ("client_id", client_id),
        ("redirect_uri", redirect_uri),
        ("scope", &scope),
        ("state", state),
        ("code_challenge_method", "S256"),
        ("code_challenge", code_challenge),
    ];
    let query = serde_urlencoded::to_string(&params).unwrap();
    format!("{}?{}", AUTH_URL, query)
}

// Returns an authorization URL (PKCE). Frontend may open it in a browser.
#[tauri::command]
pub async fn spotify_authenticate(
    request: SpotifyAuthRequest,
    db: State<'_, Database>,
    mem: State<'_, SpotifyAuthMemory>,
) -> Result<String, String> {
    // If no client id provided, try using backend-stored credentials
    if request.client_id.trim().is_empty() {
        let creds = db.get_credentials("spotify_client").await.map_err(|e| e.to_string())?
            .ok_or_else(|| "Spotify client credentials not set on backend".to_string())?;
        let client_id = creds.access_token;
        let (pkce_challenge, pkce_verifier) = oauth2::PkceCodeChallenge::new_random_sha256();
        let state = oauth2::CsrfToken::new_random();
        *mem.code_verifier.lock().unwrap() = Some(pkce_verifier.secret().to_string());
        *mem.redirect_uri.lock().unwrap() = Some(request.redirect_uri.clone());
        return Ok(build_auth_url(&client_id, &request.redirect_uri, state.secret(), pkce_challenge.as_str()));
    }

    // Save client credentials in DB (temporary, until token exchange completes)
    db.save_credentials(UserCredentials {
        service: "spotify_client".into(),
        access_token: request.client_id.clone(),
        refresh_token: Some(request.client_secret.clone()),
        expires_at: None,
    }).await.map_err(|e| e.to_string())?;

    // Prepare PKCE using a spec-compliant random verifier (43-128 chars)
    let (pkce_challenge, pkce_verifier) = oauth2::PkceCodeChallenge::new_random_sha256();
    let state = oauth2::CsrfToken::new_random();

    // Persist verifier and redirect in memory
    *mem.code_verifier.lock().unwrap() = Some(pkce_verifier.secret().to_string());
    *mem.redirect_uri.lock().unwrap() = Some(request.redirect_uri.clone());

    // Return URL to open in browser
    let url = build_auth_url(&request.client_id, &request.redirect_uri, state.secret(), pkce_challenge.as_str());
    Ok(url)
}

// Starts OAuth using server-stored client credentials and an optional redirect URI.
#[tauri::command]
pub async fn spotify_begin_auth(
    db: State<'_, Database>,
    mem: State<'_, SpotifyAuthMemory>,
    redirect_uri: Option<String>,
) -> Result<String, String> {
    let creds = db.get_credentials("spotify_client").await.map_err(|e| e.to_string())?
        .ok_or_else(|| "Spotify client credentials not set on backend".to_string())?;
    let client_id = creds.access_token;
    let redirect = redirect_uri.unwrap_or_else(|| "http://127.0.0.1:3000".to_string());

    let (pkce_challenge, pkce_verifier) = oauth2::PkceCodeChallenge::new_random_sha256();
    let state = oauth2::CsrfToken::new_random();
    *mem.code_verifier.lock().unwrap() = Some(pkce_verifier.secret().to_string());
    *mem.redirect_uri.lock().unwrap() = Some(redirect.clone());
    Ok(build_auth_url(&client_id, &redirect, state.secret(), pkce_challenge.as_str()))
}

#[derive(Serialize, Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: i64,
    refresh_token: Option<String>,
}

#[tauri::command]
pub async fn spotify_complete_auth(
    code: String,
    db: State<'_, Database>,
    mem: State<'_, SpotifyAuthMemory>,
) -> Result<String, String> {
    // Load stored client creds
    let creds = db.get_credentials("spotify_client").await.map_err(|e| e.to_string())?
        .ok_or_else(|| "Spotify client credentials not set".to_string())?;
    let client_id = creds.access_token;

    // Load verifier + redirect
    let verifier = mem.code_verifier.lock().unwrap().clone().ok_or("Missing code verifier".to_string())?;
    let redirect_uri = mem.redirect_uri.lock().unwrap().clone().ok_or("Missing redirect uri".to_string())?;

    let client = reqwest::Client::new();
    let params = [
        ("grant_type", "authorization_code"),
        ("code", code.as_str()),
        ("redirect_uri", redirect_uri.as_str()),
        ("client_id", client_id.as_str()),
        ("code_verifier", verifier.as_str()),
    ];

    let resp = client
        .post(TOKEN_URL)
        .form(&params)
        .send().await.map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("Spotify token exchange failed: {}", resp.status()));
    }
    let tr: TokenResponse = resp.json().await.map_err(|e| e.to_string())?;

    let expires_at = Utc::now() + Duration::seconds(tr.expires_in);
    db.save_credentials(UserCredentials {
        service: "spotify".into(),
        access_token: tr.access_token,
        refresh_token: tr.refresh_token,
        expires_at: Some(expires_at.timestamp()),
    }).await.map_err(|e| e.to_string())?;

    Ok("Spotify authenticated".into())
}

async fn refresh_access_token(db: &Database) -> Result<String, String> {
    let token_row = db.get_credentials("spotify").await.map_err(|e| e.to_string())?
        .ok_or_else(|| "Spotify not authenticated".to_string())?;
    let refresh = token_row.refresh_token.ok_or_else(|| "No refresh token stored. Re-authenticate.".to_string())?;
    let client_row = db.get_credentials("spotify_client").await.map_err(|e| e.to_string())?
        .ok_or_else(|| "Missing client credentials. Save in Settings.".to_string())?;
    let client_id = client_row.access_token;
    let client_secret_opt = client_row.refresh_token;

    let client = reqwest::Client::new();
    let mut req = client.post(TOKEN_URL);
    if let Some(secret) = &client_secret_opt {
        req = req.basic_auth(&client_id, Some(secret));
    }
    let params = [
        ("grant_type", "refresh_token"),
        ("refresh_token", refresh.as_str()),
        ("client_id", client_id.as_str()),
    ];
    let resp = req.form(&params).send().await.map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("Spotify refresh failed: {}", resp.status()));
    }
    let tr: TokenResponse = resp.json().await.map_err(|e| e.to_string())?;
    let expires_at = Utc::now() + Duration::seconds(tr.expires_in);
    db.save_credentials(UserCredentials {
        service: "spotify".into(),
        access_token: tr.access_token,
        refresh_token: tr.refresh_token.or(Some(refresh)),
        expires_at: Some(expires_at.timestamp()),
    }).await.map_err(|e| e.to_string())?;
    Ok(db.get_credentials("spotify").await.map_err(|e| e.to_string())?.unwrap().access_token)
}

async fn ensure_access_token(db: &Database) -> Result<String, String> {
    if let Some(row) = db.get_credentials("spotify").await.map_err(|e| e.to_string())? {
        if let Some(exp) = row.expires_at {
            if Utc::now().timestamp() < exp - 60 { return Ok(row.access_token); }
        }
        return refresh_access_token(db).await;
    }
    Err("Spotify not authenticated".into())
}

#[tauri::command]
pub async fn spotify_get_playlists(db: State<'_, Database>) -> Result<Vec<String>, String> {
    let token = ensure_access_token(&db).await?;
    let client = reqwest::Client::new();
    let resp = client
        .get("https://api.spotify.com/v1/me/playlists?limit=20")
        .bearer_auth(token)
        .send().await.map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        if resp.status().as_u16() == 401 { return Err("Spotify unauthorized. Re-authenticate.".into()); }
        return Err(format!("Spotify API error: {}", resp.status()));
    }
    let json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
    let mut names = Vec::new();
    if let Some(items) = json.get("items").and_then(|v| v.as_array()) {
        for it in items { if let Some(n) = it.get("name").and_then(|n| n.as_str()) { names.push(n.to_string()); } }
    }
    Ok(names)
}

#[tauri::command]
pub async fn spotify_get_tracks(db: State<'_, Database>) -> Result<Vec<String>, String> {
    let token = ensure_access_token(&db).await?;
    let client = reqwest::Client::new();
    let resp = client
        .get("https://api.spotify.com/v1/me/tracks?limit=20")
        .bearer_auth(token)
        .send().await.map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        if resp.status().as_u16() == 401 { return Err("Spotify unauthorized. Re-authenticate.".into()); }
        return Err(format!("Spotify API error: {}", resp.status()));
    }
    let json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
    let mut tracks = Vec::new();
    if let Some(items) = json.get("items").and_then(|v| v.as_array()) {
        for it in items {
            if let Some(track) = it.get("track") {
                let name = track.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
                let artists = track.get("artists").and_then(|v| v.as_array()).unwrap_or(&vec![]).iter()
                    .filter_map(|a| a.get("name").and_then(|n| n.as_str()))
                    .collect::<Vec<_>>().join(", ");
                tracks.push(format!("{} - {}", name, artists));
            }
        }
    }
    Ok(tracks)
}


