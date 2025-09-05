use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;

use universeg_api::{bootstrap, infrastructure};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = bootstrap::build_app().await?;
    let cfg = infrastructure::config::Config::from_env()?;

    let addr: SocketAddr = format!("0.0.0.0:{}", cfg.port).parse()?;
    info!(port = cfg.port, "Booting universeg API");

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
