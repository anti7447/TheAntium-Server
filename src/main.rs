mod api;
mod database;

use actix_web::{App, HttpResponse, HttpServer, Responder, error::ErrorNotFound, web};
use api::post_verify;
use database::init_db;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::{
    collections::HashMap,
    sync::{Mutex, atomic::AtomicU64},
};

const ADDRESS: &str = "127.0.0.1";
const PORT: u16 = 8080;

type UserDatabase = Mutex<HashMap<u64, User>>;

// #[derive(Serialize, Deserialize)]
struct AntiumState {
    counter: AtomicU64,
    pool: SqlitePool,
}

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
}

#[derive(Serialize)]
struct CreateUserResponse {
    id: u64,
    name: String,
}

#[derive(Serialize)]
struct AntiumStateResponse {
    counter_count: u64,
}

// #[actix_web::get("/")]
// async fn index(state: web::Data<AntiumState>) -> String {
//     let mut counter = state.counter.lock().unwrap();
//     *counter += 1;
//     format!("Request number: {counter}")
// }

// #[actix_web::post("/users/create")]
// async fn create_user(
//     state: web::Data<AntiumState>,
//     user_data: web::Json<User>,
//     db: web::Data<UserDatabase>,
// ) -> impl Responder {
//     let mut counter = state.counter.lock().unwrap();
//     *counter += 1;

//     let mut db = db.lock().unwrap();
//     let new_id = db.keys().max().unwrap_or(&0) + 1;
//     let name = user_data.name.clone();
//     db.insert(new_id, user_data.into_inner());
//     HttpResponse::Created().json(CreateUserResponse { id: new_id, name })
// }

// #[actix_web::get("/state")]
// async fn get_antium_state(state: web::Data<AntiumState>) -> impl Responder {
//     let mut counter = state.counter.lock().unwrap();
//     *counter += 1;
//     HttpResponse::Ok().json(AntiumStateResponse {
//         counter_count: *counter,
//     })
// }

// #[actix_web::get("/users/{id}")]
// async fn get_user(
//     state: web::Data<AntiumState>,
//     user_id: web::Path<u64>,
//     db: web::Data<UserDatabase>,
// ) -> Result<impl Responder, actix_web::Error> {
//     let mut counter = state.counter.lock().unwrap();
//     *counter += 1;

//     let user_id = user_id.into_inner();
//     let db = db.lock().unwrap();

//     match db.get(&user_id) {
//         Some(user_data) => Ok(HttpResponse::Ok().json(user_data)),
//         None => Err(ErrorNotFound("User not found!")),
//     }
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let user_db = web::Data::new(Mutex::new(HashMap::<u64, User>::new()));
    let options = SqliteConnectOptions::new()
        .filename("database.db")
        .create_if_missing(true);
    let pool = SqlitePool::connect_with(options).await.unwrap();

    init_db(&pool).await;

    let state = web::Data::new(AntiumState {
        counter: AtomicU64::new(0),
        pool: pool,
    });
    HttpServer::new(move || {
        App::new()
            // .app_data(user_db.clone())
            .app_data(state.clone())
            .service(
                web::scope("/api/v1")
                    // .service(web::scope("/users"))
                    .service(post_verify), // .route("/verify", web::post().to(api::post_verify)),
            )
        // .service(index)
        // .service(create_user)
        // .service(get_antium_state)
        // .service(get_user)
    })
    .bind((ADDRESS, PORT))?
    .run()
    .await
}
