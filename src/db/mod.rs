use std::{fs, sync::{Mutex, OnceLock}};

use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

pub static DB_POOL: OnceLock<SqlitePool> = OnceLock::new();

pub async fn init(file: &str) {
    match fs::exists(file) {
        Ok(res) => {
            if !res {
                fs::File::create(file).expect(&format!("Failed to create database file: {file}"));
                println!("Created new database file");
            }
        },
        Err(_) => {
            fs::File::create(file).expect(&format!("Failed to create database file: {file}"));
            println!("Created new database file");
        }
    }
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&format!("sqlite://{}", file))
        .await;

    let _ = match pool {
        Ok(pool_) => DB_POOL.set(pool_),
        Err(error) => panic!("Failed to connect to db: {}", error)
    };

    println!("Connected to database!");
}

pub fn get() -> &'static SqlitePool {
    return DB_POOL.get().expect("Not initialized")
}
