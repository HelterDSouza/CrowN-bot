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
        guild_id: i64,
    ) -> Result<Option<GuildConfiguration>, sqlx::Error> {
        let row = sqlx::query_as!(
            GuildConfiguration,
            r#"select roll_channel, guild_id,name, prefix, is_active from GuildConfigurations as gc where gc.guild_id = ?"#,
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
            r#"INSERT INTO GuildConfigurations(name, guild_id, prefix, is_active) 
                Values(?, ?, ?, ?)
                ON CONFLICT DO NOTHING
                RETURNING 
                    roll_channel,
                    name, 
                    guild_id, 
                    prefix, 
                    is_active"#,
            guild.name,
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

    pub async fn deactivate(
        &self,
        guild_id: &str,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        let row = sqlx::query!(
            "UPDATE GuildConfigurations SET is_active = false WHERE guild_id = $1",
            guild_id
        )
        .execute(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn activate(
        &self,
        guild_id: i64,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        let row = sqlx::query!(
            "UPDATE GuildConfigurations SET is_active = true WHERE guild_id = $1",
            guild_id
        )
        .execute(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn fetch_prefixes(&self) -> CommandResult<DashMap<GuildId, String>> {
        let prefixes = DashMap::new();

        let cursor = sqlx::query!("SELECT guild_id, prefix FROM GuildConfigurations")
            .fetch_all(&self.pool)
            .await?;

        for guild in cursor {
            prefixes.insert(GuildId::from(guild.guild_id as u64), guild.prefix);
        }
        Ok(prefixes)
    }

    pub async fn update_prefix(&self, guild: i64, prefix: &str) -> Result<(), sqlx::Error> {
        let _ = sqlx::query_as!(
            GuildConfiguration,
            r#"UPDATE GuildConfigurations SET prefix = $1 WHERE guild_id = $2"#,
            prefix,
            guild
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn fetch_rolls_channels(&self) -> CommandResult<DashMap<GuildId, ChannelId>> {
        let rolls_channels = DashMap::new();

        let cursor = sqlx::query!(
            "SELECT guild_id, roll_channel  FROM GuildConfigurations WHERE is_active = 1"
        )
        .fetch_all(&self.pool)
        .await?;

        for guild in cursor {
            if let Some(channel_id) = guild.roll_channel {
                rolls_channels.insert(
                    GuildId::from(guild.guild_id as u64),
                    ChannelId::from(channel_id as u64),
                );
            }
        }
        Ok(rolls_channels)
    }

    pub async fn set_roll_channel(
        &self,
        channel_id: &str,
        guild_id: &str,
    ) -> Result<(), sqlx::Error> {
        let _ = sqlx::query!(
            "UPDATE GuildConfigurations SET roll_channel = $1 where guild_id = $2",
            channel_id,
            guild_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
