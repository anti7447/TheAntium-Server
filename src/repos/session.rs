use crate::db::Pool;
use crate::types::session::SessionFull;
use chrono::{DateTime, Utc};

pub struct SessionRepo {
    pool: Pool,
}

impl SessionRepo {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        user_id: u32,
        device_name: &String,
        expires_at: &DateTime<Utc>,
    ) -> Result<u32, sqlx::Error> {
        let res: u32 = sqlx::query_scalar(include_str!("../db/sql/session/create.sql"))
            .bind(user_id)
            .bind(device_name)
            .bind(expires_at)
            .fetch_one(&self.pool)
            .await?;
        Ok(res)
    }

    pub async fn get_by_id(&self, id: u32) -> Result<Option<SessionFull>, sqlx::Error> {
        sqlx::query_as(include_str!("../db/sql/session/get_by_id.sql"))
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn get_by_user(&self, user_id: u32) -> Result<Vec<SessionFull>, sqlx::Error> {
        sqlx::query_as(include_str!("../db/sql/session/get_by_user.sql"))
            .bind(user_id)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn delete(&self, id: u32) -> Result<(), sqlx::Error> {
        sqlx::query(include_str!("../db/sql/session/delete.sql"))
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn delete_by_user(&self, user_id: u32) -> Result<(), sqlx::Error> {
        sqlx::query(include_str!("../db/sql/session/delete_by_user.sql"))
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
