use serenity::framework::standard::macros::group;



#[group("Admin")]
#[owner_privilege]
#[description = "Group guild admins commands"]
#[summary = "Admin!"]
#[only_in(guilds)]
struct Admin;
