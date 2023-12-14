use sqlx::{Pool, Sqlite};

use crate::db::models::guild_configuration::GuildConfiguration;
pub struct GuildRepository {
    pub pool: Pool<Sqlite>,
}

impl GuildRepository {
    pub async fn find_one_guild(
        &self,
        guild_id: &str,
    ) -> Result<Option<GuildConfiguration>, sqlx::Error> {
        let row = sqlx::query_as!(
            GuildConfiguration,
            r#"select id as "id:_", guild_id,guild_name, prefix, is_active from guild_configurations as gc where gc.guild_id = ?"#,
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
}