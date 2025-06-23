pub mod auth;
pub mod types;
pub mod users;

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

// pub fn get_scope() -> actix_web::Scope {
//     return web::scope("/api").service(auth::get_scope());
// }
