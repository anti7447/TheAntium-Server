use actix_web::{HttpRequest, HttpResponse, Responder, cookie::Cookie, get, web};
use askama::Template;

use crate::db::Pool;
use crate::security::jwt::get_token;
use crate::security::jwt::verify_token;
use crate::{DOMAIN, db};

#[derive(Template)]
#[template(path = "user.html")]
struct UserpaeTamplate {
    username: String,
    tag: String,
    created_at: chrono::DateTime<chrono::Utc>,
    role: String,
}

#[get("/")]
pub async fn main_page(req: HttpRequest) -> impl Responder {
    // println!("{:#?}", req.cookie("session_token"));
    // let cookie = Cookie::build("session_token", "value")
    //     .domain("theantium.fun")
    //     .http_only(true)
    //     .path("/")
    //     .secure(true)
    //     .finish();
    // println!("{:#?}", cookie);
    HttpResponse::Ok()
        // .cookie(cookie)
        .body(include_str!("../front/main.html"))
}

#[get("/register")]
pub async fn register_page(req: HttpRequest) -> impl Responder {
    // println!("{:#?}", req.cookie("session_token"));
    // let cookie = Cookie::build("session_token", "value")
    //     .domain("theantium.fun")
    //     .http_only(true)
    //     .path("/")
    //     .secure(true)
    //     .finish();
    // println!("{:#?}", cookie);
    HttpResponse::Ok()
        // .cookie(cookie)
        .body(include_str!("../front/register.html"))
}

#[get("/login")]
pub async fn login_page() -> impl Responder {
    HttpResponse::Ok().body(include_str!("../front/signin.html"))
}

#[get("/user")]
pub async fn user_page(pool: web::Data<Pool>, req: HttpRequest) -> impl Responder {
    log::debug!("Cool: {:#?}", req.cookie("user_token"));
    let user_token: String;
    if let Some(st) = req.cookie("user_token") {
        user_token = st.value().to_string();
    } else {
        return HttpResponse::Found()
            .append_header(("Location", "/login"))
            .finish();
        // user_token = "st.value().to_string()".to_string();
    }

    let claims = match verify_token(user_token) {
        Ok(t) => t,
        Err(e) => match e.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken => {
                return HttpResponse::BadRequest().body(format!("It isn's a token"));
            }
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                return HttpResponse::BadRequest().body(format!("Token has expired"));
            }
            _ => {
                return HttpResponse::InternalServerError()
                    .body(format!("idk, {e}\nPLease, make a bug report (sid)"));
            }
        },
    };

    // User ID as &str
    let uid_s = &claims.sub[1..];

    let user_id: u32 = match uid_s.parse() {
        Ok(id) => id,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("idk, {e}\nPLease, make a bug report (uid_u/user)"));
        }
    };

    let user_res = db::user::get_user_by_id(&pool, &user_id).await;
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

    let userpage = UserpaeTamplate {
        username: user.username,
        tag: user.tag,
        created_at: user.created_at,
        role: user.role,
    };

    // let userpage = UserpaeTamplate { username: todo!(), tag: todo!(), created_at: todo!(), role: todo!() }
    HttpResponse::Ok()
        // .cookie(cookie)
        .body(userpage.to_string())
}
