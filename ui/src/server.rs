use axum::{
    routing::{get, post},
    Router,
};
use dioxus::server::{render_handler, DioxusRouterExt, FullstackState, ServeConfig};
use std::path::PathBuf;
use tower_http::services::ServeDir;

use crate::App;

/// Replicate dioxus-server's internal `public_path()` logic:
/// honour `DIOXUS_PUBLIC_PATH` env var first, then fall back to `<exe>/../public`.
///
/// The resulting path is the directory that `dx serve` / `dx build` places the
/// compiled WASM bundle into (e.g. `public/wasm/portfolio.js`).
fn dx_public_path() -> Option<PathBuf> {
    if let Ok(path) = std::env::var("DIOXUS_PUBLIC_PATH") {
        return Some(PathBuf::from(path));
    }
    Some(
        std::env::current_exe()
            .ok()?
            .parent()?
            .join("public"),
    )
}

/// Build the Axum router: SSR fallback + validated API handlers + static assets.
///
/// Per PORT-MONOLITH-1: one binary, one process, one port.
///
/// WHY we mount manually instead of calling `.serve_static_assets()`:
///
/// `serve_static_assets()` enumerates every subdirectory in `public/` and calls
/// `nest_service("<subdir>", ServeDir::new(...))` for each one.  The dx CLI always
/// creates `public/assets/` (even though it puts nothing there — the WASM bundle
/// goes into `public/wasm/`).  Our own `nest_service("/assets", ServeDir::new("assets"))`
/// then collides with that registration, causing axum to panic on startup:
///
///   "Invalid route "/assets/{*...}": Insertion failed due to conflict with
///    previously registered route: /assets/{*...}"
///
/// Fix: skip `serve_static_assets()` entirely and mount only what we need:
///   - `/assets/...`  → repo's `assets/` dir  (CSS, images, sounds, Bevy JS/WASM)
///   - `/wasm/...`    → `public/wasm/`          (dx-built portfolio.js / portfolio_bg.wasm)
///
/// This is robust to dx rebuilds: dx never writes into `public/assets/`, so there
/// is no clobber risk.  If dx ever adds new top-level subdirs to `public/` they will
/// NOT be served automatically — but in practice dx only uses `public/wasm/`.
pub fn build_router() -> Router {
    let state = FullstackState::new(ServeConfig::new(), App);

    // Explicit state type so DioxusRouterExt (impl'd on Router<FullstackState>) is reachable
    // before .with_state() consumes it.
    let mut router: Router<FullstackState> = Router::new()
        .route("/api/contact", post(crate::handlers::contact::contact))
        .route("/api/pageview", post(crate::handlers::pageview::pageview))
        // Our raw static files: CSS, images, sounds, Bevy WASM loader.
        // All absolute "/assets/..." URLs in lib.rs and component files resolve here.
        .nest_service("/assets", ServeDir::new("assets"));

    // Mount the dx-generated WASM bundle at /wasm.  This is the path the dx-generated
    // index.html references (e.g. <script src="/wasm/portfolio.js">).
    // We mount only the `wasm/` subdirectory to avoid any collision with `/assets`.
    if let Some(public_path) = dx_public_path() {
        let wasm_path = public_path.join("wasm");
        if wasm_path.exists() {
            router = router.nest_service("/wasm", ServeDir::new(wasm_path));
        }
    }

    // Register dioxus server functions (e.g. RSX server actions) without
    // re-running serve_static_assets.
    router = router.register_server_functions();

    router
        .fallback(get(render_handler))
        .with_state(state)
}
