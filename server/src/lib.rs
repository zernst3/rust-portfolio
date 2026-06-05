use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};
use dioxus::server::{render_handler, FullstackState, ServeConfig};
use tower_http::services::ServeDir;

/// Build the Axum router: SSR fallback + API stubs + static assets.
///
/// Per PORT-MONOLITH-1: one binary, one process, one port.
/// SSR via `render_handler` fallback; Dioxus server-function registration deferred
/// until the first `#[server]` fn is added (none in scaffold phase).
pub fn build_router() -> Router {
    let state = FullstackState::new(ServeConfig::new(), ui::App);
    Router::new()
        .route("/api/contact", post(contact_stub))
        .route("/api/pageview", post(pageview_stub))
        .nest_service("/assets", ServeDir::new("assets"))
        .fallback(get(render_handler))
        .with_state(state)
}

async fn contact_stub() -> StatusCode {
    StatusCode::NO_CONTENT
}

async fn pageview_stub() -> StatusCode {
    StatusCode::NO_CONTENT
}
