//! Integration tests for POST /api/contact.
//!
//! PROC-REGRESSION-TEST-1: spins up the real Axum app via tower::ServiceExt::oneshot.

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

fn valid_contact_body() -> Body {
    Body::from(
        r#"{"name":"Test User","email":"test@example.com","subject":"Hello","message":"World"}"#,
    )
}

#[tokio::test]
async fn contact_valid_payload_returns_no_content() -> anyhow::Result<()> {
    let app = server::build_router();
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/contact")
                .header("content-type", "application/json")
                .body(valid_contact_body())?,
        )
        .await?;
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    Ok(())
}

#[tokio::test]
async fn contact_invalid_email_returns_unprocessable() -> anyhow::Result<()> {
    let app = server::build_router();
    let body = Body::from(r#"{"name":"X","email":"not-an-email","subject":"S","message":"M"}"#);
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/contact")
                .header("content-type", "application/json")
                .body(body)?,
        )
        .await?;
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    Ok(())
}

#[tokio::test]
async fn contact_missing_field_returns_unprocessable() -> anyhow::Result<()> {
    let app = server::build_router();
    let body = Body::from(r#"{"name":"X","email":"x@example.com","subject":"S"}"#);
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/contact")
                .header("content-type", "application/json")
                .body(body)?,
        )
        .await?;
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    Ok(())
}
