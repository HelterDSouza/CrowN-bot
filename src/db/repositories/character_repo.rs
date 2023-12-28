use anyhow::Result;
use sqlx::{Pool, Sqlite};

use crate::db::models::character::Character;

use super::BaseRepository;

pub struct CharacterRepository {
    pub pool: Pool<Sqlite>,
}

impl BaseRepository for CharacterRepository {
    fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

impl CharacterRepository {
    pub async fn create_resource(
        &self,
        name: &str,
        serie_id: u32,
        image_url: &str,
    ) -> Result<Character, sqlx::Error> {
        let row = sqlx::query_as!(
            Character,
            r#"INSERT INTO characters(name, series_id, image) VALUES ($1,$2,$3) ON CONFLICT (name) DO NOTHING RETURNING id as "id!:u32", name, series_id as "series_id!:u32",image"#,
            name,serie_id,image_url
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(row)
    }
    pub async fn fetch_resource(&self, name: &str) -> Result<Option<Character>, sqlx::Error> {
        let row = sqlx::query_as!(
            Character,
            r#"SELECT id as "id!:u32", name, series_id as "series_id!:u32",image FROM characters WHERE name = $1"#,
            name
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row)
    }
    pub async fn update_resource(&self, name: &str, id: u32) -> Result<(), sqlx::Error> {
        let _ = sqlx::query!(r#"UPDATE characters SET name = $1 WHERE id = $2"#, name, id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn fetch_id_or_create(
        &self,
        name: &str,
        serie_id: u32,
        image_url: &str,
    ) -> Character {
        match self.fetch_resource(name).await {
            Ok(created) => match created {
                Some(character) => character,
                None => match self.create_resource(name, serie_id, image_url).await {
                    Ok(character) => character,
                    Err(_) => todo!(),
                },
            },
            Err(_) => todo!(),
        }
    }
}
