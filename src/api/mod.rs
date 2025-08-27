pub mod auth;

use actix_web::{HttpResponse, Responder, post, web};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
    message: String,
    status: u16,
}

#[derive(Deserialize, Serialize)]
pub struct Info {
    user_id: u32,
}

/// Method: POST
///
/// Path: /api/v1/verify
///
/// (!) WIP (!)
#[post("/verify")]
pub async fn post_verify(info: web::Json<Info>) -> impl Responder {
    HttpResponse::Accepted().json(info)
}

pub fn get_scope() -> actix_web::Scope {
    web::scope("/api/v1").service(auth::get_scope())
}
