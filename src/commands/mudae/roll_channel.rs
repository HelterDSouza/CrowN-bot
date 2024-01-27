use std::time::Duration;

use poise::serenity_prelude::{self as serenity};

use crate::data::{Context, Error};
use crate::db::repositories::guild_repo::GuildRepository;

async fn get_answer(ctx: &Context<'_>, msg: &serenity::Message) -> Option<serenity::ChannelId> {
    match msg.content.to_lowercase().as_str() {
        "y" | "yes" => Some(msg.channel_id),
        "n" | "no" => {
            let replay = "The procedure has been stopped.";
            if let Err(err) = ctx.reply(replay).await {
                tracing::error!("Error replying to message: {:?}", err);
            }
            None
        }
        _ => None,
    }
}

#[poise::command(
    prefix_command,
    slash_command,
    category = "mudae",
    guild_only,
    owners_only,
    rename = "setrollchannel",
    aliases("rollchannel", "rc")
)]
pub async fn set_roll_channel(
    ctx: Context<'_>,
    channel: Option<serenity::GuildChannel>,
) -> Result<(), Error> {
    let roll_map = ctx.data().roll_channel_map.clone();
    let pool = ctx.data().pool.clone();
    // repositorios
    let guild_repo = GuildRepository::new(pool);

    let channel_id = match channel.as_ref() {
        Some(channel) => channel.id,
        None => {
            let ask_channel = "No channel specified, do you want to use this channel? (y/n/yes/no)";
            if let Err(err) = ctx.reply(ask_channel).await {
                eprintln!("Error sending message: {:?}", err);
            }

            let collector = ctx
                .author()
                .await_reply(ctx)
                .timeout(Duration::from_secs(20));

            if let Some(answer) = collector.await {
                if let Some(answer) = get_answer(&ctx, &answer).await {
                    answer
                } else {
                    return Ok(());
                }
            } else {
                if let Err(err) = ctx.reply("Time's up.").await {
                    eprintln!("Error sending message: {:?}", err);
                }
                return Ok(());
            }
        }
    };

    let guild_id = ctx.guild_id().unwrap();
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
            if let Err(err) = ctx.reply("saved").await {
                tracing::error!("Error replying to message: {:?}", err);
            }
        }
        Err(err) => {
            tracing::error!("Error replying to message: {:?}", err);
            if let Err(err) = ctx.reply("error saving").await {
                tracing::error!("Error replying to message: {:?}", err);
            }
        }
    };
    Ok(())
}

