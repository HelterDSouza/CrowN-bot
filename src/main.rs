use anyhow::Result;
use config::Config;
use serenity::{
    all::GatewayIntents,
    framework::{standard::Configuration, StandardFramework},
    Client,
};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Pool, Sqlite,
};
use crate::config::config;
mod config;
async fn initialize_framework() -> StandardFramework {
    let framework = StandardFramework::new();
    framework.configure(Configuration::new().prefix("$").delimiter("$"));
    framework
}
async fn initialize_intents() -> GatewayIntents {
    GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT
}
async fn run_migration(pool: &Pool<Sqlite>) {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .expect("Error Migration");
}
async fn initialize_database(config: &Config) -> Result<Pool<Sqlite>> {

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(SqliteConnectOptions::from_str(config.db_url())?.create_if_missing(true))
        .await?;

    run_migration(&pool).await;

    Ok(pool)
}
#[tokio::main]
async fn main() -> Result<()> {
    let config = config().await;


    let fw = initialize_framework().await;

    let intents = initialize_intents().await;

    let pool = initialize_database(config).await?;

    let mut client = Client::builder(config.token(), intents)
        .framework(fw)
        .await?;

    if let Err(err) = client.start().await {
        println!("error: {:?}", err)
    }

    Ok(())
}
