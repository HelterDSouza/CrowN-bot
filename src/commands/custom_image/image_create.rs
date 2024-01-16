use serenity::{
    all::{Message, ReactionType},
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
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
#[sub_commands(add_nsfw_custom_images)]
#[delimiters("$")]
pub async fn add_custom_images(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let pool = data
        .get::<DatabasePool>()
        .cloned()
        .expect("expected a pool connection");

    // repositories
    let character_repo = CharacterRepository::new(pool.clone());
    let image_repo = ImageRepository::new(pool.clone());

    // get character name from args
    let character_name = match args.single::<String>() {
        Ok(name) => name,
        Err(_err) => {
            check_msg(msg.reply(&ctx.http, CHARACTER_NAME_NOT_PROVIDED).await);
            return Ok(());
        }
    };
    let character_name = character_name.trim();

    let character = match character_repo.fetch_resource(character_name).await {
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
        loggin_response(Level::ERROR, "Please provide at least one image link");
        check_msg(msg.reply(&ctx.http, IMAGE_LINK_NOT_PROVIDED).await);
        return Ok(());
    }
    //PERF: Pensar num parse melhor, so pegar o id da imagem e de onde ela é
    //NOTE: template  -> https://imgur.com/{id}.png
    //NOTE: template -> https://cdn.imgchest.com/files/{id}.png
    if links.iter().any(|&link| !image_host_parse(link)) {
        loggin_response(Level::ERROR, "Invalid image host detected");
        check_msg(msg.reply(&ctx.http, "Invalid Image host detected").await);
        return Ok(());
    };

    // All links are from Imgur or Imgchest
    for link in &links {
        // TODO: Add owner by
        match image_repo.create_resource(link, character.id, false).await {
            Ok(_) => tracing::debug!("Image added successfully"),
            Err(err) => tracing::error!("Error adding image: {err}"),
        }
    }

    let _ = msg
        .react(&ctx.http, ReactionType::Unicode("✅".to_string()))
        .await;
    Ok(())
}

#[command]
#[aliases("nsfw")]
#[owner_privilege]
#[delimiters("$")]
pub async fn add_nsfw_custom_images(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let pool = data
        .get::<DatabasePool>()
        .cloned()
        .expect("expected a pool connection");

    // repositories
    let character_repo = CharacterRepository::new(pool.clone());
    let image_repo = ImageRepository::new(pool.clone());

    // get character name from args
    let character_name = match args.single::<String>() {
        Ok(name) => name,
        Err(_err) => {
            check_msg(msg.reply(&ctx.http, CHARACTER_NAME_NOT_PROVIDED).await);
            return Ok(());
        }
    };
    let character_name = character_name.trim();

    let character = match character_repo.fetch_resource(character_name).await {
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
        loggin_response(Level::ERROR, "Please provide at least one image link");
        check_msg(msg.reply(&ctx.http, IMAGE_LINK_NOT_PROVIDED).await);
        return Ok(());
    }
    //PERF: Pensar num parse melhor, so pegar o id da imagem e de onde ela é
    //NOTE: template  -> https://imgur.com/{id}.png
    //NOTE: template -> https://cdn.imgchest.com/files/{id}.png
    if links.iter().any(|&link| !image_host_parse(link)) {
        loggin_response(Level::ERROR, "Invalid image host detected");
        check_msg(msg.reply(&ctx.http, "Invalid Image host detected").await);
        return Ok(());
    };

    // All links are from Imgur or Imgchest
    for link in &links {
        // TODO: Add owner by
        match image_repo.create_resource(link, character.id, true).await {
            Ok(_) => tracing::debug!("Image added successfully"),
            Err(err) => tracing::error!("Error adding image: {err}"),
        }
    }

    let _ = msg
        .react(&ctx.http, ReactionType::Unicode("✅".to_string()))
        .await;
    Ok(())
}
fn image_host_parse(link: &str) -> bool {
    link.starts_with("https://i.imgur.com/")
        || link.starts_with("https://cdn.imgchest.com/files/")
        || link.starts_with("https://imgur.com/")
}

