use sqlx;
use sqlx::MySql;
use sqlx::pool::PoolOptions;

pub async fn create_pool(database_url: &str) -> Result<sqlx::MySqlPool, sqlx::Error> {
    let pool = sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    Ok(pool)
}