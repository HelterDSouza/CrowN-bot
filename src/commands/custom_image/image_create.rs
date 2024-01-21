use serenity::{
    all::{Message, ReactionType},
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    utils::MessageBuilder,
};
use tracing::Level;

use crate::{
    constants::{
        CHARACTER_NAME_NOT_PROVIDED, CHARACTER_NOT_FOUND, DATABASE_QUERY_ERROR,
        IMAGE_LINK_NOT_PROVIDED,
    },
    data::DatabasePool,
    db::repositories::{
        character_repo::CharacterRepository, image_repo::ImageRepository, BaseRepository,
    },
    utils::{check_msg, loggin_response},
};

#[command]
#[aliases("aci", "addcustomimage")]
#[owner_privilege]
#[sub_commands(add_nsfw_custom_images, remove_custom_images)]
#[delimiters("$")]
pub async fn add_sfw_custom_images(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        let message = MessageBuilder::new()
            .push("Command Syntax: $aci <name of an existing character> $ <imgur OR imgchest link>")
            .push_line("This command adds a new image for the character.")
            .push_line("To add multiple images to the same character, separate the links with a $.")
            .push("\n\n")
            .push_line_safe("Ensure your image is hosted on <https://imgur.com/> or <https://imgchest.com/> and follows Discord rules.")
            .push("To ").push_bold("remove ").push("one image: ").push("$aci remove <name> $ <custom image position (between 1 and 50)>")
            .build();

        check_msg(msg.reply(&ctx.http, message).await);

        return Ok(());
    }
    add_custom_image(false, ctx, msg, &mut args).await
}

#[command]
#[aliases("nsfw")]
#[owner_privilege]
#[delimiters("$")]
pub async fn add_nsfw_custom_images(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        let message = MessageBuilder::new()
            .push("Command Syntax: $aci nsfw <name of an existing character> $ <imgur OR imgchest link>")
            .push_line("This command adds a new image for the character.")
            .push_line("This command adds a new image for the character (considered nsfw).")
            .push("\n\n")
            .push_line_safe("Ensure your image is hosted on <https://imgur.com/> or <https://imgchest.com/> and follows Discord rules.")
            .push("To ").push_bold("remove ").push("one image: ").push("$aci remove <name> $ <custom image position (between 1 and 50)>")
            .build();

        check_msg(msg.reply(&ctx.http, message).await);

        return Ok(());
    }
    add_custom_image(true, ctx, msg, &mut args).await
}

#[command]
#[aliases("remove")]
#[owner_privilege]
#[delimiters("$", " $ ")]
pub async fn remove_custom_images(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if args.is_empty() {
        let message = MessageBuilder::new()
            .push("Command Syntax: $aci <name of an existing character> $ <imgur OR imgchest link>")
            .push_line("This command adds a new image for the character.")
            .push_line("To add multiple images to the same character, separate the links with a $.")
            .push("\n\n")
            .push_line_safe("Ensure your image is hosted on <https://imgur.com/> or <https://imgchest.com/> and follows Discord rules.")
            .push("To ").push_bold("remove ").push("one image: ").push("$aci remove <name> $ <custom image position (between 1 and 50)>")
            .build();

        check_msg(msg.reply(&ctx.http, message).await);

        return Ok(());
    }

    remove_custom_image(ctx, msg, &mut args).await
}
// general custom_image
async fn remove_custom_image(ctx: &Context, msg: &Message, args: &mut Args) -> CommandResult {
    let data = ctx.data.read().await;
    let pool = data
        .get::<DatabasePool>()
        .cloned()
        .expect("expected a pool connection");

    // repositories
    let character_repo = CharacterRepository::new(pool.clone());
    let image_repo = ImageRepository::new(pool.clone());

    let character_name = match args.single::<String>() {
        Ok(name) => name,
        Err(_) => {
            check_msg(msg.reply(&ctx.http, CHARACTER_NAME_NOT_PROVIDED).await);
            return Ok(());
        }
    };

    let character = match character_repo.fetch_resource(character_name.trim()).await {
        Ok(Some(character)) => character,
        Ok(None) => {
            check_msg(msg.reply(&ctx.http, CHARACTER_NOT_FOUND).await);
            return Ok(());
        }
        Err(_) => {
            check_msg(msg.reply(&ctx.http, DATABASE_QUERY_ERROR).await);
            return Ok(());
        }
    };
    let mut links = match image_repo.fetch_collection_by_character(character.id).await {
        Ok(links) => links.into_iter().enumerate(),
        Err(_) => {
            loggin_response(Level::ERROR, DATABASE_QUERY_ERROR);
            return Ok(());
        }
    };
    loggin_response(Level::DEBUG, &format!("{links:?}"));
    while let Ok(index) = args.single::<u32>() {
        loggin_response(Level::DEBUG, &format!("{args:?}"));
        loggin_response(Level::DEBUG, &format!("{index}"));

        let remove_image = links.find(|link| link.0 as u32 + 1 == index).unwrap().1;
        loggin_response(Level::DEBUG, &format!("{remove_image:?}"));
        match image_repo.remove_resource(&remove_image.image_url).await {
            Ok(_) => tracing::debug!("Image remove successfully"),
            Err(err) => {
                tracing::error!("Error removing image: {err}");
                return Ok(());
            }
        }
    }

    Ok(())
}

