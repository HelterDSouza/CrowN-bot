#[derive(Debug, sqlx::FromRow)]
pub struct GuildConfiguration {
    pub id: Option<u32>,
    pub guild_id: String,
    pub guild_name: String,
    pub prefix: String,
    pub is_active: bool,
}

impl Default for GuildConfiguration {
    fn default() -> Self {
        Self {
            id: None,
            guild_name: String::default(),
            guild_id: String::default(),
            prefix: "$".to_string(),
            is_active: true,
        }
    }
}

impl GuildConfiguration {
    pub fn new(guild_name: &str, guild_id: &str) -> Self {
        Self {
            guild_name: guild_name.to_string(),
            guild_id: guild_id.to_string(),
            ..Default::default()
        }
    }
}
