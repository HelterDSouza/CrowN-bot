use sqlx::{Pool, Sqlite};

use crate::db::models::account::Account;

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
            r#"INSERT INTO Accounts(discord_id) values($1)"#,
            account.discord_id,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
    pub async fn fetch_resource(&self, discord_id: i64) -> Result<Account, sqlx::Error> {
        let row = sqlx::query_as!(
            Account,
            r#"SELECT discord_id FROM Accounts WHERE discord_id = $1"#,
            discord_id,
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(row)
    }
}
