use actix_web::{
    HttpResponse, Responder,
    cookie::{Cookie, CookieBuilder, time::Duration},
    post, web,
};
use argon2::{
    self, PasswordHash, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use argon2::{Argon2, PasswordHasher};
use chrono::{DateTime, TimeDelta, Utc};
use rand::{self, Rng};
use serde::{Deserialize, Serialize};
use sqlx::{Error, Executor};

use crate::{
    api::types::{UserFull, UserLoginRequest},
    db::{self, Pool},
    security,
};

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

pub fn get_scope() -> actix_web::Scope {
    return web::scope("/auth");
}

pub async fn login(pool: web::Data<Pool>, login: UserLoginRequest) -> impl Responder {
    let res = db::user::get_user(&pool, &login.tag).await;
    match res {
        Ok(Some(user)) => {
            let hash = PasswordHash::new(&user.password_hash);
            match hash {
                Ok(hash) => {
                    match Argon2::default().verify_password(login.password.as_bytes(), &hash) {
                        Err(_) => HttpResponse::Unauthorized().finish(),
                        Ok(_) => HttpResponse::Ok()
                            .cookie(
                                CookieBuilder::new(
                                    "refresh",
                                    security::jwt::get_token(
                                        "ya_hz_chto_tut_pisat".to_string(),
                                        Utc::now(),
                                        Utc::now() + TimeDelta::weeks(4),
                                    )
                                    .unwrap_or_default(),
                                )
                                .http_only(true)
                                .max_age(Duration::weeks(4))
                                .finish(),
                            )
                            .cookie(
                                CookieBuilder::new(
                                    "access",
                                    security::jwt::get_token(
                                        "ya_hz_chto_tut_pisat".to_string(),
                                        Utc::now(),
                                        Utc::now() + TimeDelta::weeks(4),
                                    )
                                    .unwrap_or_default(),
                                )
                                .http_only(true)
                                .max_age(Duration::minutes(10))
                                .finish(),
                            )
                            .json(user),
                    }
                }
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
        Ok(None) => HttpResponse::Unauthorized().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
