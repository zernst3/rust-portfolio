//! Integration tests for POST /api/pageview.
//!
//! PROC-REGRESSION-TEST-1: spins up the real Axum app via tower::ServiceExt::oneshot.

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

#[tokio::test]
async fn pageview_returns_no_content() -> anyhow::Result<()> {
    let app = server::build_router();
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/pageview")
                .body(Body::empty())?,
        )
        .await?;
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    Ok(())
}

#[tokio::test]
async fn pageview_ignores_body() -> anyhow::Result<()> {
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
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    Ok(())
}
