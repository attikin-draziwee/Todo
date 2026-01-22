use std::time::Duration;

use sqlx::{MySql, Pool, mysql::MySqlPoolOptions};

pub async fn configure_db(db_url: &str) -> Result<Pool<MySql>, sqlx::Error> {
    MySqlPoolOptions::new()
        .max_connections(20)
        .min_connections(5)
        .acquire_timeout(Duration::from_secs(10)) // Таймаут получения подключения
        .idle_timeout(Duration::from_secs(600)) // Время жизни idle подключения
        .max_lifetime(Duration::from_secs(1800)) // Максимальное время жизни
        .connect(db_url)
        .await
}
