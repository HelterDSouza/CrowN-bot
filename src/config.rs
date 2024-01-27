use std::env;

use dotenv::dotenv;
use tokio::sync::OnceCell;

#[derive(Debug)]
struct DatabaseConfig {
    url: String,
}

#[derive(Debug)]
struct BotConfig {
    token: String,
    default_prefix: String,
}
#[derive(Debug)]
pub struct Config {
    bot: BotConfig,
    db: DatabaseConfig,
}
impl Config {
    pub fn db_url(&self) -> &str {
        &self.db.url
    }
    pub fn token(&self) -> &str {
        &self.bot.token
    }
    pub fn default_prefix(&self) -> &str {
        &self.bot.default_prefix
    }
}

pub static CONFIG: OnceCell<Config> = OnceCell::const_new();

async fn init_config() -> Config {
    dotenv().ok();

    println!("{:#?}", env::vars());
    let bot_config = BotConfig {
        token: env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set"),
        default_prefix: env::var("DISCORD_PREFIX").expect("DISCORD_PREXI must be set"),
    };

    let database_config = DatabaseConfig {
        url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
    };
    Config {
        bot: bot_config,
        db: database_config,
    }
}

pub async fn config() -> &'static Config {
    CONFIG.get_or_init(init_config).await
}
