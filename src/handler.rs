use serenity::all::{Guild, UnavailableGuild};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use crate::events::guild_create::on_guild_create_setup;
use crate::events::guild_delete::on_guild_delete_deactivate;
use crate::events::message::on_message_roll;
use crate::events::ready::on_ready_log_bot_connected;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        on_ready_log_bot_connected(&ready.user);
    }
    async fn guild_delete(&self, ctx: Context, incomplete: UnavailableGuild, full: Option<Guild>) {
        on_guild_delete_deactivate(&ctx, &incomplete, &full).await;
    }
    async fn guild_create(&self, ctx: Context, guild: Guild, _is_new: Option<bool>) {
        on_guild_create_setup(&ctx, &guild, &_is_new).await;
    }
    async fn message(&self, ctx: Context, msg: Message) {
        //Ignorar mensagens do próprio bot
        if msg.author.id == 1175543083799154708 {
            return;
        }
        //Lida com DMs
        if msg.guild_id.is_none() {
            return;
        }

        //Lidar com mensagens relacionadas a rolagens
        on_message_roll(&ctx, &msg).await;
    }
}
