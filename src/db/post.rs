use crate::db::Pool;

pub async fn create(
    pool: &Pool,
    author_id: &u32,
    name: &String,
    content: &String,
) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    sqlx::query(include_str!("sql/post/create.sql"))
        .bind(author_id) // Author ID
        .bind(name) // Name
        .bind(content) // Content
        .execute(pool)
        .await
}
