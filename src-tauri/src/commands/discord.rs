use crate::services::database::{Database, UserCredentials};
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Serialize, Deserialize)]
pub struct DiscordAuthRequest {
	pub bot_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct DiscordMessageRequest {
	pub channel_id: String,
	pub content: String,
}

#[tauri::command]
pub async fn discord_authenticate(
	request: DiscordAuthRequest,
	db: State<'_, Database>,
) -> Result<String, String> {
	let creds = UserCredentials {
		service: "discord".to_string(),
		access_token: request.bot_token,
		refresh_token: None,
		expires_at: None,
	};

	db.save_credentials(creds)
		.map_err(|e| e.to_string())?;

	Ok("Discord connected successfully".to_string())
}

#[tauri::command]
pub async fn discord_send_message(
	request: DiscordMessageRequest,
	db: State<'_, Database>,
) -> Result<String, String> {
	let creds = db
		.get_credentials("discord")
		.map_err(|e| e.to_string())?
		.ok_or("Discord not authenticated")?;

	let client = reqwest::Client::new();
	let url = format!(
		"https://discord.com/api/v10/channels/{}/messages",
		request.channel_id
	);

	let body = serde_json::json!({ "content": request.content });

	let response = client
		.post(&url)
		.header("Authorization", format!("Bot {}", creds.access_token))
		.header("Content-Type", "application/json")
		.json(&body)
		.send()
		.await
		.map_err(|e| e.to_string())?;

	if !response.status().is_success() {
		return Err(format!("Discord API error: {}", response.status()));
	}

	Ok("Message sent successfully".to_string())
}



