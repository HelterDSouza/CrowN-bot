use sqlx::{Pool, Sqlite};

use super::BaseRepository;

pub struct ImageRepository {
    pub pool: Pool<Sqlite>,
}

impl BaseRepository for ImageRepository {
    fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

impl ImageRepository {
    pub async fn create_resource(
        &self,
        image: &str,
        character_id: u32,
        is_nsfw: bool,
    ) -> Result<(), sqlx::Error> {
        let _ = sqlx::query!(
            r#"INSERT INTO CustomImages(image_url,character_id,is_nsfw) VALUES($1,$2,$3)"#,
            image,
            character_id,
            is_nsfw
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
    pub async fn fetch_collection(
        &self,
        _account_id: u64,
        is_nsfw: bool,
    ) -> Result<Vec<CustomImage>, sqlx::Error> {
        let row = sqlx::query_as!(
            CustomImage,
            "SELECT image_url FROM CustomImages WHERE is_nsfw = $1",
            is_nsfw
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(row)
    }
}

#[derive(Debug)]
pub struct CustomImage {
    pub image_url: String,
}

