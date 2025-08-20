use actix_web::{HttpRequest, HttpResponse, Responder, delete, get, post, web};
use argon2::password_hash::{SaltString, rand_core::OsRng};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::{Duration, Utc};
use log::debug;
use serde::{Deserialize, Serialize};
use sqlx::Error;

use crate::db;
use crate::db::Pool;
use crate::security::jwt::get_token;

use crate::api::types::*;

#[derive(Deserialize)]
pub struct SignRequest {
    tag: String,
    password: String,
}

#[derive(Serialize)]
pub struct SignRespond {
    user: UserPublicView,
    session_token: String,
    user_token: String,
}

// TODO:
// Checking for the presence of a user
// Checking the password
#[post("/login")]
pub async fn post_login(
    pool: web::Data<Pool>,
    data: web::Json<SignRequest>,
    req: HttpRequest,
) -> impl Responder {
    let SignRequest { tag, password } = data.into_inner();
    let device_name = get_device_name(&req).unwrap_or("Unknow Device").to_string();
    let now = Utc::now();
    let session_exp = now + Duration::days(6 * 30);
    let user_exp = now + Duration::minutes(15);

    let user_res = db::user::get_user(&pool, &tag).await;
    debug!("{:?}", user_res);
    let user_opt = match user_res {
        Ok(user) => user,
        Err(e) => match e {
            _ => {
                return HttpResponse::InternalServerError()
                    .body(format!("idk, {e}\nPLease, make a bug report (user)"));
            }
        },
    };
    let user = match user_opt {
        Some(user) => user,
        None => {
            return HttpResponse::Unauthorized().finish();
        }
    };

    let hash = match PasswordHash::new(&user.password_hash) {
        Ok(h) => h,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let status = Argon2::default()
        .verify_password(password.as_bytes(), &hash)
        .is_ok();

    if !status {
        HttpResponse::Unauthorized().finish();
    }

    let session_id_res =
        db::user::create_session(&pool, &user.id, &device_name, &session_exp).await;
    let session_id = match session_id_res {
        Ok(id) => id,
        Err(e) => match e {
            _ => {
                return HttpResponse::InternalServerError()
                    .body(format!("idk, {e}\nPLease, make a bug report (session)"));
            }
        },
    };

    let session_token_res = get_token(session_id.to_string(), now, session_exp);
    let session_token = match session_token_res {
        Ok(token) => token,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("idk, {e}\nPLease, make a bug report (st)"));
        }
    };

    let user_token_res = get_token(user.id.to_string(), now, user_exp);
    let user_token = match user_token_res {
        Ok(token) => token,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("idk, {e}\nPLease, make a bug report (ut)"));
        }
    };

    HttpResponse::Accepted().json(SignRespond {
        user: UserPublicView::from(user),
        session_token,
        user_token,
    })
}

#[post("/refresh")]
pub async fn post_refresh() -> impl Responder {
    HttpResponse::Ok().body("logIN")
}

#[delete("/logout")]
pub async fn delete_logout() -> impl Responder {
    HttpResponse::Ok().body("logout")
}

fn get_device_name<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    req.headers().get("Device-Name")?.to_str().ok()
}

// $argon2id$v=19$m=19456,t=2,p=1$cvtZm9CFRDdkbVNcCKRPaw$8s7RFp2KUjjbv3GZH7a8oLMn6f4c+XPf7ZlF4TSi0ng
// $argon2id$v=19$m=19456,t=2,p=1$jcerSVgHdyZLwGjjUisv0g$P+TtFdtE7qD3jqKHwMOQ8K6L+kEGDQAbmK3tzNWazL4
