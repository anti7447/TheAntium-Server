use chrono::{DateTime, Timelike, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::Error};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

pub fn get_token(sub: String, now: DateTime<Utc>, exp: DateTime<Utc>) -> Result<String, Error> {
    // let now = SystemTime::now()
    //     .duration_since(UNIX_EPOCH)
    //     .unwrap()
    //     .as_secs();
    // let exp = now + secs; // 15 минут

    let claims = Claims {
        sub,
        iat: now.timestamp() as usize,
        exp: exp.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(include_bytes!("../../secret.pem")),
    )
}

pub fn verify_token(token: String) -> Result<Claims, Error> {
    // `token` is a struct with 2 fields: `header` and `claims` where `claims` is your own struct.
    let token = decode::<Claims>(
        &token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    )?;

    Ok(token.claims)
}
