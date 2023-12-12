#[derive(Debug, sqlx::FromRow)]
pub struct GuildConfiguration {
    pub id: i32,
    pub guild_id: String,
    pub prefix: String,
    pub is_active: bool,
}
