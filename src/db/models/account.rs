pub struct Account {
    pub id: Option<u64>,
    pub discord_id: i64,
    pub username: String,
    pub discriminator: String,
    pub global_name: String,
}
pub struct AccountResponse {
    pub discord_id: i64,
    pub username: String,
    pub discriminator: String,
    pub global_name: String,
}
