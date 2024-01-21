use serenity::{all::Message, client::Context};

use crate::{data::RollChannelMap, utils::handle_rolls_message};

pub async fn on_message_roll(ctx: &Context, msg: &Message) {
    let data = ctx.data.read().await;
    let guild_id = msg.guild_id.expect("Should get guild_id");

    let channel_id = match data.get::<RollChannelMap>() {
        Some(roll_channel_map) => match roll_channel_map.get(&guild_id) {
            Some(value) => *value.value(),
            None => return,
        },
        None => return,
    };
    if msg.channel_id == channel_id {
        // tracing::debug!("Received roll message: {:?}", msg);
        let _ = handle_rolls_message(msg, ctx).await;
    }
}

