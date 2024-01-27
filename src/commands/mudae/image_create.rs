use poise::{CreateReply, FrameworkError};
use serenity::utils::MessageBuilder;
use tracing::Level;

use crate::{
    constants::{
        CHARACTER_NAME_NOT_PROVIDED, CHARACTER_NOT_FOUND, DATABASE_QUERY_ERROR,
        IMAGE_LINK_NOT_PROVIDED,
    },
    data::{Context, Data, Error},
    db::repositories::{
        account_repo::AccountRepository, character_repo::CharacterRepository,
        image_repo::ImageRepository, BaseRepository,
    },
    utils::log_response,
};

#[poise::command(
    slash_command,
    prefix_command,
    aliases("aci"),
    category = "mudae",
    check = "check_valid_user",
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

    let mut args = text.split('$').map(|x| x.trim());

    let character_name = match args.next() {
        Some(name) => name,
        None => {
            ctx.reply(CHARACTER_NAME_NOT_PROVIDED).await?;
            return Ok(());
        }
    };

    let character = match character_repo.fetch_resource(character_name).await {
        Ok(Some(character)) => character,
        Ok(None) => {
            ctx.reply(CHARACTER_NOT_FOUND).await?;
            return Ok(());
        }
        Err(_) => {
            ctx.reply(DATABASE_QUERY_ERROR).await?;
            return Ok(());
        }
    };

    let links: Vec<&str> = args.collect();

    if links.is_empty() {
        ctx.reply(IMAGE_LINK_NOT_PROVIDED).await?;
        return Ok(());
    }

    if links.iter().any(|&link| !check_valid_host(link)) {
        log_response(Level::ERROR, "Invalid image host detected");
        ctx.reply("Invalid Image host detected").await?;
        return Ok(());
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

    let _ = ctx.reply("✅".to_string()).await?;

    Ok(())
}

async fn argument_parse_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        FrameworkError::ArgumentParse { ctx, .. } => {
            let message = build_argument_error_message();
            let _reply_handle = ctx.send(CreateReply::default().content(message)).await;
        }
        _ => todo!(),
    }
}

fn build_argument_error_message() -> String {
    MessageBuilder::new()
        .push("Command Syntax: $aci <name of an existing character> $ <imgur OR imgchest link>")
        .push_line("This command adds a new image for the character.")
        .push_line("To add multiple images to the same character, separate the links with a $.")
        .push("\n")
        .push_line_safe("Ensure your image is hosted on <https://imgur.com/> or <https://imgchest.com/> and follows Discord rules.")
        .push("To ").push_bold("remove ").push("one image: ").push("$aci remove <name> $ <custom image position (between 1 and 50)>")
        .build()
}

fn check_valid_host(link: &str) -> bool {
    link.starts_with("https://i.imgur.com/")
        || link.starts_with("https://cdn.imgchest.com/files/")
        || link.starts_with("https://imgur.com/")
}
async fn check_valid_user(ctx: Context<'_>) -> Result<bool, Error> {
    let pool = ctx.data().pool.clone();

    if let Err(err) = AccountRepository::new(pool)
        .fetch_resource(ctx.author().id.get() as i64)
        .await
    {
        return Ok(false);
    };
    return Ok(true);
}
#[cfg(test)]
mod test {
    use crate::commands::mudae::image_create::check_valid_host;

    #[test]
    fn test_check_valid_host() {
        let image_link = "https://imgur.com/foo.png";
        assert!(check_valid_host(image_link))
    }
    #[test]
    fn test_check_valid_host_imgur() {
        let image_link = "https://i.imgur.com/foo.png";
        assert!(check_valid_host(image_link))
    }
    #[test]
    fn test_check_valid_host_image_chest() {
        let image_link = "https://cdn.imgchest.com/files/dummy.png";
        assert!(check_valid_host(image_link))
    }
}
