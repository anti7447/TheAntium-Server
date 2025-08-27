use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize)]
pub struct UserCreateRequest {
    pub tag: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct UserFull {
    pub id: u32,
    pub tag: String,
    pub username: String,
    pub avatar_url: Option<String>,
    pub banner_url: Option<String>,
    pub password_hash: String,
    pub telegram_id: Option<u64>,
    pub banned: bool,
    pub role: String,
    pub is_legend: bool,
    pub permissions: u8,
    pub privacy: u8,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct UserPublicView {
    pub id: u32,
    pub tag: String,
    pub username: String,
    pub avatar_url: Option<String>,
    pub banner_url: Option<String>,
    pub telegram_id: Option<u64>,
    pub banned: bool,
    pub role: String,
    pub is_legend: bool,
    pub created_at: DateTime<Utc>,
    pub last_seen: Option<DateTime<Utc>>,
}

impl From<UserFull> for UserPublicView {
    fn from(user: UserFull) -> Self {
        UserPublicView {
            id: user.id,
            tag: user.tag,
            username: user.username,
            avatar_url: user.avatar_url,
            banner_url: user.banner_url,
            telegram_id: None,
            banned: user.banned,
            role: user.role,
            is_legend: user.is_legend,
            created_at: user.created_at,
            last_seen: None,
        }
    }
}

#[derive(Deserialize)]
pub struct UserLoginRequest {
    pub tag: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserLoginResponse {
    pub user: UserPublicView,
    pub access: String,
    pub refresh: String,
}
