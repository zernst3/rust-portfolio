//! Integration tests for POST /api/pageview.
//!
//! PROC-REGRESSION-TEST-1: spins up the real Axum app via tower::ServiceExt::oneshot.

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

// Serializes env-var mutations across concurrent tokio tests in this file.
static ENV_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

#[tokio::test]
async fn pageview_without_mailgun_returns_503() -> anyhow::Result<()> {
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
                .uri("/api/pageview")
                .body(Body::empty())?,
        )
        .await?;
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    Ok(())
}

#[tokio::test]
async fn pageview_ignores_body() -> anyhow::Result<()> {
    let _guard = ENV_LOCK.lock().await;
    std::env::remove_var("MAILGUN_API_KEY");
    std::env::remove_var("MAILGUN_DOMAIN");
    std::env::remove_var("MAILGUN_BASE_URL");
    // Body is ignored (PORT-API-2). Handler returns 503 without Mailgun config.
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
async fn pageview_with_mock_mailgun_succeeds() -> anyhow::Result<()> {
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
                .uri("/api/pageview")
                .body(Body::empty())?,
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
async fn pageview_when_mailgun_errors_returns_503() -> anyhow::Result<()> {
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
                .uri("/api/pageview")
                .body(Body::empty())?,
        )
        .await?;

    std::env::remove_var("MAILGUN_API_KEY");
    std::env::remove_var("MAILGUN_DOMAIN");
    std::env::remove_var("MAILGUN_BASE_URL");
    server_handle.abort();

    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    Ok(())
}
