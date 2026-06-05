//! Integration tests for POST /api/contact.
//!
//! PROC-REGRESSION-TEST-1: spins up the real Axum app via tower::ServiceExt::oneshot.

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

// Serializes env-var mutations across concurrent tokio tests in this file.
static ENV_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

fn valid_contact_body() -> Body {
    Body::from(
        r#"{"name":"Test User","email":"test@example.com","subject":"Hello","message":"World"}"#,
    )
}

#[tokio::test]
async fn contact_valid_payload_without_mailgun_returns_503() -> anyhow::Result<()> {
    // Hold ENV_LOCK so concurrent tests cannot set MAILGUN_API_KEY while we run.
    let _guard = ENV_LOCK.lock().await;
    std::env::remove_var("MAILGUN_API_KEY");
    std::env::remove_var("MAILGUN_DOMAIN");
    std::env::remove_var("MAILGUN_BASE_URL");
    // With no Mailgun config the handler returns 503. 204 requires staging creds.
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
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
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

/// Spin up a local Axum mock that responds to all Mailgun messages POSTs with
/// `mock_status`, return its base URL (e.g. "http://127.0.0.1:PORT").
async fn start_mock_mailgun(
    mock_status: StatusCode,
) -> anyhow::Result<(String, tokio::task::JoinHandle<()>)> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;
    let mock = axum::Router::new().route(
        "/v3/{domain}/messages",
        axum::routing::post(move || async move { mock_status }),
    );
    let handle = tokio::spawn(async move {
        axum::serve(listener, mock).await.ok();
    });
    Ok((format!("http://{addr}"), handle))
}

#[tokio::test]
async fn contact_with_mock_mailgun_succeeds() -> anyhow::Result<()> {
    let _guard = ENV_LOCK.lock().await;
    let (base_url, server_handle) = start_mock_mailgun(StatusCode::OK).await?;

    std::env::set_var("MAILGUN_API_KEY", "test-key");
    std::env::set_var("MAILGUN_DOMAIN", "test.mailgun.org");
    std::env::set_var("MAILGUN_BASE_URL", &base_url);

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

    std::env::remove_var("MAILGUN_API_KEY");
    std::env::remove_var("MAILGUN_DOMAIN");
    std::env::remove_var("MAILGUN_BASE_URL");
    server_handle.abort();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    Ok(())
}

#[tokio::test]
async fn contact_when_mailgun_errors_returns_503() -> anyhow::Result<()> {
    let _guard = ENV_LOCK.lock().await;
    let (base_url, server_handle) = start_mock_mailgun(StatusCode::INTERNAL_SERVER_ERROR).await?;

    std::env::set_var("MAILGUN_API_KEY", "test-key");
    std::env::set_var("MAILGUN_DOMAIN", "test.mailgun.org");
    std::env::set_var("MAILGUN_BASE_URL", &base_url);

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

    std::env::remove_var("MAILGUN_API_KEY");
    std::env::remove_var("MAILGUN_DOMAIN");
    std::env::remove_var("MAILGUN_BASE_URL");
    server_handle.abort();

    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    Ok(())
}
