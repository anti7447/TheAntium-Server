use chrono::{DateTime, Utc};

use crate::api::types::UserFull;
use crate::api::types::Session;
use crate::db::Pool;

pub async fn create(
    pool: &Pool,
    tag: &String,
    username: &String,
    password_hash: &String,
) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    sqlx::query(include_str!("sql/user/register.sql"))
        .bind(tag) // Tag
        .bind(username) // Username
        .bind(password_hash) // Password Hash
        // .bind(token) // Token
        .execute(pool)
        .await
}

pub async fn get_id(
    pool: &Pool,
    tag: &String,
    password_hash: &String,
) -> Result<Option<u32>, sqlx::Error> {
    sqlx::query_scalar(include_str!("sql/user/get_id.sql"))
        .bind(tag)
        .bind(password_hash)
        .fetch_optional(pool)
        .await
}

pub async fn get_user(pool: &Pool, tag: &String) -> Result<Option<UserFull>, sqlx::Error> {
    sqlx::query_as(include_str!("sql/user/get_user.sql"))
        .bind(tag)
        .fetch_optional(pool)
        .await
}

pub async fn create_session(
    pool: &Pool,
    user_id: &u32,
    device_name: &String,
    expires_at: &DateTime<Utc>,
) -> Result<u32, sqlx::Error> {
    sqlx::query_scalar(include_str!("sql/user/create_session.sql"))
        .bind(user_id)
        .bind(device_name)
        .bind(expires_at)
        .fetch_one(pool)
        .await
}

pub async fn get_session(pool: &Pool, id: &u32) -> Result<Session, sqlx::Error> {
    sqlx::query_as(include_str!("sql/user/get_session.sql"))
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn get_session_lite(pool: &Pool, id: &u32) -> Result<(u32, bool), sqlx::Error> {
    sqlx::query_as(include_str!("sql/user/get_session_lite.sql"))
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn delete_session(pool: &Pool, session_id: &u32) -> Result<(), sqlx::Error> {
    sqlx::query(include_str!("sql/user/del_session.sql"))
        .bind(session_id)
        .execute(pool)
        .await?;

    Ok(())
}