use anyhow::Context;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub type DbPool = Pool<Postgres>;

pub async fn init_db_pool(database_url: &str) -> anyhow::Result<DbPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .context("Failed to create database connection pool")?;

    print!("Database connected ✅");
    Ok(pool)
}
