use thiserror::Error;

#[derive(Debug, Error)]
pub enum MailgunError {
    #[error("MAILGUN_API_KEY env var not set")]
    MissingApiKey,
    #[error("MAILGUN_DOMAIN env var not set")]
    MissingDomain,
    #[error("request transport failed: {0}")]
    Transport(#[from] reqwest::Error),
    #[error("Mailgun returned non-2xx: {0}")]
    NonSuccess(reqwest::StatusCode),
}

/// Read MAILGUN_API_KEY, MAILGUN_DOMAIN, and optionally MAILGUN_BASE_URL from env.
///
/// MAILGUN_BASE_URL defaults to "https://api.mailgun.net". Tests override it to
/// point at a local mock server so the full HTTP path is exercised without a
/// live Mailgun account (no new dep required; existing axum + reqwest suffice).
pub fn read_env() -> Result<(String, String, String), MailgunError> {
    let api_key = std::env::var("MAILGUN_API_KEY").map_err(|_| MailgunError::MissingApiKey)?;
    let domain = std::env::var("MAILGUN_DOMAIN").map_err(|_| MailgunError::MissingDomain)?;
    let base_url =
        std::env::var("MAILGUN_BASE_URL").unwrap_or_else(|_| "https://api.mailgun.net".to_string());
    Ok((api_key, domain, base_url))
}

/// One-shot Mailgun API send per PORT-EMAIL-1.
///
/// Builds a new reqwest::Client per call (handler runs once per submission;
/// connection-reuse savings don't justify shared-state plumbing).
/// `base_url` is normally "https://api.mailgun.net"; tests pass a local mock.
pub async fn send(
    api_key: &str,
    domain: &str,
    base_url: &str,
    params: &[(&str, &str)],
) -> Result<(), MailgunError> {
    let url = format!("{base_url}/v3/{domain}/messages");
    let resp = reqwest::Client::new()
        .post(&url)
        .basic_auth("api", Some(api_key))
        .form(params)
        .send()
        .await?;

    if resp.status().is_success() {
        Ok(())
    } else {
        Err(MailgunError::NonSuccess(resp.status()))
    }
}
