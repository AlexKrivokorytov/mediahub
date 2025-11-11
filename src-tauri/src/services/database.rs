use rusqlite::{params, Connection};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Clone)]
pub struct Database {
    conn: std::sync::Arc<Mutex<Connection>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserCredentials {
    pub service: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: Option<i64>,
}

impl Database {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("media_hub.db")?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS credentials (
                service TEXT PRIMARY KEY,
                access_token TEXT NOT NULL,
                refresh_token TEXT,
                expires_at INTEGER
            )", 
            []
        )?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS play_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                track_id TEXT NOT NULL,
                track_title TEXT NOT NULL,
                artist TEXT NOT NULL,
                played_at INTEGER NOT NULL
            )", 
            []
        )?;
        
        Ok(Self { conn: std::sync::Arc::new(Mutex::new(conn)) })
    }

    pub fn save_credentials(&self, creds: UserCredentials) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO credentials (service, access_token, refresh_token, expires_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![
                creds.service,
                creds.access_token,
                creds.refresh_token,
                creds.expires_at
            ],
        )?;
        Ok(())
    }

    pub fn get_credentials(&self, service: &str) -> Result<Option<UserCredentials>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT service, access_token, refresh_token, expires_at FROM credentials WHERE service = ?1"
        )?;
        let mut rows = stmt.query([service])?;
        
        if let Some(row) = rows.next()? {
            Ok(Some(UserCredentials {
                service: row.get(0)?,
                access_token: row.get(1)?,
                refresh_token: row.get(2)?,
                expires_at: row.get(3)?,
            }))
        } else {
            Ok(None)
        }
    }
}
