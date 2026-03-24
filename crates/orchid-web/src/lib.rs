use anyhow::Result;
use axum::response::Html;
use axum::{routing::get, Router};
use tracing::info;

async fn index() -> Html<&'static str> {
    Html(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Orchid — Agentic Dev OS</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
            background: #0a0a0a; color: #e0e0e0;
            display: flex; align-items: center; justify-content: center;
            min-height: 100vh;
        }
        .container { max-width: 640px; padding: 2rem; text-align: center; }
        h1 { font-size: 2.5rem; margin-bottom: 0.5rem; color: #b48ead; }
        .tagline { color: #88c0d0; font-size: 1.1rem; margin-bottom: 2rem; }
        .status {
            background: #1a1a2e; border: 1px solid #2a2a4a; border-radius: 8px;
            padding: 1.5rem; margin-bottom: 1.5rem; text-align: left;
        }
        .status h2 { font-size: 1rem; color: #a3be8c; margin-bottom: 0.75rem; }
        .status ul { list-style: none; }
        .status li { padding: 0.3rem 0; font-size: 0.9rem; }
        .status li::before { content: "→ "; color: #b48ead; }
        .commands {
            background: #1a1a2e; border: 1px solid #2a2a4a; border-radius: 8px;
            padding: 1.5rem; text-align: left; font-family: monospace; font-size: 0.85rem;
        }
        .commands code { color: #a3be8c; }
        .commands .dim { color: #666; }
        a { color: #88c0d0; }
    </style>
</head>
<body>
    <div class="container">
        <h1>Orchid</h1>
        <p class="tagline">Agentic Dev OS — v0.1.0</p>
        <div class="status">
            <h2>Surfaces</h2>
            <ul>
                <li>Web dashboard — you are here</li>
                <li>TUI workspace — <code>orchid workspace</code></li>
                <li>CLI agents — <code>orchid agent</code></li>
            </ul>
        </div>
        <div class="status">
            <h2>Domains</h2>
            <ul>
                <li>Code sessions (repos, branches, PRs)</li>
                <li>Content sessions (posts, threads, newsletters)</li>
                <li>Relationship sessions (contacts, leads, clients)</li>
                <li>Flows (trigger → agent → action)</li>
            </ul>
        </div>
        <div class="commands">
            <div><span class="dim"># summarize git → draft content</span></div>
            <div><code>orchid flow -n dev-to-content -r .</code></div>
            <br>
            <div><span class="dim"># health check</span></div>
            <div><code>curl localhost:3100/health</code></div>
        </div>
        <br>
        <p><a href="https://github.com/mbaneshi/orchid">github.com/mbaneshi/orchid</a></p>
    </div>
</body>
</html>"#,
    )
}

async fn health() -> &'static str {
    "orchid ok"
}

pub async fn serve() -> Result<()> {
    let config = orchid_core::Config::load()?;
    let addr = format!("{}:{}", config.web.host, config.web.port);

    let app = Router::new()
        .route("/", get(index))
        .route("/health", get(health));

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("orchid-web listening on {addr}");
    axum::serve(listener, app).await?;

    Ok(())
}
