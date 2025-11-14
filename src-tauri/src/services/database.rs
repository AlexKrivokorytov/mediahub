use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio_postgres::{Client, NoTls};

pub struct Database {
    client: Client,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserCredentials {
    pub service: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: Option<i64>,
}

impl Database {
    pub async fn new() -> Result<Self> {
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set for Postgres connection");

        let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

        // Spawn the connection task to drive the I/O in the background.
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Postgres connection error: {}", e);
            }
        });

        client
            .batch_execute(
                "
            CREATE TABLE IF NOT EXISTS users (
                id SERIAL PRIMARY KEY,
                username TEXT NOT NULL UNIQUE
            );

            CREATE TABLE IF NOT EXISTS credentials (
                service TEXT PRIMARY KEY,
                access_token TEXT NOT NULL,
                refresh_token TEXT,
                expires_at BIGINT
            );

            CREATE TABLE IF NOT EXISTS play_history (
                id SERIAL PRIMARY KEY,
                track_id TEXT NOT NULL,
                track_title TEXT NOT NULL,
                artist TEXT NOT NULL,
                played_at BIGINT NOT NULL
            );
            ",
            )
            .await?;

        Ok(Self { client })
    }

    pub async fn save_credentials(&self, creds: UserCredentials) -> Result<()> {
        self.client
            .execute(
                "INSERT INTO credentials (service, access_token, refresh_token, expires_at)
             VALUES ($1, $2, $3, $4)
             ON CONFLICT (service) DO UPDATE SET
                 access_token = EXCLUDED.access_token,
                 refresh_token = EXCLUDED.refresh_token,
                 expires_at = EXCLUDED.expires_at",
                &[
                    &creds.service,
                    &creds.access_token,
                    &creds.refresh_token,
                    &creds.expires_at,
                ],
            )
            .await?;
        Ok(())
    }

    pub async fn get_credentials(&self, service: &str) -> Result<Option<UserCredentials>> {
        let row_opt = self
            .client
            .query_opt(
                "SELECT service, access_token, refresh_token, expires_at FROM credentials WHERE service = $1",
                &[&service],
            )
            .await?;

        if let Some(row) = row_opt {
            Ok(Some(UserCredentials {
                service: row.get(0),
                access_token: row.get(1),
                refresh_token: row.get(2),
                expires_at: row.get(3),
            }))
        } else {
            Ok(None)
        }
    }
}
