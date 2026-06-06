use axum::{
    routing::{get, post},
    Router,
};
use dioxus::server::{render_handler, DioxusRouterExt, FullstackState, ServeConfig};
use tower_http::services::ServeDir;

use crate::App;

/// Build the Axum router: SSR fallback + validated API handlers + static assets.
///
/// Per PORT-MONOLITH-1: one binary, one process, one port.
///
/// ASSET ROUTING STRATEGY (fixes hydration-never-attaches bug):
///
/// In release mode, dx CLI emits its hydration bootstrap script and WASM bundle
/// under `public/assets/` (e.g. `public/assets/portfolio-dxhb<hash>.js` and
/// `public/assets/portfolio_bg-dxh<hash>.wasm`).  The SSR HTML references them at
/// `/assets/<hashed-name>`.  We must let `serve_static_assets()` own the `/assets`
/// URL prefix so those files are served.
///
/// `serve_static_assets()` (dioxus-server 0.7.9) calls `serve_dir_cached()` which
/// reads `exe/../public/` at startup and for each entry calls
/// `nest_service("/<entry>", ServeDir::new(...))` — so it registers `/assets` for
/// `public/assets/` automatically.  If we ALSO call
/// `nest_service("/assets", ServeDir::new("assets"))` we get an axum route-conflict
/// panic.
///
/// Fix: give `/assets` entirely to `serve_static_assets()`.  Serve OUR repo static
/// files (CSS, images, sounds, Bevy WASM loader) at `/static` instead, by mounting
/// `ServeDir::new("assets")` (the on-disk dir keeps its name so `make build-bevy`
/// `--out-dir assets` continues to work) at the `/static` URL prefix.
///
/// All `/assets/...` hrefs in lib.rs and component files have been updated to
/// `/static/...` to match this new prefix.  dx's `/assets/...` references in its
/// generated HTML are left untouched — those are served by `serve_static_assets()`.
pub fn build_router() -> Router {
    let state = FullstackState::new(ServeConfig::new(), App);

    // Explicit state type so DioxusRouterExt (impl'd on Router<FullstackState>) is reachable
    // before .with_state() consumes it.
    let router: Router<FullstackState> = Router::new()
        .route("/api/contact", post(crate::handlers::contact::contact))
        .route("/api/pageview", post(crate::handlers::pageview::pageview))
        // Repo static files: CSS, images, sounds, Bevy WASM loader.
        // Mounted at /static to leave /assets free for dx's serve_static_assets().
        // On-disk dir is still named `assets/`; only the URL prefix changed.
        .nest_service("/static", ServeDir::new("assets"))
        // Let dx own /assets (and any other subdirs it creates in public/).
        // Registers nest_service("/assets", ...) from public/assets/ at runtime.
        .serve_static_assets()
        // Register dioxus server functions (e.g. RSX server actions).
        .register_server_functions();

    router.fallback(get(render_handler)).with_state(state)
}
