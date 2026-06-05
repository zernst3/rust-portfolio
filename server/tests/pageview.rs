//! Integration tests for POST /api/pageview.
//!
//! PROC-REGRESSION-TEST-1: spins up the real Axum app via tower::ServiceExt::oneshot.

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

#[tokio::test]
async fn pageview_without_mailgun_returns_503() -> anyhow::Result<()> {
    // MAILGUN_API_KEY and MAILGUN_DOMAIN are not set in the test environment.
    // Handler returns 503. 204 requires live Mailgun config; tested in staging.
    let app = server::build_router();
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/pageview")
                .body(Body::empty())?,
        )
        .await?;
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    Ok(())
}

#[tokio::test]
async fn pageview_ignores_body() -> anyhow::Result<()> {
    // Body is ignored (PORT-API-2). Handler still returns 503 (no Mailgun env).
    let app = server::build_router();
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/pageview")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"ignored": true}"#))?,
        )
        .await?;
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    Ok(())
}
