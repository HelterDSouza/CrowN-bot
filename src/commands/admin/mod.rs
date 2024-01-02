use serenity::framework::standard::macros::group;

pub mod prefix;
pub mod roll_channel;

use self::prefix::*;
use self::roll_channel::*;

#[group("Admin")]
#[owner_privilege]
#[description = "Group guild admins commands"]
#[summary = "Admin!"]
#[only_in(guilds)]
#[commands(set_prefix, set_roll_channel)]
struct Admin;
