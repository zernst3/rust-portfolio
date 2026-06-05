//! Portfolio site monolith.
//!
//! Single-binary axum + Dioxus Fullstack server. Serves SSR HTML for all
//! routes, plus the two contact-form endpoints (`POST /api/contact`,
//! `POST /api/pageview`), plus static asset serving from `assets/`.
//!
//! Bot scaffolds the full router shape per the v0.1 work-queue. This file
//! is a placeholder so `cargo check` passes on the empty workspace.

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("rust-portfolio server: scaffold placeholder. bot replaces during v0.1.");
}
