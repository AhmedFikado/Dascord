use crate::application::dto::channel::ChannelResponse;
use crate::infrastructure::repositories::ServerRepository;
use crate::infrastructure::repositories::channel::ChannelRepository;
use crate::utils::error::{AppError, AppResult};
use uuid::Uuid;

pub struct GetChannelInfoUseCase<CR: ChannelRepository, SR: ServerRepository> {
    channel_repo: CR,
    server_repo: SR,
}

impl<CR: ChannelRepository, SR: ServerRepository> GetChannelInfoUseCase<CR, SR> {
    pub fn new(channel_repo: CR, server_repo: SR) -> Self {
        Self {
            channel_repo,
            server_repo,
        }
    }

    pub async fn execute(&self, channel_id: Uuid, user_id: Uuid) -> AppResult<ChannelResponse> {
        let channel = self
            .channel_repo
            .find_by_id(channel_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Channel not found".to_string()))?;

        let is_member = self
            .server_repo
            .is_member(channel.server_id, user_id)
            .await?;
        if !is_member {
            return Err(AppError::Unauthorized(
                "Not a member of this server".to_string(),
            ));
        }

        Ok(ChannelResponse::from(channel))
    }
}

pub struct UpdateChannelUseCase<CR: ChannelRepository, SR: ServerRepository> {
    channel_repo: CR,
    server_repo: SR,
}

impl<CR: ChannelRepository, SR: ServerRepository> UpdateChannelUseCase<CR, SR> {
    pub fn new(channel_repo: CR, server_repo: SR) -> Self {
        Self {
            channel_repo,
            server_repo,
        }
    }

    pub async fn execute(
        &self,
        channel_id: Uuid,
        user_id: Uuid,
        name: String,
    ) -> AppResult<ChannelResponse> {
        let mut channel = self
            .channel_repo
            .find_by_id(channel_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Channel not found".to_string()))?;

        let role = self
            .server_repo
            .get_member_role(channel.server_id, user_id)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Not a member".to_string()))?;

        if !role.can_have_permissions() {
            return Err(AppError::Unauthorized(
                "Insufficient permissions".to_string(),
            ));
        }

        channel.name = name;
        let updated = self.channel_repo.update(channel).await?;
        Ok(ChannelResponse::from(updated))
    }
}

pub struct DeleteChannelUseCase<CR: ChannelRepository, SR: ServerRepository> {
    channel_repo: CR,
    server_repo: SR,
}

impl<CR: ChannelRepository, SR: ServerRepository> DeleteChannelUseCase<CR, SR> {
    pub fn new(channel_repo: CR, server_repo: SR) -> Self {
        Self {
            channel_repo,
            server_repo,
        }
    }

    pub async fn execute(&self, channel_id: Uuid, user_id: Uuid) -> AppResult<ChannelResponse> {
        let channel = self
            .channel_repo
            .find_by_id(channel_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Channel not found".to_string()))?;

        let role = self
            .server_repo
            .get_member_role(channel.server_id, user_id)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Not a member".to_string()))?;

        if !role.can_have_permissions() {
            return Err(AppError::Unauthorized(
                "Insufficient permissions".to_string(),
            ));
        }

        let response = ChannelResponse::from(channel);
        self.channel_repo.delete(channel_id).await?;
        Ok(response)
    }
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::channel::Channel;
    use crate::domain::value_objects::ServerRole;
    use crate::infrastructure::repositories::mocks::{
        mock_channel_repository::MockChannelRepository,
        mock_server_repository::MockServerRepository,
    };

    #[tokio::test]
    async fn test_get_channel_info_success() {
        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let channel = Channel::new(server_id, "General".to_string());

        let mock_channel_repo = MockChannelRepository::new().with_channel(channel.clone());
        let mock_server_repo =
            MockServerRepository::new().with_member(server_id, user_id, ServerRole::Member);

        let use_case = GetChannelInfoUseCase::new(mock_channel_repo, mock_server_repo);
        let result = use_case.execute(channel.id, user_id).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.name, "General");
    }

