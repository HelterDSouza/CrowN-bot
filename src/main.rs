mod commands;
mod config;
mod data;
mod db;
mod framework;
mod handler;

use crate::config::config;
use crate::data::{DatabasePool, PrefixMap, PubConfig};
use crate::db::{initialize_database, repositories::guild_repo::GuildRepository};

use anyhow::Result;
use data::RollChannelMap;
use serenity::{all::GatewayIntents, http::Http, Client};

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

async fn initialize_intents() -> GatewayIntents {
    GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT
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

    let mut pub_config = HashMap::new();

    pub_config.insert(
        "default_prefix".to_string(),
        config.default_prefix().to_string(),
    );

    init_tracing();

    let http = Http::new(config.token());

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();

            if let Some(owner) = &info.owner {
                owners.insert(owner.id);
            }

            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let intents = initialize_intents().await;

    let pool = initialize_database(config).await?;

    let guild_repo = GuildRepository::new(pool.clone());
    let prefixes = guild_repo.fetch_prefixes().await.unwrap();

    let rolls_channels = guild_repo.fetch_rolls_channels().await.unwrap();

    let framework = framework::initialize_framework(owners, bot_id).await;

    let mut client = Client::builder(config.token(), intents)
        .event_handler(handler::Handler)
        .framework(framework)
        .await?;

    {
        let mut data = client.data.write().await;

        data.insert::<PrefixMap>(Arc::new(prefixes));
        data.insert::<RollChannelMap>(Arc::new(rolls_channels));
        data.insert::<DatabasePool>(pool);
        data.insert::<PubConfig>(Arc::new(pub_config));
    }

    if let Err(err) = client.start().await {
        println!("error: {:?}", err)
    }

    Ok(())
}
