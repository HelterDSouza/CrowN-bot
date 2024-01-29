#[derive(Debug, Default, sqlx::FromRow)]
pub struct GuildConfiguration {
    pub guild_id: i64,
    pub roll_channel: Option<i64>,
    pub name: String,
    pub prefix: String,
    pub is_active: bool,
}

impl GuildConfiguration {
    pub fn new(name: &str, guild_id: i64) -> Self {
        Self {
            guild_id,
            name: name.to_string(),
            is_active: true,
            ..Default::default()
        }
    }
    pub fn set_prefix(mut self, prefix: &str) -> GuildConfiguration {
        self.prefix = prefix.to_string();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let name = "TestGuild";
        let guild_id = 123;

        let config = GuildConfiguration::new(name, guild_id);

        assert_eq!(config.guild_id, guild_id);
        assert_eq!(config.roll_channel, None, "roll_channel should be None");
        assert_eq!(config.name, name.to_string(), "name should match");
        assert_eq!(config.prefix, String::default(), "prefix should be default");
        assert!(config.is_active, "is_active should be true");
    }
}
