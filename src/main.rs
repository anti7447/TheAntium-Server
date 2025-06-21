mod api;
mod db;

use actix_web::{middleware::Logger, web::{self, Data}, App, HttpServer};
use api::post_verify;
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
            .service(
                web::scope("/api/v1")
                    .service(post_verify)
            )
            .service(api::get_scope())
            .wrap(Logger::default())
    })
    .bind((ADDRESS, PORT))?
    .run()
    .await
}
