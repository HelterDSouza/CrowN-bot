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

    let (character_name, mut positions) = match parse_command_arguments(&text).await {
        Ok(result) => result,
        Err(error) => {
            ctx.say(error).await?;
            return Ok(());
        }
    };

    let character = match fetch_character(character_repo, character_name).await {
        Ok(c) => c,
        Err(err) => {
            ctx.say(err).await?;
            return Ok(());
        }
    };

    let images = match fetch_character_images(image_repo, &character.id).await {
        Ok(c) => c.iter,
        Err(err) => {
            ctx.say(err).await?;
            return Ok(());
        }
    };

    for index in positions {
        log_response(Level::DEBUG, &format!("{index}"));

        let remove_image = images.find(|link| link.0 as u32 + 1 == index).unwrap().1;
        // log_response(Level::DEBUG, &format!("{remove_image:?}"));
        // match image_repo.remove_resource(&remove_image.image_url).await {
        //     Ok(_) => tracing::debug!("Image remove successfully"),
        //     Err(err) => {
        //         tracing::error!("Error removing image: {err}");
        //         return Ok(());
        //     }
        // }
    }
    Ok(())
}

async fn fetch_character_images(
    repo: ImageRepository,
    character_id: &u32,
) -> Result<Vec<CustomImage>, &str> {
    match repo.fetch_collection_by_character(*character_id).await {
        Ok(links) => Ok(links),
        Err(why) => {
            log_response(
                Level::ERROR,
                &format!("Error: {DATABASE_QUERY_ERROR}\nWhy: {why}"),
            );
            Err(DATABASE_QUERY_ERROR)
        }
    }
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
// let character = match character_repo.fetch_resource(character_name.trim()).await { Ok(Some(character)) => character,
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
