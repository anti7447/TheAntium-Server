use actix_web::{HttpResponse, Responder, get, web};

#[get("/")]
pub async fn main_page() -> impl Responder {
    HttpResponse::Ok().body(include_str!("../front/main.html"))
}
