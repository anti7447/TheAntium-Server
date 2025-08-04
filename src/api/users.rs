use actix_web::{HttpResponse, Responder, post, web};
use argon2::{
    self,
    password_hash::{SaltString, rand_core::OsRng},
};
use argon2::{Argon2, PasswordHasher};
use serde::{Deserialize, Serialize};
use sqlx::Error;

use crate::api::ErrorResponse;
use crate::db::{self, Pool};

#[derive(Deserialize)]
pub struct UserCreateRequest {
    tag: String,
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct UserCreateResponse {
    id: u32,
    tag: String,
    username: String,
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

    // let token = get_token(sub, secs);
    let passhash: String = Argon2::default()
        .hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng))
        .unwrap()
        .to_string();

    let res = db::user::create(&pool, &tag, &username, &passhash).await;

    match res {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => match e {
            Error::Database(db) => match db.kind() {
                sqlx::error::ErrorKind::CheckViolation => HttpResponse::BadRequest()
                    .json(ErrorResponse {
                    error: "BadRequest".to_string(),
                    message:
                        "Check: tag <= 32 characters and consists only of Latin letters and numbers"
                            .to_string(),
                    status: 400,
                }),
                sqlx::error::ErrorKind::UniqueViolation => {
                    HttpResponse::Conflict().json(ErrorResponse {
                        error: "Conflict".to_string(),
                        message: "User already exists".to_string(),
                        status: 409,
                    })
                }
                _ => HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "InternalServerError".to_string(),
                    message: "Something in DB-request ¯\\_(ツ)_/¯".to_string(),
                    status: 500,
                }),
            },
            _ => HttpResponse::InternalServerError().json(ErrorResponse {
                error: "InternalServerError".to_string(),
                message: "¯\\_(ツ)_/¯".to_string(),
                status: 500,
            }),
        },
    }
}
