use actix_web::{HttpRequest, HttpResponse, Responder, body::BoxBody};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    tag: String,
    username: String,
    avatar_url: Option<String>,
    banner_url: Option<String>,
    banned: bool,
    role: String,
    is_legend: bool,
    created_at: u64,
    last_login_at: u64,
}

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
