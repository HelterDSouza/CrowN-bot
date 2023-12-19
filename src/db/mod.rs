pub mod models;
pub mod repos;

use std::str::FromStr;

use anyhow::Result;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Pool, Sqlite,
};

use crate::config::Config;

pub async fn run_migration(pool: &Pool<Sqlite>) {
    tracing::info!("Initiating database migration process");
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .expect("Error Migration");
    tracing::info!("Database migration process completed successfully");
}

pub async fn initialize_database(config: &Config) -> Result<Pool<Sqlite>> {
    tracing::info!("Database initialization in progress");
    tracing::debug!("{}", config.db_url());

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(SqliteConnectOptions::from_str(config.db_url())?.create_if_missing(true))
        .await?;

    run_migration(&pool).await;
    tracing::info!("Database initialization completed successfully");

    Ok(pool)
}
