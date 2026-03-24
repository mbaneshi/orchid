use anyhow::Result;
use axum::{Router, routing::get};
use tracing::info;

async fn health() -> &'static str {
    "orchid ok"
}

pub async fn serve() -> Result<()> {
    let config = orchid_core::Config::load()?;
    let addr = format!("{}:{}", config.web.host, config.web.port);

    let app = Router::new().route("/health", get(health));

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("orchid-web listening on {addr}");
    axum::serve(listener, app).await?;

    Ok(())
}
