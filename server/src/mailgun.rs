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

/// Read MAILGUN_API_KEY and MAILGUN_DOMAIN from the environment.
pub fn read_env() -> Result<(String, String), MailgunError> {
    let api_key = std::env::var("MAILGUN_API_KEY").map_err(|_| MailgunError::MissingApiKey)?;
    let domain = std::env::var("MAILGUN_DOMAIN").map_err(|_| MailgunError::MissingDomain)?;
    Ok((api_key, domain))
}

/// One-shot Mailgun API send per PORT-EMAIL-1.
///
/// Builds a new reqwest::Client per call (handler runs once per submission;
/// connection-reuse savings don't justify shared-state plumbing).
pub async fn send(
    api_key: &str,
    domain: &str,
    params: &[(&str, &str)],
) -> Result<(), MailgunError> {
    let url = format!("https://api.mailgun.net/v3/{domain}/messages");
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
