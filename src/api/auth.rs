use actix_web::{HttpResponse, Responder, web};
use argon2::PasswordHasher;
use argon2::{
    self,
    password_hash::{SaltString, rand_core::OsRng},
};
use rand::{self, Rng};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::AntiumState;
use crate::db::create_user;

#[derive(Deserialize)]
pub struct UserCreateRequest {
    username: String,
    display_name: String,
    password: String,
}

#[derive(Serialize)]
pub struct UserCreateResponse {
    username: String,
    display_name: String,
    token: String,
}

pub fn get_scope() -> actix_web::Scope {
    return web::scope("/auth").route("/register", web::post().to(register));
}

pub async fn register(
    data: web::Data<AntiumState>,
    create: web::Json<UserCreateRequest>,
) -> impl Responder {
    let UserCreateRequest {
        username,
        display_name,
        password,
    } = create.into_inner();

    let pool = &data.pool;

    let token: String = rand::rng()
        .sample_iter(&rand::distr::Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();
    let passhash: String = argon2::Argon2::default()
        .hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng))
        .expect("damn")
        .to_string();

    let res = create_user(
        pool,
        username.clone(),
        display_name.clone(),
        passhash,
        token.clone(),
    )
    .await;

    // let res = sqlx::query(include_str!("../db/sql/user/create.sql"))
    //     .bind(&username)
    //     .bind(&display_name)
    //     .bind(&passhash)
    //     .bind(&token)
    //     .execute(data.get_ref())
    //     .await;

    match res {
        Err(e) => match e.as_database_error() {
            Some(error) => {
                if error.is_unique_violation() {
                    HttpResponse::Conflict().body("User already exists")
                } else {
                    println!("{e} {error}");
                    HttpResponse::InternalServerError().body("DB error")
                }
            }
            None => {
                println!("{e}");
                HttpResponse::InternalServerError().body("Unknown DB error")
            }
        },
        Ok(_) => HttpResponse::Created().json(UserCreateResponse {
            username,
            display_name,
            token,
        }),
    }
}
