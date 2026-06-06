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
/// `.serve_static_assets()` is REQUIRED: it registers routes for the dx-generated
/// WASM bundle (e.g. `/wasm/portfolio.js`, `/wasm/portfolio_bg.wasm`) from the
/// `public/` directory placed next to the server binary by `dx serve`.  Without it,
/// those paths fall through to `render_handler`, which returns SSR HTML instead of
/// JS/WASM — causing the WASM bootstrap to fail and the page to never become
/// interactive.
pub fn build_router() -> Router {
    let state = FullstackState::new(ServeConfig::new(), App);
    // Explicit state type so DioxusRouterExt (impl'd on Router<FullstackState>) is reachable
    // before .with_state() consumes it.
    let router: Router<FullstackState> = Router::new()
        .route("/api/contact", post(crate::handlers::contact::contact))
        .route("/api/pageview", post(crate::handlers::pageview::pageview))
        // Serve the project's own asset tree (sounds, styles, images, Bevy WASM, etc.).
        .nest_service("/assets", ServeDir::new("assets"))
        // Serve dx-generated WASM bundle + any other files placed in the public/ directory.
        // Without this, /wasm/*.js and /wasm/*.wasm fall through to render_handler and
        // return SSR HTML → the WASM never loads → page hangs / never becomes interactive.
        .serve_static_assets()
        .fallback(get(render_handler));
    router.with_state(state)
}
