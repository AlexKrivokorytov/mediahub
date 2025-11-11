use crate::services::database::{Database, UserCredentials};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Serialize, Deserialize)]
pub struct ShikimoriAuthRequest {
	pub access_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct AnimeEntry {
	pub id: i32,
	pub title: String,
	pub episodes_watched: i32,
	pub total_episodes: i32,
	pub status: String,
	pub rating: Option<f32>,
}

#[tauri::command]
pub async fn shikimori_authenticate(
	request: ShikimoriAuthRequest,
	db: State<'_, Database>,
) -> Result<String, String> {
	let creds = UserCredentials {
		service: "shikimori".to_string(),
		access_token: request.access_token,
		refresh_token: None,
		expires_at: None,
	};

	db.save_credentials(creds)
		.map_err(|e| e.to_string())?;

	Ok("Shikimori connected successfully".to_string())
}

#[tauri::command]
pub async fn shikimori_get_anime_list(
	db: State<'_, Database>,
) -> Result<Vec<AnimeEntry>, String> {
	let creds = db
		.get_credentials("shikimori")
		.map_err(|e| e.to_string())?
		.ok_or("Shikimori not authenticated")?;

	let client = reqwest::Client::new();
	let response = client
		.get("https://shikimori.one/api/users/whoami/anime_rates")
		.header("Authorization", format!("Bearer {}", creds.access_token))
		.send()
		.await
		.map_err(|e| e.to_string())?;

	if !response.status().is_success() {
		return Err(format!(
			"Shikimori API error: {}",
			response.status()
		));
	}

	Ok(vec![])
}



