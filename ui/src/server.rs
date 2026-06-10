use axum::{
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use dioxus::server::{render_handler, DioxusRouterExt, FullstackState, ServeConfig};
use tower_http::{compression::CompressionLayer, services::ServeDir};

use crate::App;

/// `/robots.txt` served verbatim from the repo. The companion `ui/index.html`
/// is picked up automatically by `dx bundle` (it scans the crate root for a
/// custom `index.html`) and replaces dioxus-cli's default prod template,
/// which is missing `<html lang>` and the meta description — Lighthouse
/// flags both.
const ROBOTS_TXT: &str = include_str!("../robots.txt");

async fn robots_txt() -> impl IntoResponse {
    ([(axum::http::header::CONTENT_TYPE, "text/plain; charset=utf-8")], ROBOTS_TXT)
}

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
/// files (CSS, images, sounds) at `/static` instead, by mounting
/// `ServeDir::new("assets")` at the `/static` URL prefix.
///
/// All `/assets/...` hrefs in lib.rs and component files have been updated to
/// `/static/...` to match this new prefix.  dx's `/assets/...` references in its
/// generated HTML are left untouched — those are served by `serve_static_assets()`.
/// The API routes (contact). Generic over state so the exact same routes
/// compose into the full app (`Router<FullstackState>`) and into the test
/// router (`Router<()>`). The handlers are stateless, so they attach to any
/// state type.
fn api_routes<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new().route("/api/contact", post(crate::handlers::contact::contact))
}

/// Test-only router: just the API routes, with NO static-asset serving or SSR.
///
/// `build_router()` can't be used in `cargo test` because `serve_static_assets()`
/// panics when it can't find the bundled `public/` dir (which only exists after
/// `dx bundle`, never under the test harness). Integration tests build this
/// instead so they exercise the real handlers without needing a bundle.
pub fn api_router() -> Router {
    api_routes::<()>()
}

pub fn build_router() -> Router {
    let state = FullstackState::new(ServeConfig::new(), App);

    // Explicit state type so DioxusRouterExt (impl'd on Router<FullstackState>) is reachable
    // before .with_state() consumes it.
    let router: Router<FullstackState> = api_routes::<FullstackState>()
        // /robots.txt at the root for crawlers.
        .route("/robots.txt", get(robots_txt))
        // Repo static files: CSS, images, sounds, the NYC photo.
        // Mounted at /static to leave /assets free for dx's serve_static_assets().
        .nest_service("/static", ServeDir::new("assets"))
        // Let dx own /assets (and any other subdirs it creates in public/).
        // Registers nest_service("/assets", ...) from public/assets/ at runtime.
        .serve_static_assets()
        // Register dioxus server functions (e.g. RSX server actions).
        .register_server_functions();

    // gzip/brotli compression on the response. The defaults compress every
    // text-like content-type (text/*, application/json, application/javascript,
    // application/wasm, application/xml) and skip already-compressed binaries
    // like mp3/png. Halves the wire size of our CSS + the WASM bundle.
    router
        .fallback(get(render_handler))
        .layer(CompressionLayer::new())
        .with_state(state)
}
