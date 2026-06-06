// Server entry: native binary with custom Axum router + Dioxus SSR.
// Per PORT-FULLSTACK-1: target-based gating — native builds get server deps
// automatically via [target.'cfg(not(target_arch="wasm32"))'.dependencies];
// no dx @server/--features flag needed to suppress them on wasm32.
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("listening on {addr}");
    axum::serve(listener, ui::build_router()).await?;
    Ok(())
}

// Web entry: WASM hydration. dx compiles this branch for wasm32-unknown-unknown.
// `dioxus::launch` boots the client-side hydration runtime.
// Per PORT-FULLSTACK-1 / rust-dioxus-11.
#[cfg(target_arch = "wasm32")]
fn main() {
    dioxus::launch(ui::App);
}