async fn add_custom_image(
    is_nsfw: bool,
    ctx: &Context,
    msg: &Message,
    args: &mut Args,
) -> CommandResult {
    let data = ctx.data.read().await;
    let pool = data
        .get::<DatabasePool>()
        .cloned()
        .expect("expected a pool connection");

    // repositories
    let character_repo = CharacterRepository::new(pool.clone());
    let image_repo = ImageRepository::new(pool.clone());

    let character_name = match args.single::<String>() {
        Ok(name) => name,
        Err(_) => {
            check_msg(msg.reply(&ctx.http, CHARACTER_NAME_NOT_PROVIDED).await);
            return Ok(());
        }
    };

    let character = match character_repo.fetch_resource(character_name.trim()).await {
        Ok(Some(character)) => character,
        Ok(None) => {
            check_msg(msg.reply(&ctx.http, CHARACTER_NOT_FOUND).await);
            return Ok(());
        }
        Err(_) => {
            check_msg(msg.reply(&ctx.http, DATABASE_QUERY_ERROR).await);
            return Ok(());
        }
    };

    let links: Vec<&str> = args.raw().skip(1).map(|x| x.trim()).collect();

    if links.is_empty() {
        // loggin_response(Level::ERROR, "Please provide at least one image link");
        check_msg(msg.reply(&ctx.http, IMAGE_LINK_NOT_PROVIDED).await);
        return Ok(());
    }
    //PERF: Pensar num parse melhor, so pegar o id da imagem e de onde ela é
    //NOTE: template  -> https://imgur.com/{id}.png
    //NOTE: template -> https://cdn.imgchest.com/files/{id}.png
    if links.iter().any(|&link| !check_valid_host(link)) {
        loggin_response(Level::ERROR, "Invalid image host detected");
        check_msg(msg.reply(&ctx.http, "Invalid Image host detected").await);
        return Ok(());
    };

    // All links are from Imgur or Imgchest
    for link in &links {
        // TODO: Add owner by
        match image_repo
            .create_resource(link, character.id, is_nsfw)
            .await
        {
            Ok(_) => tracing::debug!("Image added successfully"),
            Err(err) => {
                tracing::error!("Error adding image: {err}");
                return Ok(());
            }
        }
    }

    let _ = msg
        .react(&ctx.http, ReactionType::Unicode("✅".to_string()))
        .await;
    Ok(())
}

fn check_valid_host(link: &str) -> bool {
    link.starts_with("https://i.imgur.com/")
        || link.starts_with("https://cdn.imgchest.com/files/")
        || link.starts_with("https://imgur.com/")
}

#[cfg(test)]
mod test {
    use crate::commands::custom_image::image_create::check_valid_host;

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

