use serde::Deserialize;
use thiserror::Error;

/// Contact-form payload per PORT-API-1.
#[derive(Debug, Deserialize)]
pub struct ContactRequest {
    pub name: String,
    #[serde(deserialize_with = "deserialize_email")]
    pub email: EmailAddress,
    pub subject: String,
    pub message: String,
}

/// Newtype wrapper that enforces basic email structure at deserialization time.
///
/// Validates: one `@`, non-empty local part, domain contains `.` with
/// non-empty parts on each side. Mailgun does the authoritative check.
#[derive(Debug)]
pub struct EmailAddress(String);

impl EmailAddress {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn deserialize_email<'de, D>(deserializer: D) -> Result<EmailAddress, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    EmailAddress::try_from(s).map_err(serde::de::Error::custom)
}

#[derive(Debug, Error)]
#[error("invalid email address: {0}")]
pub struct InvalidEmail(String);

impl TryFrom<String> for EmailAddress {
    type Error = InvalidEmail;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let (local, domain) = s.split_once('@').ok_or_else(|| InvalidEmail(s.clone()))?;
        if local.is_empty() {
            return Err(InvalidEmail(s));
        }
        let dot_pos = domain.find('.').ok_or_else(|| InvalidEmail(s.clone()))?;
        if dot_pos == 0 || dot_pos == domain.len() - 1 {
            return Err(InvalidEmail(s));
        }
        Ok(EmailAddress(s))
    }
}
