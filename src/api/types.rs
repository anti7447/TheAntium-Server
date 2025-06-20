use actix_web::{HttpRequest, HttpResponse, Responder, body::BoxBody};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub tag: String,
    pub username: Option<String>,
    pub avatar_url: Option<String>,
    pub banner_url: Option<String>,
    pub password_hash: Option<String>,
    pub salt: Option<String>,
    pub token: String,
    pub telegram_id: Option<u64>,
    pub banned: bool,
    pub role: String,
    pub is_legend: bool,
    pub created_at: u64,
    pub updated_at: u64,
}

// #[derive(Serialize, Deserialize)]
// pub struct User {
//     tag: String,
//     username: String,
//     avatar_url: Option<String>,
//     banner_url: Option<String>,
//     banned: bool,
//     role: String,
//     is_legend: bool,
//     created_at: u64,
//     last_login_at: u64,
// }

#[derive(Serialize)]
pub struct Post {}

#[derive(Serialize)]
pub struct Comment {}

impl Responder for User {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        todo!()
    }
}
