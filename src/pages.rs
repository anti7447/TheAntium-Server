use actix_web::{HttpRequest, HttpResponse, Responder, cookie::Cookie, get, web};

#[get("/")]
pub async fn main_page(req: HttpRequest) -> impl Responder {
    println!("{:#?}", req.cookie("session_token"));
    let cookie = Cookie::build("session_token", "value")
        .domain("theantium.fun")
        .http_only(true)
        .path("/")
        .secure(true)
        .finish();
    println!("{:#?}", cookie);
    HttpResponse::Ok()
        .cookie(cookie)
        .body(include_str!("../front/main.html"))
}

#[get("/register")]
pub async fn register_page() -> impl Responder {
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
