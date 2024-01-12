use serenity::{
    all::Message,
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
};

use crate::{
    constants::{CHARACTER_NAME_NOT_PROVIDED, CHARACTER_NOT_FOUND, DATABASE_QUERY_ERROR},
    data::DatabasePool,
    db::repositories::{
        character_repo::CharacterRepository, image_repo::ImageRepository, BaseRepository,
    },
};

#[command]
#[aliases("aci", "addcustomimage", "ai")]
#[owner_privilege]
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
            let _ = msg.reply(&ctx.http, CHARACTER_NAME_NOT_PROVIDED).await;
            return Ok(());
        }
    };
    let character_name = character_name.trim();

    let character = match character_repo.fetch_resource(character_name).await {
        Ok(Some(character)) => character,
        Ok(None) => {
            tracing::error!(CHARACTER_NOT_FOUND);
            return Ok(());
        }
        Err(_) => {
            tracing::error!(DATABASE_QUERY_ERROR);
            return Ok(());
        }
    };
    let links: Vec<&str> = args.raw().skip(1).map(|x| x.trim()).collect();

    if links.is_empty() {
        tracing::error!("Please provide at least one image link");
        return Ok(());
    }
    //PERF: Pensar num parse melhor, so pegar o id da imagem e de onde ela é
    //NOTE: template  -> https://imgur.com/{id}.png
    //NOTE: template -> https://cdn.imgchest.com/files/{id}.png
    if links.iter().any(|&link| !image_host_parse(link)) {
        tracing::error!("Invalid image host detected");
    } else {
        // All links are from Imgur or Imgchest
        for link in &links {
            match image_repo.create_resource(link, character.id).await {
                Ok(_) => tracing::debug!("Image added successfully"),
                Err(err) => tracing::error!("Error adding image: {err}"),
            }
        }
    }
    Ok(())
}
fn image_host_parse(link: &str) -> bool {
    link.starts_with("https://i.imgur.com/")
        || link.starts_with("https://cdn.imgchest.com/files/")
        || link.starts_with("https://imgur.com/")
}

