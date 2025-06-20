use actix_web::web;

pub mod auth;

pub fn get_scope() -> actix_web::Scope {
    return web::scope("/api")
        .service(auth::get_scope())
}
