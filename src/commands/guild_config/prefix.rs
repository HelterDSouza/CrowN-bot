use serenity::{
    all::Message,
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
};

use crate::{
    data::{DatabasePool, PrefixMap},
    db::repositories::guild_repo::GuildRepository,
};

#[command]
#[aliases("prefix")]
#[owner_privilege]
#[delimiters(" ")]
pub async fn set_prefix(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;

    // Typemaps
    let prefix_map = data.get::<PrefixMap>().cloned().unwrap();
    let pool = data.get::<DatabasePool>().cloned().unwrap();

    // repositorios
    let guild_repo = GuildRepository::new(pool);

    match args.single::<String>() {
        Ok(prefix) => {
            match msg.guild_id {
                Some(x) => {
                    if let Some(mut old_prefix) = prefix_map.get_mut(&x) {
                        // Atualiza o prefix antigo com o nome prefix
                        if let Ok(_guild) = guild_repo.update_prefix(&x.to_string(), &prefix).await
                        {
                            // FIXME: Ta funcionando, mas ta feio.

                            *old_prefix = prefix.clone();
                            msg.channel_id
                                .say(
                                    &ctx.http,
                                    format!("{prefix} is my new prefix for this server."),
                                )
                                .await?;
                            return Ok(());
                        }
                    }
                }
                None => {
                    msg.channel_id
                        .say(
                            &ctx.http,
                            "Oops! Something went wrong. I can't find your server Id.",
                        )
                        .await?;
                    return Ok(());
                }
            };
        }

        Err(_) => {
            // TODO: Colocar o previous prefix dinamicamente
            msg.channel_id.say(&ctx.http, "Oops! It seems you forgot to provide a new prefix. \nPlease use the command like this: `$$prefix <new_prefix>`").await?;
            return Ok(());
        }
    };
    Ok(())
}
