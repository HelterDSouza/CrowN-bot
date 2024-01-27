use std::time::Duration;

use poise::serenity_prelude::{self as serenity};

use crate::{
    data::{Context, Error},
    db::repositories::guild_repo::GuildRepository,
};
#[poise::command(
    prefix_command,
    slash_command,
    category = "Configuration",
    guild_only,
    guild_cooldown = "5",
    rename = "modify-guild-prefix",
    aliases("prefix")
)]
pub async fn set_prefix(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    let guild_repo = GuildRepository::new(ctx.data().pool.clone());

    let prefix_map = &ctx.data().prefix_map;

    let prefix_str = {
        let prefix = prefix_map.try_get(&guild_id).unwrap();
        &*prefix.clone()
    };

    let embed = serenity::CreateEmbed::default()
        .title("Guild Settings")
        .field("Prefix", prefix_str, true);

    let ctx_id = ctx.id();
    let prefix_id = format!("{}modal", ctx.id());
    let thing = serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new(&prefix_id).label("change prefix")
    ]);

    let builder = poise::CreateReply::default()
        .embed(embed.clone())
        .components(vec![thing]);
    let msg = ctx.send(builder.clone()).await?;
    while let Some(press) = serenity::ComponentInteractionCollector::new(ctx)
        .filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()))
        .timeout(Duration::from_secs(15))
        .await
    {
        if press.data.custom_id == prefix_id {
            let data = poise::execute_modal_on_component_interaction::<NewPrefix>(
                ctx,
                press.clone(),
                None,
                None,
            )
            .await?;

            if let Some(data) = data {
                let prefix_str = format!("`{}`", data.input.clone());
                // should validate it.

                if (guild_repo
                    .update_prefix(&guild_id.to_string(), &data.input.clone())
                    .await)
                    .is_ok()
                {
                    {
                        let mut teste = prefix_map.try_get_mut(&guild_id).unwrap();
                        *teste = data.input.clone();
                    }
                }

                let embed = serenity::CreateEmbed::default()
                    .title("Guild Settings")
                    .field("Prefix", prefix_str, true);
                msg.edit(ctx, poise::CreateReply::default().embed(embed))
                    .await?;
            }
        }
    }

    Ok(())
}

#[derive(Debug, poise::Modal)]
struct NewPrefix {
    input: String,
}
