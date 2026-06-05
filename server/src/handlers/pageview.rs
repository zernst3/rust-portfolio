use axum::http::StatusCode;

/// `POST /api/pageview` — returns 204 with no metadata captured.
///
/// Per PORT-API-2: handler signature carries NO extractors — no IP,
/// no user-agent, no headers, no path, no timestamp. Mailgun send
/// deferred to phase v0.1-backend.
pub async fn pageview() -> StatusCode {
    tracing::info!("pageview ping received");
    StatusCode::NO_CONTENT
}
