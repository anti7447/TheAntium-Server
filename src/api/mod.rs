pub mod types;

use actix_web::{HttpResponse, Responder, post, web};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Info {
    user_id: u64,
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
