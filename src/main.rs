mod config;
mod data;
mod db;
mod handler;

use anyhow::Result;
use config::Config;
use data::DatabasePool;
use serenity::{
    all::GatewayIntents,
    framework::{standard::Configuration, StandardFramework},
    Client,
};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Pool, Sqlite,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::config;

mod config;
mod db;
mod handler;

async fn initialize_framework() -> StandardFramework {
    let framework = StandardFramework::new();
    framework.configure(Configuration::new().prefix("$").delimiter("$"));
    framework
}
async fn initialize_intents() -> GatewayIntents {
    GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT
}

async fn run_migration(pool: &Pool<Sqlite>) {
    tracing::info!("Initiating database migration process");
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .expect("Error Migration");
    tracing::info!("Database migration process completed successfully");
}

async fn initialize_database(config: &Config) -> Result<Pool<Sqlite>> {
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

fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                // .unwrap_or_else(|_| "crown_bot=info".into()),
                .unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = config().await;

    init_tracing();

    let fw = initialize_framework().await;

    let intents = initialize_intents().await;

    let pool = initialize_database(config).await?;

    let mut client = Client::builder(config.token(), intents)
        .event_handler(handler::Handler)
        .framework(fw)
        .await?;

    {
        let mut data = client.data.write().await;

        data.insert::<DatabasePool>(pool);
    }

    if let Err(err) = client.start().await {
        println!("error: {:?}", err)
    }

    Ok(())
}
