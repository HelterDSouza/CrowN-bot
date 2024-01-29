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
        owner: i64,
    ) -> Result<(), sqlx::Error> {
        let query = sqlx::query!(
            r#"INSERT INTO CustomImages (image_url,character_id,is_nsfw) VALUES($1,$2,$3)"#,
            image,
            character_id,
            is_nsfw,
        );

        query.execute(&self.pool).await?;
        Ok(())
    }
    pub async fn fetch_collection_by_character(
        &self,
        character_id: u32,
    ) -> Result<Vec<CustomImage>, sqlx::Error> {
        let row = sqlx::query_as!(
            CustomImage,
            "SELECT image_url FROM CustomImages WHERE character_id = $1",
            character_id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(row)
    }
    pub async fn fetch_collection(
        &self,
        _account_id: u64,
        _is_nsfw: bool,
    ) -> Result<Vec<CustomImage>, sqlx::Error> {
        let row = sqlx::query_as!(CustomImage, "SELECT image_url FROM CustomImages",)
            .fetch_all(&self.pool)
            .await?;
        Ok(row)
    }
    pub async fn remove_resource(&self, image_url: &str) -> Result<(), sqlx::Error> {
        let row = sqlx::query!("DELETE FROM CustomImages where image_url = $1", image_url)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct CustomImage {
    pub image_url: String,
}
