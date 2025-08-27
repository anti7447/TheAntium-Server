use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub kind: String,
    pub exp: usize,
    pub iat: usize,
}

pub enum TokenType {
    Refresh,
    Access,
}

pub fn encode_token(sub: u32, kind: TokenType, lifetime: Duration) -> Result<String, Error> {
    let claims = Claims {
        sub: sub.to_string(),
        kind: match kind {
            TokenType::Access => "access".to_string(),
            TokenType::Refresh => "refresh".to_string(),
        },
        iat: Utc::now().timestamp() as usize,
        exp: (Utc::now() + lifetime).timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(include_bytes!("../../secret.pem")),
    )
}

pub fn decode_token(token: &str) -> Result<Claims, Error> {
    let token = decode::<Claims>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    )?;

    Ok(token.claims)
}
