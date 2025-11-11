#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod services;
mod models;

use tauri::Manager;
use services::database::Database;

#[tokio::main]
async fn main() {
    let db = Database::new().expect("Failed to initialize database");
    
    tauri::Builder::default()
        .manage(db)
        .invoke_handler(tauri::generate_handler![
            commands::spotify::spotify_authenticate,
            commands::spotify::spotify_get_playlists,
            commands::spotify::spotify_get_tracks,
            commands::soundcloud::soundcloud_authenticate,
            commands::soundcloud::soundcloud_get_tracks,
            commands::shikimori::shikimori_authenticate,
            commands::shikimori::shikimori_get_anime_list,
            commands::discord::discord_authenticate,
            commands::discord::discord_send_message,
            commands::player::play_track,
            commands::player::pause_track,
            commands::player::set_volume
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
