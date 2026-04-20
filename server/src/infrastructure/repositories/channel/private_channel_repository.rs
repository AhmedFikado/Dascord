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
    async fn update_last_message_at(&self, channel_id: Uuid, sender_id: Uuid, last_message_at: chrono::DateTime<chrono::Utc>) -> AppResult<()>;
    async fn hide_channel(&self, channel_id: Uuid, user_id: Uuid) -> AppResult<()>;
    async fn unhide_channel(&self, channel_id: Uuid, user_id: Uuid) -> AppResult<()>;
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
        let query = "INSERT INTO privateMessageChannel (id, user1, user2) VALUES ($1, $2, $3) RETURNING id, user1, user2, created_at, user1_hidden, user2_hidden";

        sqlx::query_as::<_, PrivateChannel>(query)
            .bind(id)
            .bind(user1)
            .bind(user2)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    async fn get_by_id(&self, id: Uuid) -> AppResult<Option<PrivateChannel>> {
        let query = "SELECT id, user1, user2, created_at, user1_hidden, user2_hidden FROM privateMessageChannel WHERE id = $1";

        sqlx::query_as::<_, PrivateChannel>(query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    async fn get_by_users(&self, user1: Uuid, user2: Uuid) -> AppResult<Option<PrivateChannel>> {
        let query = "SELECT id, user1, user2, created_at, user1_hidden, user2_hidden FROM privateMessageChannel WHERE (user1 = $1 AND user2 = $2) OR (user1 = $2 AND user2 = $1)";

        sqlx::query_as::<_, PrivateChannel>(query)
            .bind(user1)
            .bind(user2)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    async fn get_user_channels(&self, user_id: Uuid) -> AppResult<Vec<PrivateChannel>> {
        let query = "SELECT id, user1, user2, created_at, user1_hidden, user2_hidden FROM privateMessageChannel WHERE (user1 = $1 AND user1_hidden = false) OR (user2 = $1 AND user2_hidden = false) ORDER BY last_message_at DESC";

        sqlx::query_as::<_, PrivateChannel>(query)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    async fn update_last_message_at(&self, channel_id: Uuid, sender_id: Uuid, last_message_at: chrono::DateTime<chrono::Utc>) -> AppResult<()> {
        // Réinitialise le hidden du destinataire quand un nouveau message arrive
        let query = "UPDATE privateMessageChannel SET last_message_at = $1, user1_hidden = CASE WHEN user1 != $2 THEN false ELSE user1_hidden END, user2_hidden = CASE WHEN user2 != $2 THEN false ELSE user2_hidden END WHERE id = $3";

        sqlx::query(query)
            .bind(last_message_at)
            .bind(sender_id)
            .bind(channel_id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(())
    }

    async fn hide_channel(&self, channel_id: Uuid, user_id: Uuid) -> AppResult<()> {
        let query = "UPDATE privateMessageChannel SET user1_hidden = CASE WHEN user1 = $1 THEN true ELSE user1_hidden END, user2_hidden = CASE WHEN user2 = $1 THEN true ELSE user2_hidden END WHERE id = $2";

        sqlx::query(query)
            .bind(user_id)
            .bind(channel_id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(())
    }

    async fn unhide_channel(&self, channel_id: Uuid, user_id: Uuid) -> AppResult<()> {
        let query = "UPDATE privateMessageChannel SET user1_hidden = CASE WHEN user1 = $1 THEN false ELSE user1_hidden END, user2_hidden = CASE WHEN user2 = $1 THEN false ELSE user2_hidden END WHERE id = $2";

        sqlx::query(query)
            .bind(user_id)
            .bind(channel_id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(())
    }
}