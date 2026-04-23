use crate::domain::entities::User;
use crate::utils::error::{AppError, AppResult};
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync + Clone {
    async fn create(&self, user: User) -> AppResult<User>;
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<User>>;
    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>>;
    async fn find_by_username(&self, username: &str) -> AppResult<Option<User>>;
    async fn update_status(&self, id: Uuid, status: &str) -> AppResult<()>;
    async fn update_user(&self, user: User) -> AppResult<Option<User>>;
    async fn update_avatar(&self, id: Uuid, avatar_id: Option<String>) -> AppResult<()>;
}

#[derive(Clone)]
pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: User) -> AppResult<User> {
        sqlx::query(
            "INSERT INTO users (id, username, email, password_hash, status, language, created_at) VALUES ($1, $2, $3, $4, $5::user_status, $6, $7)"
        )
        .bind(user.id)
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(&user.status)
        .bind(&user.language)
        .bind(user.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(user)
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<User>> {
        let result = sqlx::query_as::<_, User>(
            "SELECT id, username, email, password_hash, status::text as status, language, avatar_id, created_at FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(result)
    }

    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>> {
        let result = sqlx::query_as::<_, User>(
            "SELECT id, username, email, password_hash, status::text as status, language, avatar_id, created_at FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(result)
    }

    async fn find_by_username(&self, username: &str) -> AppResult<Option<User>> {
        let result = sqlx::query_as::<_, User>(
            "SELECT id, username, email, password_hash, status::text as status, language, avatar_id, created_at FROM users WHERE username = $1"
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(result)
    }

    async fn update_status(&self, id: Uuid, status: &str) -> AppResult<()> {
        sqlx::query("UPDATE users SET status = $1::user_status WHERE id = $2")
            .bind(status)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(())
    }

    async fn update_user(&self, user: User) -> AppResult<Option<User>> {
        sqlx::query_as::<_, User>(
            "UPDATE users SET username = $1, email = $2, language = $3 WHERE id = $4 RETURNING id, username, email, password_hash, status::text as status, language, avatar_id, created_at"
        )
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.language)
        .bind(user.id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    async fn update_avatar(&self, id: Uuid, avatar_id: Option<String>) -> AppResult<()> {
        sqlx::query("UPDATE users SET avatar_id = $1 WHERE id = $2")
            .bind(avatar_id)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(())
    }
}
