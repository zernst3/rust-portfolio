// Server entry: native binary with custom Axum router + Dioxus SSR.
// Per PORT-FULLSTACK-1: target-based gating — native builds get server deps
// automatically via [target.'cfg(not(target_arch="wasm32"))'.dependencies];
// no dx @server/--features flag needed to suppress them on wasm32.
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    // Bind address resolution:
    // - When `PORT` is set (production container, e.g. Azure Container Apps; and
    //   also `dx serve`, which injects IP+PORT), bind 0.0.0.0:$PORT. Binding all
    //   interfaces is REQUIRED behind a container ingress — 127.0.0.1 would be
    //   unreachable from the platform proxy. 0.0.0.0 still covers localhost, so
    //   `dx serve`'s proxy connects fine in dev.
    // - Otherwise (running the binary bare locally) fall back to
    //   `fullstack_address_or_localhost()` → 127.0.0.1:8080.
    let addr: std::net::SocketAddr = match std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
    {
        Some(port) => std::net::SocketAddr::from(([0, 0, 0, 0], port)),
        None => dioxus_cli_config::fullstack_address_or_localhost(),
    };
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
