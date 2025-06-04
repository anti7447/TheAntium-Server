use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
  HttpResponse::Ok().body("The Antium Server")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
  HttpResponse::Ok().body(req_body)
}

#[get("/users")]
async fn users() -> impl Responder {
  HttpResponse::Ok().body("Юзверьc")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .service(hello)
      .service(echo)
      .service(users)
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
