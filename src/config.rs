use anyhow::{Context, Result};

pub struct Config {
    pub matrix_homeserver_url: String,
    pub matrix_user_id: String,
    pub matrix_password: String,
    pub matrix_room_alias: String,
    pub database_url: String,
    pub webhook_listen_addr: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            matrix_homeserver_url: std::env::var("MATRIX_HOMESERVER_URL")
                .context("MATRIX_HOMESERVER_URL must be set")?,
            matrix_user_id: std::env::var("MATRIX_USER_ID")
                .context("MATRIX_USER_ID must be set")?,
            matrix_password: std::env::var("MATRIX_PASSWORD")
                .context("MATRIX_PASSWORD must be set")?,
            matrix_room_alias: std::env::var("MATRIX_ROOM_ALIAS")
                .context("MATRIX_ROOM_ALIAS must be set")?,
            database_url: std::env::var("DATABASE_URL")
                .context("DATABASE_URL must be set")?,
            webhook_listen_addr: std::env::var("WEBHOOK_LISTEN_ADDR")
                .unwrap_or_else(|_| "0.0.0.0:8080".to_string()),
        })
    }
}
