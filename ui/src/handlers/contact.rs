use axum::{extract::Json, http::StatusCode};

use crate::dto::ContactRequest;
use crate::mailgun;

/// `POST /api/contact` — validates DTO, fires two Mailgun emails, returns 204.
///
/// Per PORT-API-1: separate route per operation (no action discriminator).
/// Per PORT-EMAIL-1: one reqwest::Client::post() per email; 5xx / timeout → 503.
/// Per PORT-API-2 (scope guard): no IP, user-agent, or metadata captured.
pub async fn contact(Json(body): Json<ContactRequest>) -> StatusCode {
    tracing::info!(name = %body.name, email = %body.email.as_str(), "contact form submitted");

    let (api_key, domain, base_url) = match mailgun::read_env() {
        Ok(trio) => trio,
        Err(e) => {
            tracing::error!(error = %e, "mailgun env not configured");
            return StatusCode::SERVICE_UNAVAILABLE;
        }
    };

    let email = body.email.as_str();

    // `From` MUST be on the Mailgun sending domain so SPF + DKIM align with the
    // From header (DMARC alignment). Sending "from" a @live.com address, or the
    // visitor's own address, THROUGH Mailgun fails DMARC at the recipient
    // (Outlook/Gmail), which accepts the message (SMTP 250) and then silently
    // drops it — delivered in Mailgun's log, but never in the inbox. The human
    // reply target goes in Reply-To instead.
    let from_addr = format!("Zachary Ernst <no-reply@{domain}>");
    let owner = "Zachary Ernst <zernst3@live.com>";

    let vars_thank_you = format!(
        r#"{{"recipient_name": "{}"}}"#,
        body.name.replace('"', "\\\"")
    );
    let vars_notify = format!(
        r#"{{"recipient_name": "{}", "text": "{}", "recipient_email": "{}"}}"#,
        body.name.replace('"', "\\\""),
        body.message.replace('"', "\\\"").replace('\n', "\\n"),
        email.replace('"', "\\\""),
    );

    // Thank-you to the visitor; replies route back to Zach.
    let thank_you_params: &[(&str, &str)] = &[
        ("from", from_addr.as_str()),
        ("to", email),
        ("h:Reply-To", "zernst3@live.com"),
        ("subject", "Thank You for your Email"),
        ("template", "thank_you_email"),
        ("h:X-Mailgun-Variables", &vars_thank_you),
    ];
    if let Err(e) = mailgun::send(&api_key, &domain, &base_url, thank_you_params).await {
        tracing::error!(error = %e, "thank-you email failed");
        return StatusCode::SERVICE_UNAVAILABLE;
    }

    // Notification to Zach; replies route back to the visitor.
    let notify_params: &[(&str, &str)] = &[
        ("from", from_addr.as_str()),
        ("to", owner),
        ("h:Reply-To", email),
        ("subject", &body.subject),
        ("template", "email_to_me"),
        ("h:X-Mailgun-Variables", &vars_notify),
    ];
    if let Err(e) = mailgun::send(&api_key, &domain, &base_url, notify_params).await {
        tracing::error!(error = %e, "notification email failed");
        return StatusCode::SERVICE_UNAVAILABLE;
    }

    tracing::info!("contact emails sent");
    StatusCode::NO_CONTENT
}
