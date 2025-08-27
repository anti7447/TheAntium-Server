use core::str;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::sync::OnceLock;

pub type Pool = sqlx::SqlitePool;
pub type QueryResult = sqlx::sqlite::SqliteQueryResult;
pub type ConnectOptions = SqliteConnectOptions;
pub type PoolOptions = SqlitePoolOptions;

static DB_POOL: OnceLock<Pool> = OnceLock::new();

pub async fn init(file: &str) {
    let options = ConnectOptions::new().filename(file).create_if_missing(true);

    let pool = PoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .expect("Failed to connect to db");

    sqlx::query(include_str!("sql/init.sql"))
        .execute(&pool)
        .await
        .expect("Failed to execute startup SQL query");

    let _ = DB_POOL.set(pool);

    println!("Connected to database!");
}

pub fn get() -> &'static Pool {
    return DB_POOL.get().expect("Not initialized");
}
