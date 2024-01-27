use dashmap::DashMap;
use serenity::all::{ChannelId, GuildId};
use sqlx::SqlitePool;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Debug)]
pub struct Data {
    pub pool: SqlitePool,
    pub prefix_map: DashMap<GuildId, String>,
    pub default_prefix: &'static str,
    pub roll_channel_map: DashMap<GuildId, ChannelId>,
}
