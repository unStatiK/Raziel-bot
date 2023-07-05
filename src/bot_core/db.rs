use crate::bot_core::constants::DB_FILE_NAME;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Error, Executor, SqlitePool};

pub struct RzDb;

impl RzDb {
    pub async fn get_connection() -> Result<SqlitePool, Error> {
        let opts = SqliteConnectOptions::new()
            .pragma("encoding", "\"UTF-8\"")
            .pragma("synchronous", "FULL")
            .create_if_missing(true)
            .filename(DB_FILE_NAME);
        SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(opts)
            .await
    }

    pub async fn tx_execute(pool: &SqlitePool, plain_query: &str) -> bool {
        let tx = pool.begin().await;
        match tx {
            Ok(mut tx) => {
                let result = tx.execute(plain_query).await;
                let tx_result = tx.commit().await;
                if result.is_ok() && tx_result.is_ok() {
                    return true;
                }
                false
            },
            Err(_e) => false
        }
    }
}