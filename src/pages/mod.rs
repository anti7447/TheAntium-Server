use actix_web::{HttpResponse, Responder, web};

pub async fn main_page() -> impl Responder {
    HttpResponse::Ok().body(include_str!("../../front/main.html"))
}

pub async fn user_page() -> impl Responder {
    // TODO: Complete handler
    HttpResponse::Ok().finish()
}

pub fn get_scope() -> actix_web::Scope {
    web::scope("/")
        .service(web::resource("/").to(main_page))
        .service(web::resource("/user").to(user_page))
}
