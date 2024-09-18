use sqlx::{pool::PoolOptions, PgPool};

pub async fn connect_db(db_url: &str) -> PgPool {
    let max_connections = 5;
    PoolOptions::new()
        .max_connections(max_connections)
        .connect(db_url)
        .await
        .expect("Failed to connect to DB")
}
