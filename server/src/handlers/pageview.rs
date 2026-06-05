use axum::http::StatusCode;

use crate::mailgun;

/// `POST /api/pageview` — sends a single fixed-text email to Zach, returns 204.
///
/// Per PORT-API-1: separate route, no action discriminator.
/// Per PORT-API-2: handler captures ZERO metadata (no IP, no UA, no path).
///   Email body is the fixed string "Someone visited your site." — nothing more.
/// Per PORT-EMAIL-1: one reqwest::Client::post(); 5xx / timeout → 503.
pub async fn pageview() -> StatusCode {
    tracing::info!("pageview ping received");

    let (api_key, domain, base_url) = match mailgun::read_env() {
        Ok(trio) => trio,
        Err(e) => {
            tracing::error!(error = %e, "mailgun env not configured");
            return StatusCode::SERVICE_UNAVAILABLE;
        }
    };

    let from = format!("Website Notification <no-reply@{domain}>");
    let params: &[(&str, &str)] = &[
        ("from", &from),
        ("to", "Zachary Ernst <zernst3@live.com>"),
        ("subject", "Someone visited your site."),
        ("text", "Someone visited your site."),
    ];

    if let Err(e) = mailgun::send(&api_key, &domain, &base_url, params).await {
        tracing::error!(error = %e, "pageview email failed");
        return StatusCode::SERVICE_UNAVAILABLE;
    }

    StatusCode::NO_CONTENT
}
