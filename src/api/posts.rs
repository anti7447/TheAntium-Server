use actix_web::{HttpResponse, Responder, post, web};
use serde::Deserialize;

use crate::db::{self, Pool};

#[derive(Deserialize)]
pub struct PostDataRequest {
    name: String,
    content: String,
    token: String,
}

#[post("/posts")]
pub async fn post_users(
    pool: web::Data<Pool>,
    post_data: web::Json<PostDataRequest>,
) -> impl Responder {
    HttpResponse::Created().body("Change this message to respond, please")
}
