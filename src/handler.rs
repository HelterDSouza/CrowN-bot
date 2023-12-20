use serenity::all::{Embed, Guild, User};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use crate::data::{DatabasePool, RollChannelMap};
use crate::db::models::guild_configuration::GuildConfiguration;
use crate::db::models::roll::EmbedRoll;
use crate::db::repos::guild_repo::GuildRepository;

fn log_bot_connected(user: &User) {
    tracing::info!("User '{name}' has connected", name = user.name);
}

async fn process_character_embed(embed: &Embed) -> Result<EmbedRoll, &'static str> {
    //TODO: Verificar se é um personagem ou qualquer roll
    //*INFO: Para saber se é um personagem, basta pegar a description em linhas e a ultima linha tem o emoji de kakera

    let name = match &embed.author {
        Some(author) => author.name.clone(),
        None => String::default(),
    };

    let url = match &embed.image {
        Some(image) => image.url.clone(),
        None => String::default(),
    };

    let serie = match &embed.description {
        Some(description) => description
            .lines()
            .take(description.lines().count() - 1)
            .collect::<Vec<_>>()
            .join(" "),
        None => String::default(),
    };

    if name.is_empty() || url.is_empty() || serie.is_empty() {
        Err("One or more required fields are empty")
    } else {
        Ok(EmbedRoll::new(name, url, serie))
    }
}

async fn handle_rolls_message(msg: &Message) {
    //* MudaeBot
    if msg.author.id == 432610292342587392 {
        if let Some(embed) = msg.embeds.get(0) {
            if let Ok(roll) = process_character_embed(embed).await {
                println!(
                    "Name: {}, URL: {}, Serie: {}",
                    roll.name, roll.serie, roll.url
                );
            }
            // match process_character_embed(embed).await {
            //     Ok(roll) => {}
            //     Err(err) => println!("No Image"),
            // }
        }
    }
}
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        log_bot_connected(&ready.user);
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, _is_new: Option<bool>) {
        // TODO: Adiciona ao contexto quando criar uma nova config
        // Obter o pool de conexões do contexto
        let pool = ctx
            .data
            .read()
            .await
            .get::<DatabasePool>()
            .cloned()
            .expect("Failed to get database pool");

        // Criar instância do repositório
        let guild_repo = GuildRepository::new(pool);

        // Extrair informações da guild
        let guild_name = &guild.name;
        let guild_id = &guild.id.to_string();

        // Criar instância da configuração da guild
        let guild_config = GuildConfiguration::new(guild_name, guild_id);

        // Verificar se a configuração já existe
        match guild_repo.find_one_guild(&guild_config.guild_id).await {
            Ok(Some(config)) => {
                tracing::debug!("Found guild {} configuration", config.guild_id)
            }
            Ok(None) => {
                tracing::debug!(
                    "Configuration not found for guild {}",
                    &guild_config.guild_id
                );
                tracing::debug!("Creating configuration");

                // Tentar criar a configuração
                match guild_repo.create(guild_config).await {
                    Ok(new_config) => {
                        tracing::debug!("{:?}", new_config);
                        tracing::info!("Guild {guild_name} recognized and loaded.");
                    }
                    Err(err) => {
                        tracing::error!("Error creating configuration: {:?}", err);
                    }
                }
            }
            Err(err) => {
                tracing::error!("Error getting guild configuration: {:?}", err);
            }
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        let data = ctx.data.read().await;
        //* Ignorar mensagens do próprio bot
        if msg.author.id == 1175543083799154708 {
            return;
        }
        //* Lida com DMs
        if msg.guild_id.is_none() {
            return;
        }

        //* Lidar com mensagens relacionadas a rolagens

        let guild_id = msg.guild_id.unwrap();
        let channel_id = match data.get::<RollChannelMap>() {
            Some(roll_channel_map) => match roll_channel_map.get(&guild_id) {
                Some(value) => *value.value(),
                None => return,
            },
            None => return,
        };

        if msg.channel_id == channel_id {
            let _ = handle_rolls_message(&msg).await;
        }
    }
}
