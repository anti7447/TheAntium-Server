use argon2::password_hash;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::{
    fs,
    sync::{Mutex, OnceLock},
};

pub async fn init_db(pool: &SqlitePool) {
    sqlx::query(include_str!("./sql/init.sql"))
        .execute(pool)
        .await
        .unwrap();
}

pub async fn create_user(
    pool: &SqlitePool,
    tag: String,
    username: String,
    password_hash: String,
    token: String,
) -> Result<(), sqlx::Error> {
    let res = sqlx::query(include_str!("./sql/user/create.sql"))
        .bind(tag)
        .bind(username)
        .bind("https://theantium.fun/avatars/default") // avatar_url
        .bind("https://theantium.fun/banners/default") // banner_url
        .bind(password_hash)
        .bind("salt") // salt
        .bind(token)
        .bind(0) // telegram_id
        .execute(pool)
        .await;

    match res {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

// static DB_POOL: OnceLock<SqlitePool> = OnceLock::new();

// pub async fn init(file: &str) {
//     let options = SqliteConnectOptions::new()
//         .filename("database.db")
//         .create_if_missing(true);
//     // let pool = SqlitePool::connect_with(options).await;

//     let pool = SqlitePoolOptions::new()
//         .max_connections(5)
//         .connect_with(options)
//         // .connect(&format!("sqlite://{}", file))
//         .await;

//     let _ = match pool {
//         Ok(pool_) => DB_POOL.set(pool_),
//         Err(error) => panic!("Failed to connect to db: {}", error),
//     };

//     println!("Connected to database!");
// }

// pub fn get() -> &'static SqlitePool {
//     return DB_POOL.get().expect("Not initialized");
// }
