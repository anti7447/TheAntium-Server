use serde::{Deserialize, Serialize};
use actix_web::{
  web, App, HttpServer, HttpResponse, Error, Responder, error::ErrorNotFound,
  dev::{ServiceRequest, ServiceResponse},
  middleware::{self, Next},
  body::MessageBody,
};
use std::{
  collections::HashMap,
  sync::{
    atomic::{AtomicU64, Ordering},
    Mutex
  }
};

type UserDatabase = Mutex<HashMap<u64, User>>;

const ADDRESS: &str = "127.0.0.1";
const PORT: u16 = 8080;

struct AntiumState {
  counter: AtomicU64
}

#[derive(Serialize, Deserialize)]
struct User {
  name: String
}

#[derive(Serialize)]
struct CreateUserResponse {
  id: u64,
  name: String
}

#[derive(Serialize)]
struct AntiumStateResponse {
  counter_count: u64
}

// GLOBAL MIDDLEWARE
async fn increment_state_counter<B>(
  req: ServiceRequest,
  next: Next<B>,
) -> Result<ServiceResponse<B>, Error>
where
  B: MessageBody {
  if let Some(state) = req.app_data::<web::Data<AntiumState>>() {
    state.counter.fetch_add(1, Ordering::Relaxed);
  }
  next.call(req).await
}


// Method: GET
// Path: /
async fn index(state: web::Data<AntiumState>) -> String {
  state.counter.fetch_add(1, Ordering::Relaxed);
  format!("Request number: {}", state.counter.load(Ordering::Relaxed))
}

// Method: POST
// Scope: /users
// Path: /create
async fn create_user(
  user_data: web::Json<User>,
  db: web::Data<UserDatabase>
) -> impl Responder {

  let mut db = match db.lock() {
    Ok(db) => db,
    Err(err) => err.into_inner()
  };

  let new_id = match db.keys().max() {
    Some(id) => *id,
    None => 1
  };

  let name = user_data.name.clone();
  db.insert(new_id, user_data.into_inner());
  HttpResponse::Created().json(CreateUserResponse {
    id: new_id,
    name,
  })
}

// Method: GET
// Path: /state
async fn get_antium_state(state: web::Data<AntiumState>) -> impl Responder {
  HttpResponse::Ok().json(AntiumStateResponse {
    counter_count: state.counter.load(Ordering::Relaxed)
  })
}

// Method: GET
// Scope: /users
// Path: /{id}
async fn get_user(
  user_id: web::Path<u64>,
  db: web::Data::<UserDatabase>
) -> Result<impl Responder, actix_web::Error> {

  let user_id = user_id.into_inner();

  let db = match db.lock() {
    Ok(db) => db,
    Err(err) => err.into_inner()
  };

  match db.get(&user_id) {
    Some(user_data) => Ok(HttpResponse::Ok().json(user_data)),
    None => Err(ErrorNotFound("User not found!"))
  }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let user_db = web::Data::new(
    Mutex::new(HashMap::<u64, User>::new())
  );

  let state = web::Data::new(AntiumState {
    counter: AtomicU64::new(0)
  });

  HttpServer::new(move || {
    App::new()
      .app_data(user_db.clone())
      .app_data(state.clone())

      .wrap(middleware::from_fn(increment_state_counter))

      .route("/", web::get().to(index))
      .route("/state", web::get().to(get_antium_state))

      .service(
        web::scope("/users")
          .route("/create", web::post().to(create_user))
          .route("/{id}", web::get().to(get_user))
      )
  })
  .bind((ADDRESS, PORT))?
  .run()
  .await
}
