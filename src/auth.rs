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
use crate::security::jwt::verify_token;

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

#[derive(Deserialize)]
pub struct RefreshRequest {
    session_token: String,
}

#[derive(Serialize)]
pub struct RefreshRespond {
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

    let session_token_res = get_token("s".to_owned() + session_id.to_string().as_str(), now, session_exp);
    let session_token = match session_token_res {
        Ok(token) => token,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("idk, {e}\nPLease, make a bug report (st)"));
        }
    };

    let user_token_res = get_token("u".to_owned() + user.id.to_string().as_str(), now, user_exp);
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
pub async fn post_refresh(
    pool: web::Data<Pool>,
    data: web::Json<RefreshRequest>,
) -> impl Responder {
    let RefreshRequest { session_token} = data.into_inner();
    let now = Utc::now();
    let user_exp = now + Duration::minutes(15);

    let claims = match verify_token(session_token) {
        Ok(t) => t,
        Err(e) => {
            match e.kind() {
                jsonwebtoken::errors::ErrorKind::InvalidToken => {
                    return HttpResponse::BadRequest()
                        .body(format!("It isn's a token"));
                }
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                    return HttpResponse::BadRequest()
                        .body(format!("Token has expired"));
                }
                _ => {
                    return HttpResponse::InternalServerError()
                        .body(format!("idk, {e}\nPLease, make a bug report (sid)"));
                }
            }
        }
    };

    // Session ID as &str
    let sid_s = &claims.sub[1..];

    debug!("{:#?}\n{:?}", claims, sid_s);

    if !claims.sub.starts_with("s") {
        return HttpResponse::BadRequest()
            .body(format!("It's not session token, bruh"));
    }

    let session_id: u32 = match sid_s.parse() {
        Ok(id) => id,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("idk, {e}\nPLease, make a bug report (sid_u)"));
        }
    };

    
    let session_res =
        db::user::get_session_lite(&pool, &session_id).await;
    let (user_id, is_valid) = match session_res {
        Ok(res) => res,
        Err(e) => match e {
            _ => {
                return HttpResponse::InternalServerError()
                    .body(format!("idk, {e}\nPLease, make a bug report (rsession)"));
            }
        },
    };

    if !is_valid {
        return HttpResponse::Unauthorized()
            .body("Session ID is invalid :(");
    }

    let user_token_res = get_token("u".to_owned() + user_id.to_string().as_str(), now, user_exp);
    let user_token = match user_token_res {
        Ok(token) => token,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("idk, {e}\nPLease, make a bug report (rut)"));
        }
    };

    HttpResponse::Accepted().json(RefreshRespond { user_token })
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
