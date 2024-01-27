use crate::{
    data::{Context, Error},
    paginate::paginate,
};
use poise::serenity_prelude::{self as serenity};

// use serenity::{
//     all::Message,
//     builder::{CreateEmbed, CreateMessage},
//     client::Context,
//     framework::standard::{macros::command, Args, CommandResult},
// };
//
// use crate::{
//     data::DatabasePool,
//     db::repositories::{image_repo::ImageRepository, BaseRepository},
// };
//
#[poise::command(prefix_command, slash_command, aliases("cil"))]
pub async fn list_custom_images(ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}
// #[command]
// #[aliases("cil")]
// #[owner_privilege]
// #[delimiters("$")]
// pub async fn list_custom_images(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult {
//     // NOTE: list user added images, without args
//     let data = ctx.data.read().await;
//     let pool = data
//         .get::<DatabasePool>()
//         .cloned()
//         .expect("expected a pool connection");
//
//     let list_embed = CreateEmbed::new();
//     let images_repo = ImageRepository::new(pool);
//     let teste = match images_repo
//         .fetch_collection(msg.author.id.into(), false)
//         .await
//     {
//         Ok(images) => list_embed.description(
//             images
//                 .iter()
//                 .map(|image| image.image_url.clone())
//                 .collect::<Vec<_>>()
//                 .join("\n"),
//         ),
//         Err(err) => {
//             tracing::error!("{err:?}");
//             return Ok(());
//         }
//     };
//     msg.channel_id
//         .send_message(&ctx.http, CreateMessage::new().embed(teste))
//         .await;
//     //    let data = ctx.data.read().await;
//     //    let pool = data
//     //        .get::<DatabasePool>()
//     //        .cloned()
//     //        .expect("expected a pool connection");
//     //    // repositories
//     //    let character_repo = CharacterRepository::new(pool.clone());
//     //    let image_repo = ImageRepository::new(pool.clone());
//     //    let owner_by = msg.author.id;
//     //    // get character name from args
//     //    let character_name = match args.single::<String>() {
//     //        Ok(name) => name,
//     //        Err(_err) => {
//     //            let _ = msg.reply(&ctx.http, CHARACTER_NAME_NOT_PROVIDED).await;
//     //            return Ok(());
//     //        }
//     //    };
//     //    let character_name = character_name.trim();
//     //
//     //    let character = match character_repo.fetch_resource(character_name).await {
//     //        Ok(Some(character)) => character,
//     //        Ok(None) => {
//     //            tracing::error!(CHARACTER_NOT_FOUND);
//     //            return Ok(());
//     //        }
//     //        Err(_) => {
//     //            tracing::error!(DATABASE_QUERY_ERROR);
//     //            return Ok(());
//     //        }
//     //    };
//     //    let links: Vec<&str> = args.raw().skip(1).map(|x| x.trim()).collect();
//     //
//     //    if links.is_empty() {
//     //        tracing::error!("Please provide at least one image link");
//     //        return Ok(());
//     //    }
//     //    //PERF: Pensar num parse melhor, so pegar o id da imagem e de onde ela é
//     //    //NOTE: template  -> https://imgur.com/{id}.png
//     //    //NOTE: template -> https://cdn.imgchest.com/files/{id}.png
//     //    if links.iter().any(|&link| !image_host_parse(link)) {
//     //        tracing::error!("Invalid image host detected");
//     //    } else {
//     //        // All links are from Imgur or Imgchest
//     //        for link in &links {
//     //            // TODO: Add owner by
//     //            match image_repo.create_resource(link, character.id, false).await {
//     //                Ok(_) => tracing::debug!("Image added successfully"),
//     //                Err(err) => tracing::error!("Error adding image: {err}"),
//     //            }
//     //        }
//     //    }
//     Ok(())
// }
//

