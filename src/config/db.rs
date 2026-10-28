use sqlx::mysql::MySqlPool;
use std::env;

pub async fn connect() -> Result<MySqlPool, sqlx::Error> {
    let db_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://root:@localhost:3306/rust_sample".to_string());

    MySqlPool::connect(&db_url).await
}
