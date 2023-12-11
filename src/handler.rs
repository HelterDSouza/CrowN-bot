use serenity::all::{Embed, User};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
fn log_bot_connected(user: &User) {
    tracing::info!("User '{name}' has connected", name = user.name);
}
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        log_bot_connected(&ready.user)
    }

    async fn message(&self, _ctx: Context, msg: Message) {
        // Ignorar mensagens do próprio bot
        if msg.author.id == 1175543083799154708 {
            return;
        }
    }
}
