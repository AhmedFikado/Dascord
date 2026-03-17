use uuid::Uuid;
use sqlx::PgPool;
use crate::domain::entities::channel::PrivateChannel;
use crate::utils::error::{AppError, AppResult};
use async_trait::async_trait;

#[async_trait]
pub trait PrivateChannelRepository: Send + Sync {
    async fn create(&self, user1: Uuid, user2: Uuid) -> AppResult<PrivateChannel>;
    async fn get_by_id(&self, id: Uuid) -> AppResult<Option<PrivateChannel>>;
    async fn get_by_users(&self, user1: Uuid, user2: Uuid) -> AppResult<Option<PrivateChannel>>;
    async fn get_user_channels(&self, user_id: Uuid) -> AppResult<Vec<PrivateChannel>>;
}

#[derive(Clone)]
pub struct PostgresPrivateChannelRepository {
    pool: PgPool,
}

impl PostgresPrivateChannelRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PrivateChannelRepository for PostgresPrivateChannelRepository {
    async fn create(&self, user1: Uuid, user2: Uuid) -> AppResult<PrivateChannel> {
        let id = Uuid::new_v4();
        let query = "INSERT INTO privateMessageChannel (id, user1, user2) VALUES ($1, $2, $3) RETURNING id, user1, user2, created_at";
        
        sqlx::query_as::<_, PrivateChannel>(query)
            .bind(id)
            .bind(user1)
            .bind(user2)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    async fn get_by_id(&self, id: Uuid) -> AppResult<Option<PrivateChannel>> {
        let query = "SELECT id, user1, user2, created_at FROM privateMessageChannel WHERE id = $1";
        
        sqlx::query_as::<_, PrivateChannel>(query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    async fn get_by_users(&self, user1: Uuid, user2: Uuid) -> AppResult<Option<PrivateChannel>> {
        let query = "SELECT id, user1, user2, created_at FROM privateMessageChannel WHERE (user1 = $1 AND user2 = $2) OR (user1 = $2 AND user2 = $1)";
        
        sqlx::query_as::<_, PrivateChannel>(query)
            .bind(user1)
            .bind(user2)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    async fn get_user_channels(&self, user_id: Uuid) -> AppResult<Vec<PrivateChannel>> {
        let query = "SELECT id, user1, user2, created_at FROM privateMessageChannel WHERE user1 = $1 OR user2 = $1 ORDER BY created_at DESC";
        
        sqlx::query_as::<_, PrivateChannel>(query)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))
    }
}