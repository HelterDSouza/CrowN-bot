use dashmap::DashMap;
use serenity::{
    all::{ChannelId, GuildId},
    prelude::TypeMapKey,
};
use sqlx::SqlitePool;
use std::{collections::HashMap, sync::Arc};

pub struct DatabasePool;

impl TypeMapKey for DatabasePool {
    type Value = SqlitePool;
}

pub struct PrefixMap;

impl TypeMapKey for PrefixMap {
    type Value = Arc<DashMap<GuildId, String>>;
}

// Config Global
pub struct PubConfig;

impl TypeMapKey for PubConfig {
    type Value = Arc<HashMap<String, String>>;
}

pub struct RollChannelMap;
impl TypeMapKey for RollChannelMap {
    type Value = Arc<DashMap<GuildId, ChannelId>>;
}
