use crate::utils::error::{AppError, AppResult};
use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UnreadChannelInfo {
    pub channel_id: Uuid,
    pub first_unread_message_id: String,
    pub is_private: bool,
}

#[async_trait]
pub trait ReadStatusRepository: Send + Sync + Clone {
    async fn add_unread(
        &self,
        user_id: Uuid,
        channel_id: Uuid,
        first_message_id: &str,
        is_private: bool,
    ) -> AppResult<()>;

    async fn mark_read(&self, user_id: Uuid, channel_id: Uuid) -> AppResult<()>;

    async fn get_unread_channels_for_server(
        &self,
        user_id: Uuid,
        server_id: Uuid,
    ) -> AppResult<Vec<UnreadChannelInfo>>;

    async fn get_unread_private_channels(
        &self,
        user_id: Uuid,
    ) -> AppResult<Vec<UnreadChannelInfo>>;

    async fn get_server_member_ids_for_channel(
        &self,
        channel_id: Uuid,
    ) -> AppResult<Vec<Uuid>>;
}

#[derive(Clone)]
pub struct PostgresReadStatusRepository {
    pool: PgPool,
}

impl PostgresReadStatusRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ReadStatusRepository for PostgresReadStatusRepository {
    async fn add_unread(
        &self,
        user_id: Uuid,
        channel_id: Uuid,
        first_message_id: &str,
        is_private: bool,
    ) -> AppResult<()> {
        sqlx::query(
            "INSERT INTO unread_channels (user_id, channel_id, first_unread_message_id, is_private)
             VALUES ($1, $2, $3, $4)
             ON CONFLICT (user_id, channel_id) DO NOTHING",
        )
        .bind(user_id)
        .bind(channel_id)
        .bind(first_message_id)
        .bind(is_private)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(())
    }

    async fn mark_read(&self, user_id: Uuid, channel_id: Uuid) -> AppResult<()> {
        sqlx::query(
            "DELETE FROM unread_channels WHERE user_id = $1 AND channel_id = $2",
        )
        .bind(user_id)
        .bind(channel_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(())
    }

    async fn get_unread_channels_for_server(
        &self,
        user_id: Uuid,
        server_id: Uuid,
    ) -> AppResult<Vec<UnreadChannelInfo>> {
        let rows = sqlx::query(
            "SELECT uc.channel_id, uc.first_unread_message_id, uc.is_private
             FROM unread_channels uc
             INNER JOIN channels c ON c.id = uc.channel_id
             WHERE uc.user_id = $1 AND c.server_id = $2 AND uc.is_private = FALSE",
        )
        .bind(user_id)
        .bind(server_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        let result = rows
            .into_iter()
            .map(|row| UnreadChannelInfo {
                channel_id: row.get("channel_id"),
                first_unread_message_id: row.get("first_unread_message_id"),
                is_private: row.get("is_private"),
            })
            .collect();

        Ok(result)
    }

    async fn get_unread_private_channels(
        &self,
        user_id: Uuid,
    ) -> AppResult<Vec<UnreadChannelInfo>> {
        let rows = sqlx::query(
            "SELECT channel_id, first_unread_message_id, is_private
             FROM unread_channels
             WHERE user_id = $1 AND is_private = TRUE",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        let result = rows
            .into_iter()
            .map(|row| UnreadChannelInfo {
                channel_id: row.get("channel_id"),
                first_unread_message_id: row.get("first_unread_message_id"),
                is_private: row.get("is_private"),
            })
            .collect();

        Ok(result)
    }

    async fn get_server_member_ids_for_channel(
        &self,
        channel_id: Uuid,
    ) -> AppResult<Vec<Uuid>> {
        let rows = sqlx::query(
            "SELECT sm.user_id
             FROM server_members sm
             INNER JOIN channels c ON c.server_id = sm.server_id
             WHERE c.id = $1",
        )
        .bind(channel_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        let result = rows.into_iter().map(|row| row.get("user_id")).collect();
        Ok(result)
    }
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::repositories::mocks::mock_read_status_repository::MockReadStatusRepository;

    #[tokio::test]
    async fn test_add_unread_then_mark_read() {
        let repo = MockReadStatusRepository::new();
        let user_id = Uuid::new_v4();
        let channel_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();

        let unread_before = repo
            .get_unread_channels_for_server(user_id, server_id)
            .await
            .unwrap();
        assert_eq!(unread_before.len(), 0);

        repo.add_unread(user_id, channel_id, "msg001", false).await.unwrap();

        let unread_after = repo
            .get_unread_channels_for_server(user_id, server_id)
            .await
            .unwrap();
        assert_eq!(unread_after.len(), 1);
        assert_eq!(unread_after[0].channel_id, channel_id);
        assert_eq!(unread_after[0].first_unread_message_id, "msg001");

        repo.mark_read(user_id, channel_id).await.unwrap();

        let unread_final = repo
            .get_unread_channels_for_server(user_id, server_id)
            .await
            .unwrap();
        assert_eq!(unread_final.len(), 0);
    }

    #[tokio::test]
    async fn test_add_unread_private_then_get() {
        let repo = MockReadStatusRepository::new();
        let user_id = Uuid::new_v4();
        let channel_id = Uuid::new_v4();

        let empty = repo.get_unread_private_channels(user_id).await.unwrap();
        assert_eq!(empty.len(), 0);

        repo.add_unread(user_id, channel_id, "msg001", true).await.unwrap();

        let unread = repo.get_unread_private_channels(user_id).await.unwrap();
        assert_eq!(unread.len(), 1);
        assert!(unread[0].is_private);

        repo.mark_read(user_id, channel_id).await.unwrap();
        let after = repo.get_unread_private_channels(user_id).await.unwrap();
        assert_eq!(after.len(), 0);
    }

    #[tokio::test]
    async fn test_add_unread_no_duplicate() {
        let repo = MockReadStatusRepository::new();
        let user_id = Uuid::new_v4();
        let channel_id = Uuid::new_v4();

        repo.add_unread(user_id, channel_id, "msg001", false).await.unwrap();
        repo.add_unread(user_id, channel_id, "msg002", false).await.unwrap();

        let unread = repo
            .get_unread_channels_for_server(user_id, Uuid::new_v4())
            .await
            .unwrap();
        // Les deux appels ne doivent créer qu'une entrée (ON CONFLICT DO NOTHING)
        assert_eq!(unread.len(), 1);
        assert_eq!(unread[0].first_unread_message_id, "msg001");
    }

    #[tokio::test]
    async fn test_mark_read_nonexistent() {
        let repo = MockReadStatusRepository::new();
        let result = repo.mark_read(Uuid::new_v4(), Uuid::new_v4()).await;
        assert!(result.is_ok());
    }
}
