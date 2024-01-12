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
    pub async fn create_resource(&self, image: &str, character_id: u32) -> Result<(), sqlx::Error> {
        let _ = sqlx::query!(
            r#"INSERT INTO custom_images(image_url,character_id) VALUES($1,$2)"#,
            image,
            character_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

