use dashmap::DashMap;
use serenity::{
    all::{ChannelId, GuildId},
    framework::standard::CommandResult,
};
use sqlx::{Pool, Sqlite};

use crate::db::models::guild_configuration::GuildConfiguration;
pub struct GuildRepository {
    pub pool: Pool<Sqlite>,
}

impl GuildRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
    pub async fn find_one_guild(
        &self,
        guild_id: &str,
    ) -> Result<Option<GuildConfiguration>, sqlx::Error> {
        let row = sqlx::query_as!(
            GuildConfiguration,
            r#"select id as "id:_",roll_channel_id, guild_id,guild_name, prefix, is_active from guild_configurations as gc where gc.guild_id = ?"#,
            guild_id
        )
        .fetch_optional(&self.pool)
        .await;

        match row {
            Ok(result) => Ok(result),
            Err(err) => Err(err),
        }
    }

    pub async fn create(
        &self,
        guild: GuildConfiguration,
    ) -> Result<GuildConfiguration, sqlx::Error> {
        let row = sqlx::query_as!(
            GuildConfiguration,
            r#"INSERT INTO guild_configurations(guild_name, guild_id, prefix, is_active) 
                Values(?, ?, ?, ?)
                ON CONFLICT DO NOTHING
                RETURNING 
                    id as "id!:u32", 
                    roll_channel_id,
                    guild_name, 
                    guild_id, 
                    prefix, 
                    is_active"#,
            guild.guild_name,
            guild.guild_id,
            guild.prefix,
            guild.is_active
        )
        .fetch_one(&self.pool)
        .await;

        match row {
            Ok(result) => Ok(result),
            Err(err) => Err(err),
        }
    }

    pub async fn fetch_prefixes(&self) -> CommandResult<DashMap<GuildId, String>> {
        let prefixes = DashMap::new();

        let cursor = sqlx::query!("SELECT guild_id, prefix FROM guild_configurations")
            .fetch_all(&self.pool)
            .await?;

        for guild in cursor {
            prefixes.insert(
                GuildId::from(guild.guild_id.trim().parse::<u64>()?),
                guild.prefix,
            );
        }
        Ok(prefixes)
    }

    pub async fn update_prefix(
        &self,
        guild: &str,
        prefix: &str,
    ) -> Result<GuildConfiguration, sqlx::Error> {
        let row = sqlx::query_as!(
            GuildConfiguration,
            r#"UPDATE guild_configurations SET prefix = $1 WHERE guild_id =$2 RETURNING
                    id as "id!:u32", 
                    roll_channel_id,
                    guild_name, 
                    guild_id, 
                    prefix, 
                    is_active"#,
            prefix,
            guild
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(row)
    }
    pub async fn fetch_rolls_channels(&self) -> CommandResult<DashMap<GuildId, ChannelId>> {
        let rolls_channels = DashMap::new();

        let cursor = sqlx::query!("SELECT guild_id, roll_channel_id  FROM guild_configurations")
            .fetch_all(&self.pool)
            .await?;

        for guild in cursor {
            if let Some(channel_id) = guild.roll_channel_id {
                rolls_channels.insert(
                    GuildId::from(guild.guild_id.parse::<u64>()?),
                    ChannelId::from(channel_id.parse::<u64>()?),
                );
            }
        }
        Ok(rolls_channels)
    }
}
