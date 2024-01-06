use serenity::{
    all::{Guild, GuildId},
    client::Context,
};

use crate::{
    data::{DatabasePool, PrefixMap},
    db::{
        models::guild_configuration::GuildConfiguration, repositories::guild_repo::GuildRepository,
    },
};

pub async fn on_guild_create_setup(ctx: &Context, guild: &Guild, _is_new: &Option<bool>) {
    let data = ctx.data.read().await;
    let prefix_map = data.get::<PrefixMap>().unwrap();

    // Obter o pool de conexões do contexto
    let pool = data
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
            tracing::debug!("Found guild {} configuration", config.guild_id);
            // update is_active
            if !config.is_active {
                match guild_repo.activate(guild_id).await {
                    Ok(_) => tracing::info!("{} activating", guild_id),
                    Err(err) => {
                        tracing::error!("unable to activate guild {}\n error: {}", guild_id, err)
                    }
                }
            }
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
                    prefix_map.insert(
                        GuildId::new(new_config.guild_id.parse().unwrap()),
                        new_config.prefix,
                    );
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
