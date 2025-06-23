use core::str;

use actix_web::{HttpRequest, HttpResponse, Responder, body::BoxBody};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserFull {
    tag: String,
    username: String,
    avatar_url: Option<String>,
    banner_url: Option<String>,
    password_hash: Option<String>,
    salt: Option<String>,
    token: String,
    telegram_id: Option<u64>,
    banned: bool,
    role: String,
    created_at: u64,
    updated_at: u64,
}

#[derive(Serialize)]
pub struct UserPublicView {
    tag: String,
    username: String,
    avatar_url: Option<String>,
    banner_url: Option<String>,
    banned: bool,
    role: String,
    created_at: u64,
}

impl From<UserFull> for UserPublicView {
    fn from(user: UserFull) -> Self {
        UserPublicView {
            tag: user.tag,
            username: user.username,
            avatar_url: user.avatar_url,
            banner_url: user.banner_url,
            banned: user.banned,
            role: user.role,
            created_at: user.created_at,
        }
    }
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
