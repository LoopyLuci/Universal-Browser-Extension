//! JWT minting and verification.

use std::time::{Duration, SystemTime};

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    UiRead, UiConsent, UiModules, UiSettings, UiAudit, UiAdmin,
}

impl Scope {
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::UiRead => "ui.read",
            Self::UiConsent => "ui.consent",
            Self::UiModules => "ui.modules",
            Self::UiSettings => "ui.settings",
            Self::UiAudit => "ui.audit",
            Self::UiAdmin => "ui.admin",
        }
    }

    #[must_use]
    pub fn companion_set() -> Vec<Self> {
        vec![Self::UiRead, Self::UiConsent, Self::UiModules, Self::UiSettings, Self::UiAudit]
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iat: u64,
    pub exp: u64,
    pub jti: Uuid,
    pub scopes: Vec<Scope>,
    pub session: String,
}

pub fn mint_token(
    secret: &[u8],
    subject: &str,
    session: &str,
    scopes: Vec<Scope>,
    ttl: Duration,
) -> anyhow::Result<String> {
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs();
    let claims = Claims {
        sub: subject.to_string(),
        iat: now,
        exp: now + ttl.as_secs(),
        jti: Uuid::now_v7(),
        scopes,
        session: session.to_string(),
    };
    let header = Header::new(Algorithm::HS256);
    Ok(encode(&header, &claims, &EncodingKey::from_secret(secret))?)
}

pub fn verify_token(secret: &[u8], token: &str) -> anyhow::Result<Claims> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;
    validation.leeway = 5;
    let data = decode::<Claims>(token, &DecodingKey::from_secret(secret), &validation)?;
    Ok(data.claims)
}