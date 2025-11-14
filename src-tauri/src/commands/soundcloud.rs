use crate::services::database::{Database, UserCredentials};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Serialize, Deserialize)]
pub struct SoundcloudAuthRequest {
	pub client_id: String,
	pub redirect_uri: String,
}

#[tauri::command]
pub async fn soundcloud_authenticate(
	request: SoundcloudAuthRequest,
	db: State<'_, Database>,
) -> Result<String, String> {
	let creds = UserCredentials {
		service: "soundcloud".to_string(),
		access_token: request.client_id,
		refresh_token: None,
		expires_at: None,
	};

	db.save_credentials(creds)
		.await
		.map_err(|e| e.to_string())?;

	Ok("SoundCloud connected successfully".to_string())
}

#[tauri::command]
pub async fn soundcloud_get_tracks(
	db: State<'_, Database>,
) -> Result<Vec<serde_json::Value>, String> {
	let creds = db
		.get_credentials("soundcloud")
		.await
		.map_err(|e| e.to_string())?
		.ok_or("SoundCloud not authenticated")?;

	let client = reqwest::Client::new();
	let response = client
		.get("https://api.soundcloud.com/me/tracks")
		.query(&[("client_id", creds.access_token)])
		.send()
		.await
		.map_err(|e| e.to_string())?;

	if !response.status().is_success() {
		return Err(format!("SoundCloud API error: {}", response.status()));
	}

	let data: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;

	Ok(vec![data])
}



