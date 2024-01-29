mod commands;
mod config;
mod constants;
mod data;
mod db;
mod events;
mod framework;
mod paginate;
mod utils;

use crate::config::config;
use crate::db::{initialize_database, repositories::guild_repo::GuildRepository};

use data::Data;
use poise::serenity_prelude::{self as serenity};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

async fn initialize_intents() -> serenity::GatewayIntents {
    serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT
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
async fn main() {
    let config = config().await;

    init_tracing();

    let pool = initialize_database(config).await.unwrap();

    let guild_repo = GuildRepository::new(pool.clone());

    let prefixes = guild_repo.fetch_prefixes().await.unwrap();
    let rolls_channels = guild_repo.fetch_rolls_channels().await.unwrap();

    let data = Data {
        pool,
        default_prefix: config.default_prefix(),
        prefix_map: prefixes,
        roll_channel_map: rolls_channels,
    };
    let framework = framework::initialize_framework(data).await;

    let intents = initialize_intents().await;

    let client = serenity::ClientBuilder::new(config.token(), intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
