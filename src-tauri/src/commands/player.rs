use tauri::State;
use crate::services::player::PlayerHandle;

#[tauri::command]
pub async fn play_track(track_url: String, volume: Option<f32>, player: State<'_, PlayerHandle>) -> Result<String, String> {
    let vol = volume.unwrap_or(0.5);
    player.play_url(track_url, vol)?;
    Ok("Track playing".to_string())
}

#[tauri::command]
pub async fn pause_track(player: State<'_, PlayerHandle>) -> Result<String, String> {
    player.pause()?;
    Ok("Track paused".to_string())
}

#[tauri::command]
pub async fn set_volume(volume: f32, player: State<'_, PlayerHandle>) -> Result<String, String> {
    player.set_volume(volume)?;
    Ok(format!("Volume set to {}", volume))
}


