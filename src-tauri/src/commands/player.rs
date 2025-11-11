#[tauri::command]
pub async fn play_track(track_url: String) -> Result<String, String> {
	// TODO: Integrate with audio playback backend.
	let _ = track_url;
	Ok("Track playing".to_string())
}

#[tauri::command]
pub async fn pause_track() -> Result<String, String> {
	Ok("Track paused".to_string())
}

#[tauri::command]
pub async fn set_volume(volume: f32) -> Result<String, String> {
	Ok(format!("Volume set to {}", volume))
}


