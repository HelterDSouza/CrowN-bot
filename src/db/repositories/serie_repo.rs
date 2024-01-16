use anyhow::Result;
use sqlx::{Pool, Sqlite};

use crate::db::models::serie::{Serie, SerieIdInResponse};

use super::BaseRepository;

pub struct SerieRepository {
    pub pool: Pool<Sqlite>,
}

impl BaseRepository for SerieRepository {
    fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

impl SerieRepository {
    pub async fn create_resource(&self, name: &str) -> Result<Serie, sqlx::Error> {
        let row = sqlx::query_as!(
            Serie,
            r#"INSERT INTO series(name) VALUES ($1) ON CONFLICT (name) DO NOTHING RETURNING id as "id:u32", name"#,
            name
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(row)
    }
    pub async fn fetch_resource(&self, name: &str) -> Result<Option<Serie>, sqlx::Error> {
        let row = sqlx::query_as!(
            Serie,
            r#"SELECT id as "id!:_", name FROM series WHERE name = $1"#,
            name
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row)
    }
    pub async fn update_resource(&self, name: &str, id: u32) -> Result<(), sqlx::Error> {
        let _ = sqlx::query!(r#"UPDATE series SET name = $1 WHERE id = $2"#, name, id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn fetch_id_or_create(&self, name: &str) -> SerieIdInResponse {
        let id = match self.fetch_resource(name).await {
            Ok(Some(serie)) => serie.id,
            Ok(None) => match self.create_resource(name).await {
                Ok(serie) => serie.id,
                Err(_) => todo!(),
            },
            Err(_) => todo!(),
        };
        SerieIdInResponse { id }
    }
}

