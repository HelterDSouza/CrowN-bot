use serenity::{
    all::{Embed, Message},
    client::Context,
};

use crate::{
    data::DatabasePool,
    db::{
        models::roll::EmbedRoll,
        repositories::{
            character_repo::CharacterRepository, serie_repo::SerieRepository, BaseRepository,
        },
    },
};

pub async fn parse_roll_embed(embed: &Embed) -> Result<EmbedRoll, &'static str> {
    let name = match &embed.author {
        Some(author) => author.name.clone(),
        None => String::default(),
    };

    let url = match &embed.image {
        Some(image) => image.url.clone(),
        None => String::default(),
    };

    let serie = match &embed.description {
        Some(description) => description
            .replace("<:wishprotect:633217581725122570>", "")
            .lines()
            .take(description.lines().count() - 1)
            .filter(|&line| !line.starts_with("<:"))
            .collect::<Vec<_>>()
            .join(" "),
        None => String::default(),
    };

    if name.is_empty() || url.is_empty() || serie.is_empty() {
        Err("One or more required fields are empty")
    } else {
        Ok(EmbedRoll::new(name, serie, url))
    }
}

pub async fn handle_rolls_message(msg: &Message, ctx: &Context) {
    let pool = ctx
        .data
        .read()
        .await
        .get::<DatabasePool>()
        .cloned()
        .expect("Should get database pool");

    // FIXME: Remover esse clone's
    let serie_repo = SerieRepository::new(pool.clone());
    let character_repo = CharacterRepository::new(pool.clone());

    //* MudaeBot
    if msg.author.id == 432610292342587392 {
        if let Some(embed) = msg.embeds.first() {
            if let Ok(roll) = parse_roll_embed(embed).await {
                let serie_id = serie_repo.fetch_id_or_create(&roll.serie).await.id;
                let _character = character_repo
                    .fetch_id_or_create(&roll.name, serie_id, &roll.url)
                    .await;
                tracing::debug!("💖 - {}: {}", &roll.name, &roll.serie);
            }
        }
    }
}