    #[tokio::test]
    async fn test_get_channel_info_not_member() {
        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let channel = Channel::new(server_id, "General".to_string());

        let mock_channel_repo = MockChannelRepository::new().with_channel(channel.clone());
        let mock_server_repo = MockServerRepository::new();

        let use_case = GetChannelInfoUseCase::new(mock_channel_repo, mock_server_repo);
        let result = use_case.execute(channel.id, user_id).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_channel_info_channel_not_found() {
        let user_id = Uuid::new_v4();
        let channel_id = Uuid::new_v4();

        let mock_channel_repo = MockChannelRepository::new();
        let mock_server_repo = MockServerRepository::new();

        let use_case = GetChannelInfoUseCase::new(mock_channel_repo, mock_server_repo);
        let result = use_case.execute(channel_id, user_id).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_channel_success() {
        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let channel = Channel::new(server_id, "Old Name".to_string());

        let mock_channel_repo = MockChannelRepository::new().with_channel(channel.clone());
        let mock_server_repo =
            MockServerRepository::new().with_member(server_id, user_id, ServerRole::Admin);

        let use_case = UpdateChannelUseCase::new(mock_channel_repo, mock_server_repo);
        let result = use_case
            .execute(channel.id, user_id, "New Name".to_string())
            .await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.name, "New Name");
    }

    #[tokio::test]
    async fn test_update_channel_insufficient_permissions() {
        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let channel = Channel::new(server_id, "Old Name".to_string());

        let mock_channel_repo = MockChannelRepository::new().with_channel(channel.clone());
        let mock_server_repo =
            MockServerRepository::new().with_member(server_id, user_id, ServerRole::Member);

        let use_case = UpdateChannelUseCase::new(mock_channel_repo, mock_server_repo);
        let result = use_case
            .execute(channel.id, user_id, "New Name".to_string())
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_channel_not_member() {
        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let channel = Channel::new(server_id, "Old Name".to_string());

        let mock_channel_repo = MockChannelRepository::new().with_channel(channel.clone());
        let mock_server_repo = MockServerRepository::new();

        let use_case = UpdateChannelUseCase::new(mock_channel_repo, mock_server_repo);
        let result = use_case
            .execute(channel.id, user_id, "New Name".to_string())
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_channel_channel_not_found() {
        let user_id = Uuid::new_v4();
        let channel_id = Uuid::new_v4();

        let mock_channel_repo = MockChannelRepository::new();
        let mock_server_repo = MockServerRepository::new();

        let use_case = UpdateChannelUseCase::new(mock_channel_repo, mock_server_repo);
        let result = use_case
            .execute(channel_id, user_id, "New Name".to_string())
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_channel_success() {
        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let channel = Channel::new(server_id, "Test Channel".to_string());

        let mock_channel_repo = MockChannelRepository::new().with_channel(channel.clone());
        let mock_server_repo =
            MockServerRepository::new().with_member(server_id, user_id, ServerRole::Owner);

        let use_case = DeleteChannelUseCase::new(mock_channel_repo, mock_server_repo);
        let result = use_case.execute(channel.id, user_id).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_channel_insufficient_permissions() {
        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let channel = Channel::new(server_id, "Test Channel".to_string());

        let mock_channel_repo = MockChannelRepository::new().with_channel(channel.clone());
        let mock_server_repo =
            MockServerRepository::new().with_member(server_id, user_id, ServerRole::Member);

        let use_case = DeleteChannelUseCase::new(mock_channel_repo, mock_server_repo);
        let result = use_case.execute(channel.id, user_id).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_channel_not_member() {
        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let channel = Channel::new(server_id, "Test Channel".to_string());

        let mock_channel_repo = MockChannelRepository::new().with_channel(channel.clone());
        let mock_server_repo = MockServerRepository::new();

        let use_case = DeleteChannelUseCase::new(mock_channel_repo, mock_server_repo);
        let result = use_case.execute(channel.id, user_id).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_channel_channel_not_found() {
        let user_id = Uuid::new_v4();
        let channel_id = Uuid::new_v4();

        let mock_channel_repo = MockChannelRepository::new();
        let mock_server_repo = MockServerRepository::new();

        let use_case = DeleteChannelUseCase::new(mock_channel_repo, mock_server_repo);
        let result = use_case.execute(channel_id, user_id).await;

        assert!(result.is_err());
    }
}
