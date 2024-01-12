pub mod image_create;

use serenity::framework::standard::macros::group;

use self::image_create::*;

#[group("CustomImage")]
#[owner_privilege]
#[description = "Group guild CustomImages commands"]
#[summary = "CustomImages!"]
#[only_in(guilds)]
#[commands(add_custom_images)]
struct CustomImage;
