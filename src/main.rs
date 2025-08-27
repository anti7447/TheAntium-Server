mod api;
mod db;
mod pages;
mod repos;
mod security;
mod services;
mod types;

use actix_files as fs;
use actix_web::{App, HttpServer, middleware::Logger, web::Data};
use argon2::Argon2;
use env_logger::Env;

use crate::{
    repos::{session::SessionRepo, user::UserRepo},
    services::{session::SessionService, user::UserService},
};

const ADDRESS: &str = "127.0.0.1";
const PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    db::init("data.db").await;

    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(UserService::new(UserRepo::new(
                db::get().clone(),
            ))))
            .app_data(Data::new(SessionService::new(SessionRepo::new(
                db::get().clone(),
            ))))
            .app_data(Data::new(db::get().clone()))
            .app_data(Data::new(Argon2::default()))
            .service(fs::Files::new("/assets", "./front/assets"))
            .service(api::get_scope())
            .service(pages::get_scope())
            .wrap(Logger::default())
    })
    .bind((ADDRESS, PORT))?
    .run()
    .await
}
