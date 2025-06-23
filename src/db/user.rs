use argon2::password_hash::{SaltString, rand_core::OsRng};
use argon2::{Argon2, PasswordHasher};
use rand::Rng;

use crate::db::Pool;

pub async fn create(
    pool: &Pool,
    tag: &String,
    username: &String,
    password_hash: &String,
    token: &String,
) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    sqlx::query(include_str!("sql/user/register.sql"))
        .bind(tag) // Tag
        .bind(username) // Username
        .bind(password_hash) // Password Hash
        .bind(token) // Token
        .execute(pool)
        .await
}
