use anyhow::Context;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use tracing::info;

pub type DbPool = Pool<Postgres>;

pub async fn init_db_pool(database_url: &str) -> anyhow::Result<DbPool> {
    info!("Initializing Postgres pool");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .context("Failed to create database connection pool")?;

    let size = pool.size();
    let idle = pool.num_idle();
    info!(pool_size = size, pool_idle = idle, "DB pool ready ");
    Ok(pool)
}
