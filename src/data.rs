use serenity::prelude::TypeMapKey;
use sqlx::SqlitePool;

pub struct DatabasePool;

impl TypeMapKey for DatabasePool {
    type Value = SqlitePool;
}
