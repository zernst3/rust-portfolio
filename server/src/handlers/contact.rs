use axum::{extract::Json, http::StatusCode};

use crate::dto::ContactRequest;

/// `POST /api/contact` — validates DTO, returns 204.
///
/// Mailgun send deferred to phase v0.1-backend per PORT-API-1.
/// Per PORT-API-2 (scope guard): this handler does NOT capture requester IP,
/// user-agent, or any metadata beyond what the visitor explicitly submitted.
pub async fn contact(Json(body): Json<ContactRequest>) -> StatusCode {
    tracing::info!(name = %body.name, email = %body.email.as_str(), "contact form submitted");
    StatusCode::NO_CONTENT
}
