pub mod image_create;
pub mod image_list;
use serenity::framework::standard::macros::group;

use self::image_create::*;

use self::image_list::*;

#[group("CustomImage")]
#[owner_privilege]
#[description = "Group guild CustomImages commands"]
#[summary = "CustomImages!"]
#[only_in(guilds)]
#[commands(add_sfw_custom_images, list_custom_images)]
struct CustomImage;

