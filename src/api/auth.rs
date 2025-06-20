use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use rand::{self, Rng};
use argon2::{self, password_hash::{rand_core::OsRng, SaltString}};
use argon2::PasswordHasher;

#[derive(Deserialize)]
pub struct UserCreateRequest {
    username: String,
    display_name: String,
    password: String
}


#[derive(Serialize)]
pub struct UserCreateResponse {
    username: String,
    display_name: String,
    token: String
}
 
pub fn get_scope() -> actix_web::Scope {
    return web::scope("/auth")
        .route("/register", web::post().to(register))
}


pub async fn register(data: web::Data<SqlitePool>, create: web::Json<UserCreateRequest>) -> impl Responder {
    let UserCreateRequest {
        username,
        display_name,
        password,
    } = create.into_inner();

    let token: String = rand::rng().sample_iter(&rand::distr::Alphanumeric).take(64).map(char::from).collect();
    let passhash: String = argon2::Argon2::default().hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng)).expect("damn").to_string();
    
    let res = sqlx::query(include_str!("../db/sql/user/create.sql"))
        .bind(&username)
        .bind(&display_name) 
        .bind(&passhash)
        .bind(&token)
        .execute(data.get_ref())
        .await;

    match res {
        Err(e) => {
            match e.as_database_error() {
                Some(error) => {
                    if error.is_unique_violation() {
                        HttpResponse::Conflict().body("User already exists")
                    } else {
                        println!("{e} {error}");
                        HttpResponse::InternalServerError().body("DB error")
                    }
                },
                None => {
                    println!("{e}");
                    HttpResponse::InternalServerError().body("Unknown DB error")
                }
            }
        }
        Ok(_) => HttpResponse::Created().json(UserCreateResponse {
            username: username,
            display_name: display_name,
            token: token
        })
    }
}