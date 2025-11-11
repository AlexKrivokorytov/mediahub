#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod services;
mod models;

use services::database::Database;
use services::player::{start_audio_engine, PlayerHandle};
use crate::commands::spotify::SpotifyAuthMemory;

#[tokio::main]
async fn main() {
    let db = Database::new().expect("Failed to initialize database");
    let _ = dotenvy::dotenv();
    // Load credentials from environment if present and persist to DB
    if let Ok(client_id) = std::env::var("SPOTIFY_CLIENT_ID") {
        let secret = std::env::var("SPOTIFY_CLIENT_SECRET").ok();
        let creds = services::database::UserCredentials {
            service: "spotify_client".into(),
            access_token: client_id,
            refresh_token: secret,
            expires_at: None,
        };
        let _ = db.save_credentials(creds);
    }
    if let Ok(scid) = std::env::var("SOUNDCLOUD_CLIENT_ID") {
        let creds = services::database::UserCredentials {
            service: "soundcloud".into(),
            access_token: scid,
            refresh_token: None,
            expires_at: None,
        };
        let _ = db.save_credentials(creds);
    }
    if let Ok(tok) = std::env::var("SHIKIMORI_TOKEN") {
        let creds = services::database::UserCredentials {
            service: "shikimori".into(),
            access_token: tok,
            refresh_token: None,
            expires_at: None,
        };
        let _ = db.save_credentials(creds);
    }
    if let Ok(bot) = std::env::var("DISCORD_BOT_TOKEN") {
        let creds = services::database::UserCredentials {
            service: "discord".into(),
            access_token: bot,
            refresh_token: None,
            expires_at: None,
        };
        let _ = db.save_credentials(creds);
    }
    let player: PlayerHandle = start_audio_engine();
    let spotify_mem = SpotifyAuthMemory::default();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(db)
        .manage(player)
        .manage(spotify_mem)
        .invoke_handler(tauri::generate_handler![
            commands::config::get_config_status,
            commands::spotify::spotify_authenticate,
            commands::spotify::spotify_begin_auth,
            commands::spotify::spotify_complete_auth,
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
