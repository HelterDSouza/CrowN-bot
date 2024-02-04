use poise::serenity_prelude::all::ReactionType;
use tracing::Level;

use crate::{
    constants::{CHARACTER_NAME_NOT_PROVIDED, CHARACTER_NOT_FOUND, DATABASE_QUERY_ERROR},
    data::{Context, Error},
    db::{
        models::character::Character,
        repositories::{
            character_repo::CharacterRepository,
            image_repo::{CustomImage, ImageRepository},
            BaseRepository,
        },
    },
    utils::log_response,
};

#[poise::command(slash_command, prefix_command, aliases("rci"), category = "mudae")]
pub async fn remove_custom_image(ctx: Context<'_>, #[rest] text: String) -> Result<(), Error> {
    let pool = ctx.data().pool.clone();

    let (character_repo, image_repo) = {
        (
            CharacterRepository::new(pool.clone()),
            ImageRepository::new(pool.clone()),
        )
    };

    let (character_name, positions) = match parse_command_arguments(&text).await {
        Ok(result) => result,
        Err(error) => {
            ctx.say(error).await?;
            return Ok(());
        }
    };

    let character = match fetch_character_async(&character_repo, character_name).await {
        Some(c) => c,
        None => {
            ctx.say(CHARACTER_NOT_FOUND).await?;
            return Ok(());
        }
    };

    let images = match fetch_character_images_async(&image_repo, &character.id).await {
        Some(c) => c,
        None => {
            ctx.say(DATABASE_QUERY_ERROR).await?;
            return Ok(());
        }
    };

    if (images.len() as u32) < positions.iter().max().copied().unwrap_or_default() {
        ctx.say("Position out of limit").await?;
        return Ok(());
    }

    for &index in &positions {
        if let Some(image) = images.get(index.checked_sub(1).unwrap_or_default() as usize) {
            log_response(Level::DEBUG, &format!("{image:?}"));
            if let Err(err) = image_repo.remove_resource(&image.image_url).await {
                tracing::error!("Error removing image: {}", err);
                return Ok(());
            }
            tracing::debug!("Image removed successfully");
        };
    }

    match ctx {
        Context::Prefix(pctx) => {
            pctx.msg
                .react(pctx, ReactionType::Unicode("✅".to_string()))
                .await?;
        }
        Context::Application(actx) => {
            actx.reply("✅".to_string()).await?;
        }
    };
    Ok(())
}
async fn fetch_character_images_async(
    repo: &ImageRepository,
    character_id: &u32,
) -> Option<Vec<CustomImage>> {
    match repo.fetch_collection_by_character(*character_id).await {
        Ok(links) => Some(links),
        Err(why) => {
            log_response(
                Level::ERROR,
                &format!("Error: {}\nWhy: {}", DATABASE_QUERY_ERROR, why),
            );
            None
        }
    }
}

async fn fetch_character_async(
    repo: &CharacterRepository,
    character_name: &str,
) -> Option<Character> {
    match repo.fetch_resource(character_name).await {
        Ok(Some(character)) => Some(character),
        Ok(None) => None,
        Err(_) => {
            log_response(Level::ERROR, "Error fetching character");
            None
        }
    }
}
async fn parse_command_arguments(text: &str) -> Result<(&str, Vec<u32>), &str> {
    let mut args = text.split('$').map(|x| x.trim());

    let character_name = match args.next() {
        Some(s) if s.is_empty() => return Err(CHARACTER_NAME_NOT_PROVIDED),
        Some(name) => name,

        None => return Err(CHARACTER_NAME_NOT_PROVIDED),
    };

    let positions: Vec<u32> = args.filter_map(|s| s.parse().ok()).collect();

    if positions.is_empty() {
        return Err("No position provided");
    }

    Ok((character_name, positions))
}

#[cfg(test)]
mod test {
    use crate::{
        commands::mudae::image_delete::parse_command_arguments,
        constants::CHARACTER_NAME_NOT_PROVIDED,
    };

    #[tokio::test]
    async fn test_parse_command_arguments() {
        let result = parse_command_arguments("CharacterName $ 1$2").await;

        assert_eq!(result, Ok(("CharacterName", vec![1, 2])));
    }

    #[tokio::test]
    async fn test_parse_command_arguments_is_no_character() {
        let result = parse_command_arguments("$1").await;
        assert_eq!(result, Err(CHARACTER_NAME_NOT_PROVIDED));

        let result = parse_command_arguments("$").await;
        assert_eq!(result, Err(CHARACTER_NAME_NOT_PROVIDED));
        let result = parse_command_arguments("").await;
        assert_eq!(result, Err(CHARACTER_NAME_NOT_PROVIDED));
    }
    #[tokio::test]
    async fn test_parse_command_arguments_is_no_link() {
        let result = parse_command_arguments("CharacterName$$$").await;
        assert_eq!(result, Err("No position provided"));

        let result = parse_command_arguments("CharacterName").await;
        assert_eq!(result, Err("No position provided"));
    }
}
