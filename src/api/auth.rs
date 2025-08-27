use crate::{
    services::{
        session::{SessionService, SessionServiceError},
        user::{UserService, UserServiceError},
    },
    types::{
        session::RefreshRequest,
        user::{UserCreateRequest, UserLoginResponse, UserPublicView},
    },
};
use actix_web::{HttpResponse, Responder, web};

use crate::types::user::UserLoginRequest;

pub fn get_scope() -> actix_web::Scope {
    return web::scope("/auth")
        .service(web::resource("/register").to(register))
        .service(web::resource("/login").to(login))
        .service(web::resource("/refresh").to(refresh))
        .service(web::resource("/logout").to(logout));
}

async fn login(
    user_service: web::Data<UserService>,
    session_service: web::Data<SessionService>,
    login: web::Json<UserLoginRequest>,
) -> impl Responder {
    match user_service
        .login(&session_service, login.0, "UserAgent".to_string())
        .await
    {
        Ok((user, access, refresh)) => HttpResponse::Ok().json(UserLoginResponse {
            user: UserPublicView::from(user),
            access,
            refresh,
        }),
        Err(err) => match err {
            UserServiceError::InvalidCredentials => HttpResponse::Unauthorized().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}

async fn register(
    service: web::Data<UserService>,
    create: web::Json<UserCreateRequest>,
) -> impl Responder {
    match service.register(create.0).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(err) => match err {
            UserServiceError::UniqueViolation => HttpResponse::Conflict().finish(),
            UserServiceError::Validation(verr) => HttpResponse::BadRequest().body(verr),
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}

async fn refresh(
    service: web::Data<SessionService>,
    req: web::Json<RefreshRequest>,
) -> impl Responder {
    match service.refresh_access(&req.refresh).await {
        Ok(access) => HttpResponse::Ok().body(access),
        Err(err) => match err {
            SessionServiceError::Expired => HttpResponse::Unauthorized().body("Expired"),
            SessionServiceError::InvalidToken => HttpResponse::Unauthorized().finish(),
            SessionServiceError::NotFound => HttpResponse::Unauthorized().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}

async fn logout(
    service: web::Data<SessionService>,
    req: web::Json<RefreshRequest>,
) -> impl Responder {
    match service.logout(&req.refresh).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => match err {
            SessionServiceError::InvalidToken => HttpResponse::Unauthorized().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        },
    }
}
