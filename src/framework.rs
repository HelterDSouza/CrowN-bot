use crate::commands::admin::*;
use crate::data::{PrefixMap, PubConfig};
use std::collections::HashSet;

use serenity::{
    all::{Message, UserId},
    client::Context,
    framework::{
        standard::{macros::hook, Configuration},
        StandardFramework,
    },
};

pub async fn initialize_framework(owners: HashSet<UserId>, id: UserId) -> StandardFramework {
    let framework = StandardFramework::new().group(&ADMIN_GROUP);

    framework.configure(
        Configuration::new()
            .dynamic_prefix(dynamic_prefix)
            .owners(owners)
            .ignore_webhooks(false)
            .no_dm_prefix(true)
            .on_mention(Some(id))
            .prefix("")
            .delimiter("$"),
    );
    framework
}

#[hook]
async fn dynamic_prefix(ctx: &Context, msg: &Message) -> Option<String> {
    let data = ctx.data.read().await;

    let (prefix, default_prefix) = {
        let prefixes = data.get::<PrefixMap>().cloned()?;

        let default_prefix = data.get::<PubConfig>()?.get("default_prefix").cloned()?;

        (prefixes, default_prefix)
    };

    let guild_id = match msg.guild_id {
        Some(value) => value,
        None => return None,
    };

    let wrapped_prefix = prefix.get(&guild_id);

    match wrapped_prefix {
        Some(prefix_guard) => Some(prefix_guard.value().to_owned()),
        None => Some(default_prefix),
    }
}
