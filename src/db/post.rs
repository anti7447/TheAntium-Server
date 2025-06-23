use crate::db::Pool;

pub async fn create(
    pool: &Pool,
    name: &String,
    content: &String,
) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    sqlx::query(include_str!("sql/post/create.sql"))
        .bind(name) // Name
        .bind(content) // Content
        .execute(pool)
        .await
}
