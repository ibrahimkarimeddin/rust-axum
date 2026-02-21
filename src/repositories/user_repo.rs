use crate::error::AppError;
use crate::models::user::{CreateUser, UpdateUser, User};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, payload: CreateUser) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (id, name, email)
            VALUES ($1, $2, $3)
            RETURNING id, name, email
            "#
        )
        .bind(Uuid::new_v4())
        .bind(payload.name)
        .bind(payload.email)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_user(&self, id: Uuid) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT id, name, email FROM users
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(AppError::UserNotFound)?;

        Ok(user)
    }

    pub async fn get_users(&self) -> Result<Vec<User>, AppError> {
        let users = sqlx::query_as::<_, User>(
            r#"
            SELECT id, name, email FROM users
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    pub async fn update_user(&self, id: Uuid, payload: UpdateUser) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET name = COALESCE($1, name),
                email = COALESCE($2, email)
            WHERE id = $3
            RETURNING id, name, email
            "#
        )
        .bind(payload.name)
        .bind(payload.email)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(AppError::UserNotFound)?;

        Ok(user)
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query(
            r#"
            DELETE FROM users WHERE id = $1
            "#
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::UserNotFound);
        }

        Ok(())
    }
}
