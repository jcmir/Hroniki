use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::str::FromStr;

pub async fn create_pool(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true)
        .foreign_keys(true);

    let max_connections = if database_url.starts_with("sqlite::memory:") {
        1
    } else {
        5
    };

    SqlitePoolOptions::new()
        .max_connections(max_connections)
        .connect_with(options)
        .await
}
