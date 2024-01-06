use serenity::all::User;

pub fn on_ready_log_bot_connected(user: &User) {
    tracing::info!("User '{name}' has connected", name = user.name);
}
