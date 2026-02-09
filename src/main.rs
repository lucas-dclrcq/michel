use std::sync::Arc;

use anyhow::{Context, Result};
use axum::routing::post;
use axum::Router;
use sqlx::PgPool;
use tracing::info;

use homelab_bot::config;
use homelab_bot::db;
use homelab_bot::matrix;
use homelab_bot::webhook;
use homelab_bot::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config = config::Config::from_env()?;

    let pool = PgPool::connect(&config.database_url)
        .await
        .context("Failed to connect to PostgreSQL")?;
    db::run_migrations(&pool).await?;
    info!("Database connected and migrations applied");

    let client = matrix::create_and_login(
        &config.matrix_homeserver_url,
        &config.matrix_user_id,
        &config.matrix_password,
    )
    .await?;

    let (room, _room_id) = matrix::join_room(&client, &config.matrix_room_alias).await?;

    let state = Arc::new(AppState { room, db: pool });

    let app = Router::new()
        .route("/webhook/seerr", post(webhook::handle_seerr_webhook))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&config.webhook_listen_addr)
        .await
        .context("Failed to bind listener")?;
    info!("Webhook server listening on {}", config.webhook_listen_addr);

    axum::serve(listener, app)
        .await
        .context("Server error")?;

    Ok(())
}
