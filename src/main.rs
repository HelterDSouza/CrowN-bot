mod config;
mod data;
mod db;
mod handler;

use anyhow::Result;
use config::Config;
use data::DatabasePool;
use serenity::{
    all::{GatewayIntents, UserId},
    framework::{standard::Configuration, StandardFramework},
    http::Http,
    Client,
};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    Pool, Sqlite,
};
use std::{collections::HashSet, str::FromStr};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::config;

async fn initialize_framework(owners: HashSet<UserId>, id: UserId) -> StandardFramework {
    let framework = StandardFramework::new();
    framework.configure(
        Configuration::new()
            .owners(owners)
            .ignore_webhooks(false)
            .no_dm_prefix(true)
            .on_mention(Some(id))
            .prefix("$")
            .delimiter("$"),
    ); // !Fix: Remover o hard code
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

    let http = Http::new(config.token());

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(owner) = &info.owner {
                owners.insert(owner.id);
            }

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };
    let id = http.get_current_user().await?.id;
    let fw = initialize_framework(owners, id).await;

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
