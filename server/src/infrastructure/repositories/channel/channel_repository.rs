use crate::domain::entities::channel::Channel;
use crate::utils::error::{AppError, AppResult};
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

#[async_trait]
pub trait ChannelRepository: Send + Sync + Clone {
    async fn create(&self, channel: Channel) -> AppResult<Channel>;
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Channel>>;
    async fn find_by_server(&self, server_id: Uuid) -> AppResult<Vec<Channel>>;
    async fn update(&self, channel: Channel) -> AppResult<Channel>;
    async fn delete(&self, id: Uuid) -> AppResult<()>;
}

#[derive(Clone)]
pub struct PostgresChannelRepository {
    pool: PgPool,
}

impl PostgresChannelRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ChannelRepository for PostgresChannelRepository {
    async fn create(&self, channel: Channel) -> AppResult<Channel> {
        sqlx::query(
            "INSERT INTO channels (id, server_id, name, created_at) VALUES ($1, $2, $3, $4)",
        )
        .bind(channel.id)
        .bind(channel.server_id)
        .bind(&channel.name)
        .bind(channel.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(channel)
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Channel>> {
        let result = sqlx::query_as::<_, Channel>(
            "SELECT id, server_id, name, created_at FROM channels WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(result)
    }

    async fn find_by_server(&self, server_id: Uuid) -> AppResult<Vec<Channel>> {
        let channels = sqlx::query_as::<_, Channel>(
            "SELECT id, server_id, name, created_at FROM channels WHERE server_id = $1",
        )
        .bind(server_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(channels)
    }

    async fn update(&self, channel: Channel) -> AppResult<Channel> {
        sqlx::query("UPDATE channels SET name = $1 WHERE id = $2")
            .bind(&channel.name)
            .bind(channel.id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(channel)
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        sqlx::query("DELETE FROM channels WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(())
    }
}
