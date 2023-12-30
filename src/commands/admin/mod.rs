use serenity::framework::standard::macros::group;

pub mod prefix;

use self::prefix::*;

#[group("Admin")]
#[owner_privilege]
#[description = "Group guild admins commands"]
#[summary = "Admin!"]
#[only_in(guilds)]
#[commands(set_prefix)]
struct Admin;
