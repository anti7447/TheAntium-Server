use crate::security::jwt::{TokenType, decode_token, encode_token};
use chrono::{DateTime, Utc};

use crate::repos::session::SessionRepo;
use chrono::Duration;
use jsonwebtoken::errors::ErrorKind as JwtErrorKind;
use sqlx::Error as DbError;

pub struct SessionService {
    pub repo: SessionRepo,
}

pub struct SessionCreate {
    user_id: u32,
    device_name: String,
    expires_at: DateTime<Utc>,
}

pub enum SessionServiceError {
    DbInternal(DbError),
    TokenEncoding(JwtErrorKind),
    InvalidToken,
    Expired,
    NotFound,
}

impl SessionService {
    pub fn new(repo: SessionRepo) -> Self {
        Self { repo }
    }

    pub async fn create_tokens(
        &self,
        user_id: u32,
        device_name: String,
    ) -> Result<(String, String), SessionServiceError> {
        let expires_at = Utc::now() + Duration::weeks(4);
        let session_id = self
            .repo
            .create(user_id, &device_name, &expires_at)
            .await
            .map_err(SessionServiceError::DbInternal)?;

        let refresh = encode_token(session_id, TokenType::Refresh, Duration::weeks(4))
            .map_err(|e| SessionServiceError::TokenEncoding(e.into_kind()))?;
        let access = encode_token(session_id, TokenType::Access, Duration::minutes(10))
            .map_err(|e| SessionServiceError::TokenEncoding(e.into_kind()))?;

        Ok((access, refresh))
    }

    pub async fn refresh_access(&self, refresh_token: &str) -> Result<String, SessionServiceError> {
        let token = decode_token(refresh_token).map_err(|_| SessionServiceError::InvalidToken)?;
        if token.kind != "refresh" {
            return Err(SessionServiceError::InvalidToken);
        }

        let session_id =
            str::parse::<u32>(&token.sub).map_err(|_| SessionServiceError::InvalidToken)?;
        let session = self
            .repo
            .get_by_id(session_id)
            .await
            .map_err(SessionServiceError::DbInternal)?
            .ok_or(SessionServiceError::NotFound)?;

        if session.expires_at < Utc::now() {
            return Err(SessionServiceError::Expired);
        }

        Ok(
            encode_token(session.user_id, TokenType::Access, Duration::minutes(10))
                .map_err(|e| SessionServiceError::TokenEncoding(e.into_kind()))?,
        )
    }

    pub async fn logout(&self, refresh_token: &str) -> Result<(), SessionServiceError> {
        let token = decode_token(refresh_token).map_err(|_| SessionServiceError::InvalidToken)?;
        if token.kind != "refresh" {
            return Err(SessionServiceError::InvalidToken);
        }
        let session_id =
            str::parse::<u32>(&token.sub).map_err(|_| SessionServiceError::InvalidToken)?;
        self.repo
            .delete(session_id)
            .await
            .map_err(SessionServiceError::DbInternal)
    }
}
