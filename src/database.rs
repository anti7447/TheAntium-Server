use sqlx::sqlite::SqlitePool;

pub async fn init_db(pool: &SqlitePool) {
    // sqlx::query(include_str!("../init.sql"))
    //     .execute(pool)
    //     .await
    //     .unwrap();
}
