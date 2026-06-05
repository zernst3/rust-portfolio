use axum::{
    routing::{get, post},
    Router,
};
use dioxus::server::{render_handler, FullstackState, ServeConfig};
use tower_http::services::ServeDir;

pub mod dto;
pub mod handlers;

/// Build the Axum router: SSR fallback + validated API handlers + static assets.
///
/// Per PORT-MONOLITH-1: one binary, one process, one port.
pub fn build_router() -> Router {
    let state = FullstackState::new(ServeConfig::new(), ui::App);
    Router::new()
        .route("/api/contact", post(handlers::contact::contact))
        .route("/api/pageview", post(handlers::pageview::pageview))
        .nest_service("/assets", ServeDir::new("assets"))
        .fallback(get(render_handler))
        .with_state(state)
}
