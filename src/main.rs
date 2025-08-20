mod api;
mod auth;
mod db;
mod pages;
mod security;

use api::post_verify;
use api::users::post_users;

use pages::main_page;

use actix_files as fs;
use actix_web::{
    middleware::Logger, web::{self, Data}, App, HttpServer
};
use argon2::Argon2;
use env_logger::Env;

const ADDRESS: &str = "127.0.0.1";
const PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    db::init("data.db").await;

    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db::get().clone()))
            .app_data(Data::new(Argon2::default()))
            .service(fs::Files::new("/assets", "./front/assets"))
            .service(main_page)
            .service(
                web::scope("/auth")
                    .service(auth::post_login)
                    .service(auth::post_refresh)
                    .service(auth::delete_logout),
            )
            .service(
                web::scope("/api/v1")
                    .service(post_users)
                    .service(post_verify),
            )
            // .service(api::get_scope())
            .wrap(Logger::default())
    })
    .bind((ADDRESS, PORT))?
    .run()
    .await
}
