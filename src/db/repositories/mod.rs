use sqlx::{Pool, Sqlite};

pub mod character_repo;
pub mod guild_repo;
pub mod image_repo;
pub mod serie_repo;

pub trait BaseRepository {
    fn new(pool: Pool<Sqlite>) -> Self;
}

