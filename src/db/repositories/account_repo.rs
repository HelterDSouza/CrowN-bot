use sqlx::{Pool, Sqlite};

use crate::db::models::account::{Account, AccountResponse};

use super::BaseRepository;

pub struct AccountRepository {
    pub pool: Pool<Sqlite>,
}

impl BaseRepository for AccountRepository {
    fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

impl AccountRepository {
    pub async fn create_resource(&self, account: Account) -> Result<(), sqlx::Error> {
        let _ = sqlx::query_as!(
            Account,
            r#"INSERT INTO Accounts(discord_id,username,discriminator,global_name) values($1,$2,$3,$4)"#,
            account.discord_id,
            account.username,
            account.discriminator,
            account.global_name
        )

        .execute(&self.pool).await?;
        Ok(())
    }
    pub async fn fetch_resource(&self, discord_id: i64) -> Result<AccountResponse, sqlx::Error> {
        let row = sqlx::query_as!(
            AccountResponse,
            r#"SELECT discord_id,username,discriminator,global_name as "global_name!" FROM Accounts WHERE discord_id = $1"#,
            discord_id,
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(row)
    }
}
