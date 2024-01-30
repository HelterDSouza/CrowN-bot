use poise::{CreateReply, FrameworkError};
use serenity::{all::ReactionType, utils::MessageBuilder};
use tracing::Level;

use crate::{
    constants::{
        CHARACTER_NAME_NOT_PROVIDED, CHARACTER_NOT_FOUND, DATABASE_QUERY_ERROR,
        IMAGE_LINK_NOT_PROVIDED,
    },
    data::{Context, Data, Error},
    db::{
        models::character::Character,
        repositories::{
            character_repo::CharacterRepository, image_repo::ImageRepository, BaseRepository,
        },
    },
    utils::log_response,
};

#[poise::command(
    slash_command,
    prefix_command,
    aliases("aci"),
    category = "mudae",
    on_error = "argument_parse_error"
)]
pub async fn add_custom_image(
    ctx: Context<'_>,
    #[flag] nsfw: bool,
    #[rest] text: String,
) -> Result<(), Error> {
    let pool = ctx.data().pool.clone();

    // repositories
    let character_repo = CharacterRepository::new(pool.clone());
    let image_repo = ImageRepository::new(pool.clone());

    let owner = ctx.author().id;

    let (character_name, links) = match parse_command_arguments(&text).await {
        Ok(result) => result,
        Err(error) => {
            ctx.say(error).await?;
            return Ok(());
        }
    };

    if links.iter().any(|&link| !check_valid_host(link)) {
        log_response(Level::ERROR, "Invalid image host detected");
        ctx.say("Invalid Image host detected").await?;
        return Ok(());
    };

    let character = match fetch_character(character_repo, character_name).await {
        Ok(c) => c,
        Err(err) => {
            ctx.say(err).await?;
            return Ok(());
        }
    };

    //PERF: Pensar num parse melhor, so pegar o id da imagem e de onde ela é
    //NOTE: template  -> https://imgur.com/{id}.png
    //NOTE: template -> https://cdn.imgchest.com/files/{id}.png

    for link in links {
        // TODO: Add owner by
        match image_repo
            .create_resource(link, character.id, nsfw, owner.get() as i64)
            .await
        {
            Ok(_) => tracing::debug!("Image added successfully"),
            Err(err) => {
                tracing::error!("Error adding image: {err}");
                return Ok(());
            }
        }
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
async fn parse_command_arguments(text: &str) -> Result<(&str, Vec<&str>), &str> {
    let mut args = text.split('$').map(|x| x.trim());

    let character_name = match args.next() {
        Some(s) if s.is_empty() => {
            return Err(CHARACTER_NAME_NOT_PROVIDED);
        }
        Some(name) => name,
        None => {
            println!("aaaaaa");
            return Err(CHARACTER_NAME_NOT_PROVIDED);
        }
    };

    let links: Vec<&str> = args.filter(|s| !s.is_empty()).collect();

    if links.is_empty() {
        return Err(IMAGE_LINK_NOT_PROVIDED);
    }

    Ok((character_name, links))
}

async fn fetch_character(
    repo: CharacterRepository,
    character_name: &str,
) -> Result<Character, &str> {
    match repo.fetch_resource(character_name).await {
        Ok(Some(character)) => Ok(character),
        Ok(None) => Err(CHARACTER_NOT_FOUND),
        Err(_) => Err(DATABASE_QUERY_ERROR),
    }
}

async fn argument_parse_error(error: poise::FrameworkError<'_, Data, Error>) {
    let FrameworkError::ArgumentParse { ctx, .. } = error else {
        return;
    };

    let message = build_argument_error_message();
    let _reply_handle = ctx.send(CreateReply::default().content(message)).await;
}

fn build_argument_error_message() -> String {
    MessageBuilder::new()
        .push("Command Syntax: <prefix>aci <name of an existing character> $ <imgur OR imgchest link>")
        .push_line("This command adds a new image for the character.")
        .push_line("To add multiple images to the same character, separate the links with a $.")
        .push("\n")
        .push_line_safe("Ensure your image is hosted on <https://imgur.com/> or <https://imgchest.com/> and follows Discord rules.")
        .push("\n")
        .push("To ").push_bold("remove ").push("one image: ").push("<prefix>aci remove <name> $ <custom image position (between 1 and 50)>")
        .build()
}

fn check_valid_host(link: &str) -> bool {
    link.starts_with("https://i.imgur.com/")
        || link.starts_with("https://cdn.imgchest.com/files/")
        || link.starts_with("https://imgur.com/")
}
#[cfg(test)]
mod test {
    use crate::{
        commands::mudae::image_create::{check_valid_host, parse_command_arguments},
        constants::{CHARACTER_NAME_NOT_PROVIDED, IMAGE_LINK_NOT_PROVIDED},
    };

    #[tokio::test]
    async fn test_parse_command_arguments() {
        let result = parse_command_arguments("CharacterName $ Link1$ Link2").await;

        assert_eq!(result, Ok(("CharacterName", vec!["Link1", "Link2"])));
    }

    #[tokio::test]
    async fn test_parse_command_arguments_is_no_character() {
        let result = parse_command_arguments("$Link1").await;
        assert_eq!(result, Err(CHARACTER_NAME_NOT_PROVIDED));

        let result = parse_command_arguments("$").await;
        assert_eq!(result, Err(CHARACTER_NAME_NOT_PROVIDED));
        let result = parse_command_arguments("").await;
        assert_eq!(result, Err(CHARACTER_NAME_NOT_PROVIDED));
    }
    #[tokio::test]
    async fn test_parse_command_arguments_is_no_link() {
        let result = parse_command_arguments("CharacterName$$$").await;
        assert_eq!(result, Err(IMAGE_LINK_NOT_PROVIDED));

        let result = parse_command_arguments("CharacterName").await;
        assert_eq!(result, Err(IMAGE_LINK_NOT_PROVIDED));
    }
    #[tokio::test]
    async fn test_check_valid_host() {
        let image_link = "https://imgur.com/foo.png";
        assert!(check_valid_host(image_link))
    }

    #[tokio::test]
    async fn test_check_valid_host_imgur() {
        let image_link = "https://i.imgur.com/foo.png";
        assert!(check_valid_host(image_link))
    }

    #[tokio::test]
    async fn test_check_valid_host_image_chest() {
        let image_link = "https://cdn.imgchest.com/files/dummy.png";
        assert!(check_valid_host(image_link))
    }
}
