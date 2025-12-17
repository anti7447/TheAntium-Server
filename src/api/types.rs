use core::str;

use actix_web::{HttpRequest, HttpResponse, Responder, body::BoxBody};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

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
pub struct Post {}

#[derive(Serialize)]
pub struct Comment {}

// impl Responder for User {
//     type Body = BoxBody;

//     fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
//         todo!()
//     }
// }

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Session {
    pub id: u32,
    pub user_id: u32,
    pub device_name: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub is_valid: bool,
}