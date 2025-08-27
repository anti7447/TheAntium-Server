use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{Error as HashingError, SaltString, rand_core::OsRng},
};

use sqlx::Error as DbError;

use crate::{repos::user::UserRepo, types::user::UserFull};
use crate::{services::session::SessionService, types::user::UserLoginRequest};
use crate::{services::session::SessionServiceError, types::user::UserCreateRequest};

pub struct UserService {
    pub repo: UserRepo,
}

pub enum UserServiceError {
    DbInternal(DbError),
    Hashing(HashingError),
    RowNotFound,
    UniqueViolation,
    Validation(String),
    InvalidCredentials,
    TokenError,
}

impl UserService {
    pub fn new(repo: UserRepo) -> Self {
        Self { repo }
    }

    pub async fn register(&self, create: UserCreateRequest) -> Result<u32, UserServiceError> {
        for ch in create.tag.chars() {
            if !ch.is_ascii_alphanumeric() && ch != '_' {
                return Err(UserServiceError::Validation(format!(
                    "Tag contains non-alphanumeric char {}",
                    ch
                )));
            }
        }
        if create.password.len() < 4 {
            return Err(UserServiceError::Validation(
                "Password too short, must be at least 4 characters long".to_string(),
            ));
        }
        match self.get_by_tag(create.tag.clone()).await {
            Ok(user) => {
                if user.is_some() {
                    return Err(UserServiceError::UniqueViolation);
                }
            }
            Err(err) => return Err(err),
        }
        match Argon2::default().hash_password(
            create.password.as_bytes(),
            &SaltString::generate(&mut OsRng),
        ) {
            Ok(hash) => match self
                .repo
                .create(create.tag, create.username, hash.to_string())
                .await
            {
                Ok(id) => Ok(id),
                Err(err) => match err {
                    DbError::Database(dbe) => match dbe.kind() {
                        sqlx::error::ErrorKind::UniqueViolation => {
                            Err(UserServiceError::UniqueViolation)
                        }
                        _ => Err(UserServiceError::RowNotFound),
                    },
                    _ => Err(UserServiceError::DbInternal(err)),
                },
            },
            Err(err) => Err(UserServiceError::Hashing(err)),
        }
    }

    pub async fn get_by_tag(&self, tag: String) -> Result<Option<UserFull>, UserServiceError> {
        match self.repo.get_by_tag(tag).await {
            Ok(user) => Ok(user),
            Err(err) => Err(UserServiceError::DbInternal(err)),
        }
    }

    pub async fn get_by_id(&self, id: u32) -> Result<Option<UserFull>, UserServiceError> {
        match self.repo.get_by_id(id).await {
            Ok(user) => Ok(user),
            Err(err) => Err(UserServiceError::DbInternal(err)),
        }
    }

    /// Creates access and refresh tokens and a session entry.
    /// # Returns
    /// (user, access, refresh)
    pub async fn login(
        &self,
        session_service: &SessionService,
        login: UserLoginRequest,
        device_name: String,
    ) -> Result<(UserFull, String, String), UserServiceError> {
        let user = self
            .get_by_tag(login.tag.clone())
            .await?
            .ok_or(UserServiceError::RowNotFound)?;

        let parsed_hash =
            PasswordHash::new(&user.password_hash).map_err(UserServiceError::Hashing)?;
        Argon2::default()
            .verify_password(login.password.as_bytes(), &parsed_hash)
            .map_err(|_| UserServiceError::InvalidCredentials)?;

        let (access, refresh) = session_service
            .create_tokens(user.id, device_name)
            .await
            .map_err(|e| match e {
                SessionServiceError::DbInternal(dbe) => UserServiceError::DbInternal(dbe),
                _ => UserServiceError::TokenError,
            })?;

        let _ = self.repo.update_last_seen(user.id).await;

        Ok((user, access, refresh))
    }
}
