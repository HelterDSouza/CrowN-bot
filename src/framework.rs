use crate::{
    commands::{
        admin::prefix::set_prefix,
        mudae::{
            image_create::add_custom_image, image_delete::remove_custom_image,
            image_list::list_custom_images, roll_channel::set_roll_channel,
        },
    },
    data::{Context, Data, Error},
    events::{
        guild_create::on_guild_create_setup, guild_delete::on_guild_delete_deactivate,
        message::on_message_roll, ready::on_ready_log_bot_connected,
    },
    utils::log_response,
};
use poise::serenity_prelude::FullEvent;
use tracing::Level;

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => {
            log_response(Level::ERROR, &format!("Failed to start bot: {:?}", error));
            panic!();
        }
        poise::FrameworkError::Command { error, ctx, .. } => {
            log_response(
                Level::ERROR,
                &format!("Error in command `{}`: {:?}", ctx.command().name, error),
            );
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                log_response(Level::ERROR, &format!("Error while handling error: {}", e));
            }
        }
    }
}

#[poise::command(prefix_command, hide_in_help)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;

    Ok(())
}
fn initialize_framework_options() -> poise::FrameworkOptions<Data, Error> {
    poise::FrameworkOptions {
        commands: vec![
            register(),
            set_roll_channel(),
            set_prefix(),
            list_custom_images(),
            add_custom_image(),
            remove_custom_image(),
        ],

        prefix_options: poise::PrefixFrameworkOptions {
            mention_as_prefix: true,
            dynamic_prefix: Some(|ctx| Box::pin(dynamic_prefix(ctx))),
            ..Default::default()
        },
        event_handler: |ctx, event, framework, data| {
            Box::pin(event_handler(ctx, event, framework, data))
        },
        on_error: |error| Box::pin(on_error(error)),
        ..Default::default()
    }
}
async fn dynamic_prefix(
    ctx: poise::PartialContext<'_, Data, Error>,
) -> Result<Option<String>, Error> {
    let (prefix, default_prefix) = {
        let prefixes = &ctx.data.prefix_map;
        let default_prefix = ctx.data.default_prefix.to_string();

        (prefixes, default_prefix)
    };

    match ctx.guild_id {
        Some(guild) => match prefix.get(&guild) {
            Some(prefix_guard) => Ok(Some(prefix_guard.value().to_owned())),
            None => Ok(Some(default_prefix)),
        },
        None => Ok(Some(default_prefix)),
    }
    // match ctx.guild_id {
    //     Some(guild_id) => match prefix.get(&guild_id) {
    //         Some(prefix_guard) => Ok(Some(prefix_guard.value().to_owned())),
    //         None => Ok(Some(default_prefix)),
    //     },
    //     None => {
    //         // No guild, use default prefix.
    //         Ok(Some(default_prefix))
    //     }
    // }
}
pub async fn initialize_framework(data: Data) -> poise::Framework<Data, Error> {
    poise::Framework::new(
        initialize_framework_options(),
        move |ctx, ready, framework| {
            Box::pin(async move {
                println!("----------------------------------------------------------------");
                println!("-------------------------- START -------------------------------");
                println!("----------------------------------------------------------------");
                on_ready_log_bot_connected(&ready.user.name);
                for guild in &ready.guilds {
                    let partial = guild.id.to_partial_guild(&ctx.http).await?;
                    log_response(
                        Level::INFO,
                        &format!("🏛 {} is on guild {}", &ready.user.name, &partial.name),
                    );
                }
                println!("----------------------------------------------------------------");
                println!("----------------------- Command List ---------------------------");
                println!("----------------------------------------------------------------");
                for comm in &framework.options().commands {
                    log_response(
                        Level::INFO,
                        &format!(
                            "name :{},\tident:{},\taliases:{:?}",
                            comm.name, comm.identifying_name, comm.aliases
                        ),
                    );
                }
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(data)
            })
        },
    )
}

async fn event_handler(
    ctx: &poise::serenity_prelude::Context,
    event: &FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        FullEvent::Message { new_message } => {
            if new_message.author.id == 1175543083799154708 {
                return Ok(());
            }
            if new_message.guild_id.is_none() {
                return Ok(());
            }
            on_message_roll(ctx, new_message, data).await;
        }
        FullEvent::GuildDelete { incomplete, full } => {
            on_guild_delete_deactivate(ctx, incomplete, full, data).await;
        }
        FullEvent::GuildCreate { guild, is_new } => {
            on_guild_create_setup(ctx, guild, is_new, data).await;
        }
        _ => {}
    }
    Ok(())
}
