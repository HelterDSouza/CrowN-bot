use serenity::{
    all::{Guild, UnavailableGuild},
    client::Context,
};

use crate::{data::Data, db::repositories::guild_repo::GuildRepository};

pub async fn on_guild_delete_deactivate(
    _ctx: &Context,
    incomplete: &UnavailableGuild,
    full: &Option<Guild>,
    data: &Data,
) {
    // Get the database pool from the context
    let pool = data.pool.clone();

    // Create a new instance of the GuildRepository using the database pool
    let guild_repo = GuildRepository::new(pool);

    // Determine the guild ID based on whether the full parameter is Some or None
    let guild_id = match full {
        Some(guild) => guild.id.to_string(),
        None => incomplete.id.to_string(),
    };

    // Deactivate the guild in the database
    if let Err(err) = guild_repo.deactivate(&guild_id).await {
        // Log an error message if deactivation fails
        tracing::error!("unable to deactivate guild {}\n error: {}", guild_id, err);
    } else {
        // Log a message indicating that the guild has been deactivated
        tracing::info!("{:?} deactivated", guild_id);
    }
}

