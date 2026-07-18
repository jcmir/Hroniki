use sqlx::{
    sqlite::{
        SqlitePool,
        SqlitePoolOptions,
    },
};

pub async fn create_pool(
    database_url: &str,
) -> Result<SqlitePool, sqlx::Error> {

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
}
