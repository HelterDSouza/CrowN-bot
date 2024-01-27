use serenity::{all::Message, client::Context};

use crate::{data::Data, utils::handle_rolls_message};

pub async fn on_message_roll(ctx: &Context, msg: &Message, data: &Data) {
    let guild_id = msg.guild_id.expect("Should get guild_id");

    let channel_id = match data.roll_channel_map.get(&guild_id) {
        Some(value) => *value.value(),
        None => return,
    };

    if msg.channel_id == channel_id {
        // tracing::debug!("Received roll message: {:?}", msg);
        let _ = handle_rolls_message(msg, ctx, data).await;
    }
}

