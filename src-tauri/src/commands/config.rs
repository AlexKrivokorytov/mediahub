use tauri::State;
use crate::services::database::Database;

#[derive(serde::Serialize)]
pub struct ConfigStatus {
    pub spotify_client: bool,
    pub soundcloud: bool,
    pub shikimori: bool,
    pub discord: bool,
}

#[tauri::command]
pub async fn get_config_status(db: State<'_, Database>) -> Result<ConfigStatus, String> {
    let has_spotify_client = db
        .get_credentials("spotify_client")
        .await
        .map_err(|e| e.to_string())?
        .is_some();
    let has_soundcloud = db
        .get_credentials("soundcloud")
        .await
        .map_err(|e| e.to_string())?
        .is_some();
    let has_shikimori = db
        .get_credentials("shikimori")
        .await
        .map_err(|e| e.to_string())?
        .is_some();
    let has_discord = db
        .get_credentials("discord")
        .await
        .map_err(|e| e.to_string())?
        .is_some();
    Ok(ConfigStatus {
        spotify_client: has_spotify_client,
        soundcloud: has_soundcloud,
        shikimori: has_shikimori,
        discord: has_discord,
    })
}
