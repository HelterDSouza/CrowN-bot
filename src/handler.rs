use serenity::all::{Embed, Guild, User};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use crate::data::DatabasePool;
use crate::db::models::guild_configuration::GuildConfiguration;
use crate::db::repos::guild_repo::GuildRepository;

fn log_bot_connected(user: &User) {
    tracing::info!("User '{name}' has connected", name = user.name);
}
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        log_bot_connected(&ready.user)
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, _is_new: Option<bool>) {
        // Obter o pool de conexões do contexto
        let pool = ctx
            .data
            .read()
            .await
            .get::<DatabasePool>()
            .cloned()
            .expect("Failed to get database pool");

        // Criar instância do repositório
        let guild_repo = GuildRepository { pool };

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

    async fn message(&self, _ctx: Context, msg: Message) {
        // Ignorar mensagens do próprio bot
        if msg.author.id == 1175543083799154708 {
            return;
        }
    }
}
