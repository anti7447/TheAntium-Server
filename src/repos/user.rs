use crate::{db::Pool, types::user::UserFull};

pub struct UserRepo {
    pool: Pool,
}

impl UserRepo {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        tag: String,
        username: String,
        password_hash: String,
    ) -> Result<u32, sqlx::Error> {
        let res: u32 = sqlx::query_scalar(include_str!("../db/sql/user/register.sql"))
            .bind(tag) // Tag
            .bind(username) // Username
            .bind(password_hash) // Password Hash
            .fetch_one(&self.pool)
            .await?;
        Ok(res)
    }

    pub async fn get_by_tag(&self, tag: String) -> Result<Option<UserFull>, sqlx::Error> {
        sqlx::query_as(include_str!("../db/sql/user/get_by_tag.sql"))
            .bind(tag)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn get_by_id(&self, id: u32) -> Result<Option<UserFull>, sqlx::Error> {
        sqlx::query_as(include_str!("../db/sql/user/get_by_id.sql"))
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn update_last_seen(&self, id: u32) -> Result<(), sqlx::Error> {
        sqlx::query(include_str!("../db/sql/user/update_last_seen.sql"))
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
