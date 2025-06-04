use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

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

async fn index_html() -> impl Responder {
  "<!DOCTYPE html>\n<html>\n<body>\n<title>\nТитле\n</title>\n</body>\n</html>\n"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .service(hello)
      .service(echo)
      .service(users)
      .service(
        web::scope("/page")
            .route("/index.html", web::get().to(index_html)),
      )
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}
