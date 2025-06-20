use actix_web::{web, App, HttpServer};
use sqlx::{Sqlite, SqlitePool};

mod db;
mod api;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    db::init("data.db").await;
    sqlx::query(include_str!("db/sql/create.sql"))
        .execute(db::get())
        .await
        .expect("Failed to execute creation SQL");
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(db::get().clone()))
            .service(api::get_scope())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}