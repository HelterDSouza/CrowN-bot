use std::time::Duration;

use serenity::utils::parse_channel_mention;
use serenity::{
    all::{ChannelId, Message},
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
};

use crate::data::{DatabasePool, RollChannelMap};
use crate::db::repositories::guild_repo::GuildRepository;

#[command]
#[aliases("rollchannel")]
#[owner_privilege]
#[delimiters(" ")]
pub async fn set_roll_channel(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // let channel = parse_channel_mention(&mention).unwrap();
    let data = ctx.data.read().await;

    // Typemaps
    let roll_map = data.get::<RollChannelMap>().cloned().unwrap();
    let pool = data.get::<DatabasePool>().cloned().unwrap();
    // repositorios
    let guild_repo = GuildRepository::new(pool);

    // Pegar a guilda id
    let guild_id = msg.guild_id.unwrap();

    let channel_id = match args.single::<String>().ok() {
        Some(mention) => {
            if let Some(channel) = parse_channel_mention(&mention) {
                channel
            } else {
                let replay = "Please, provide a valid channel";
                let _ = msg.reply(&ctx.http, replay).await;
                return Ok(());
            }
        }
        None => {
            let ask_channel = "No channel specified, do you want to use this channel? (y/n/yes/no)";
            if let Err(err) = msg.reply(ctx, ask_channel).await {
                eprintln!("Error sending message: {:?}", err);
            }

            let collector = msg
                .author
                .await_reply(&ctx.shard)
                .timeout(Duration::from_secs(20));

            if let Some(answer) = collector.await {
                if let Some(answer) = get_answer(ctx, &answer).await {
                    answer
                } else {
                    return Ok(());
                }
            } else {
                if let Err(err) = msg.reply(ctx, "Time's up.").await {
                    eprintln!("Error sending message: {:?}", err);
                }
                return Ok(());
            }
        }
    };
    match guild_repo
        .set_roll_channel(&channel_id.to_string(), &guild_id.to_string())
        .await
    {
        Ok(_) => {
            if let Some(mut roll) = roll_map.get_mut(&guild_id) {
                *roll = channel_id;
            } else {
                roll_map.insert(guild_id, channel_id);
            };
            if let Err(err) = msg.reply(&ctx.http, "saved").await {
                tracing::error!("Error replying to message: {:?}", err);
            }
        }
        Err(err) => {
            tracing::error!("Error replying to message: {:?}", err);
            if let Err(err) = msg.reply(&ctx.http, "error saving").await {
                tracing::error!("Error replying to message: {:?}", err);
            }
        }
    };
    Ok(())
}

async fn get_answer(ctx: &Context, msg: &Message) -> Option<ChannelId> {
    match msg.content.to_lowercase().as_str() {
        "y" | "yes" => Some(msg.channel_id),
        "n" | "no" => {
            let replay = "The procedure has been stopped.";
            if let Err(err) = msg.reply(&ctx.http, replay).await {
                tracing::error!("Error replying to message: {:?}", err);
            }
            None
        }
        _ => None,
    }
}

