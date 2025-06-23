use actix_web::Result;
use core::str;
use sqlx::{
    Error,
    error::{DatabaseError, ErrorKind},
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use std::sync::OnceLock;

use crate::db;

pub mod post;
pub mod user;

pub type Pool = sqlx::SqlitePool;
pub type QueryResult = sqlx::sqlite::SqliteQueryResult;
pub type ConnectOptions = SqliteConnectOptions;
pub type PoolOptions = SqlitePoolOptions;

pub enum DatabaseResult {
    Ok(QueryResult),
    UnknownDatabaseError(Box<dyn DatabaseError>),
    CheckViolation(Box<dyn DatabaseError>),
    UniqueViolation(Box<dyn DatabaseError>),
    ForeignKeyViolation(Box<dyn DatabaseError>),
    NotNullViolation(Box<dyn DatabaseError>),
    UnknownError(sqlx::Error),
}

pub fn wrap(res: Result<QueryResult, Error>) -> DatabaseResult {
    match res {
        Ok(qr) => DatabaseResult::Ok(qr),
        Err(error) => match error {
            Error::Database(db_error) => {
                let kind = db_error.kind();
                match kind {
                    ErrorKind::CheckViolation => DatabaseResult::CheckViolation(db_error),
                    ErrorKind::UniqueViolation => DatabaseResult::UniqueViolation(db_error),
                    ErrorKind::ForeignKeyViolation => DatabaseResult::ForeignKeyViolation(db_error),
                    ErrorKind::NotNullViolation => DatabaseResult::NotNullViolation(db_error),
                    _ => DatabaseResult::UnknownDatabaseError(db_error),
                }
            }
            other => DatabaseResult::UnknownError(other),
        },
    }
}

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
