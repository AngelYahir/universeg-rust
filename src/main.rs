use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;

mod controllers;
mod infra;
mod routes;
mod state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = infra::config::Config::from_env()?;
    infra::logging::init_tracing(cfg.debug);
    let db = infra::db::init_db_pool(&cfg.database_url).await?;
    let app_state = state::AppState::new(cfg.clone(), db);

    let app = routes::routes().with_state(app_state);

    let addr: SocketAddr = format!("0.0.0.0:{}", cfg.port).parse()?;
    info!(port = cfg.port, "Booting universeg API");

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
