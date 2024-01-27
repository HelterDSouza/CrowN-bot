// async fn remove_custom_image(ctx: &Context, msg: &Message, args: &mut Args) -> CommandResult {
//     let data = ctx.data.read().await;
//     let pool = data
//         .get::<DatabasePool>()
//         .cloned()
//         .expect("expected a pool connection");
//
//     // repositories
//     let character_repo = CharacterRepository::new(pool.clone());
//     let image_repo = ImageRepository::new(pool.clone());
//
//     let character_name = match args.single::<String>() {
//         Ok(name) => name,
//         Err(_) => {
//             check_msg(msg.reply(&ctx.http, CHARACTER_NAME_NOT_PROVIDED).await);
//             return Ok(());
//         }
//     };
//
//     let character = match character_repo.fetch_resource(character_name.trim()).await {
//         Ok(Some(character)) => character,
//         Ok(None) => {
//             check_msg(msg.reply(&ctx.http, CHARACTER_NOT_FOUND).await);
//             return Ok(());
//         }
//         Err(_) => {
//             check_msg(msg.reply(&ctx.http, DATABASE_QUERY_ERROR).await);
//             return Ok(());
//         }
//     };
//     let mut links = match image_repo.fetch_collection_by_character(character.id).await {
//         Ok(links) => links.into_iter().enumerate(),
//         Err(_) => {
//             loggin_response(Level::ERROR, DATABASE_QUERY_ERROR);
//             return Ok(());
//         }
//     };
//     loggin_response(Level::DEBUG, &format!("{links:?}"));
//     while let Ok(index) = args.single::<u32>() {
//         loggin_response(Level::DEBUG, &format!("{args:?}"));
//         loggin_response(Level::DEBUG, &format!("{index}"));
//
//         let remove_image = links.find(|link| link.0 as u32 + 1 == index).unwrap().1;
//         loggin_response(Level::DEBUG, &format!("{remove_image:?}"));
//         match image_repo.remove_resource(&remove_image.image_url).await {
//             Ok(_) => tracing::debug!("Image remove successfully"),
//             Err(err) => {
//                 tracing::error!("Error removing image: {err}");
//                 return Ok(());
//             }
//         }
//     }
//
//     Ok(())
// }

