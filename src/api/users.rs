use actix_web::{HttpResponse, Responder, post, web};
use argon2::{
    self,
    password_hash::{SaltString, rand_core::OsRng},
};
use argon2::{Argon2, PasswordHasher};
use rand::{self, Rng};
use serde::{Deserialize, Serialize};
use sqlx::Error;

use crate::db::{self, Pool};

#[derive(Deserialize)]
pub struct UserCreateRequest {
    tag: String,
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct UserCreateResponse {
    tag: String,
    username: String,
    token: String,
}

// pub fn get_scope() -> actix_web::Scope {
//     return web::scope("/auth").route("/register", web::post().to(register));
// }

#[post("/users")]
pub async fn post_users(
    pool: web::Data<Pool>,
    create: web::Json<UserCreateRequest>,
) -> impl Responder {
    let UserCreateRequest {
        tag,
        username,
        password,
    } = create.into_inner();

    let token: String = rand::rng()
        .sample_iter(&rand::distr::Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();
    let passhash: String = Argon2::default()
        .hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng))
        .expect("damn")
        .to_string();

    let res = db::user::create(&pool, &tag, &username, &passhash, &token).await;

    match res {
        Ok(_) => HttpResponse::Created().json(UserCreateResponse {
            tag,
            username,
            token,
        }),
        Err(e) => match e {
            Error::Database(db) => match db.kind() {
                sqlx::error::ErrorKind::CheckViolation => HttpResponse::BadRequest().finish(),
                sqlx::error::ErrorKind::UniqueViolation => {
                    HttpResponse::Conflict().body("User already exists")
                }
                _ => HttpResponse::InternalServerError().finish(),
            },
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}
