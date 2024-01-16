#[derive(Debug, sqlx::FromRow)]
pub struct GuildConfiguration {
    pub id: Option<u32>,
    pub guild_id: String,
    pub roll_channel: Option<String>,
    pub name: String,
    pub prefix: String,
    pub is_active: bool,
}

impl Default for GuildConfiguration {
    fn default() -> Self {
        Self {
            id: None,
            name: String::default(),
            guild_id: String::default(),
            roll_channel: Some(String::default()),
            prefix: "$$".to_string(),
            is_active: true,
        }
    }
}

impl GuildConfiguration {
    pub fn new(name: &str, guild_id: &str) -> Self {
        Self {
            name: name.to_string(),
            guild_id: guild_id.to_string(),
            ..Default::default()
        }
    }
}

