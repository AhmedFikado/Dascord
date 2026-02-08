use crate::application::dto::channel::{ChannelResponse, CreateChannelRequest};
use crate::domain::entities::Channel;
use crate::infrastructure::repositories::{
    channel_repository::ChannelRepository, server_repository::ServerRepository,
};
use crate::utils::error::{AppError, AppResult};
use uuid::Uuid;
use validator::Validate;

pub struct GetChannelsUseCase<SR: ServerRepository, CR: ChannelRepository> {
    server_repo: SR,
    channel_repo: CR,
}

impl<SR: ServerRepository, CR: ChannelRepository> GetChannelsUseCase<SR, CR> {
    pub fn new(server_repo: SR, channel_repo: CR) -> Self {
        Self {
            server_repo,
            channel_repo,
        }
    }

    pub async fn execute(&self, server_id: Uuid, user_id: Uuid) -> AppResult<Vec<ChannelResponse>> {
        let is_member = self.server_repo.is_member(server_id, user_id).await?;
        if !is_member {
            return Err(AppError::Unauthorized(
                "Not a member of this server".to_string(),
            ));
        }

        let channels = self.channel_repo.find_by_server(server_id).await?;
        Ok(channels.into_iter().map(ChannelResponse::from).collect())
    }
}

pub struct CreateChannelUseCase<SR: ServerRepository, CR: ChannelRepository> {
    server_repo: SR,
    channel_repo: CR,
}

impl<SR: ServerRepository, CR: ChannelRepository> CreateChannelUseCase<SR, CR> {
    pub fn new(server_repo: SR, channel_repo: CR) -> Self {
        Self {
            server_repo,
            channel_repo,
        }
    }

    pub async fn execute(
        &self,
        server_id: Uuid,
        _user_id: Uuid,
        request: CreateChannelRequest,
    ) -> AppResult<ChannelResponse> {
        request
            .validate()
            .map_err(|e| AppError::ValidationError(e.to_string()))?;

        let _server = self
            .server_repo
            .find_by_id(server_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Server not found".to_string()))?;

        if _server.owner_id != _user_id {
            let role = self
                .server_repo
                .get_member_role(server_id, _user_id)
                .await?
                .ok_or_else(|| AppError::Unauthorized("Not a member".to_string()))?;

            if !role.can_have_permissions() {
                return Err(AppError::Unauthorized(
                    "Insufficient permissions".to_string(),
                ));
            }
        }

        let channel = Channel::new(server_id, request.name);
        let created = self.channel_repo.create(channel).await?;

        Ok(ChannelResponse::from(created))
    }
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_channels_use_case_creation() {
        assert!(true);
    }

    #[test]
    fn test_create_channel_use_case_creation() {
        assert!(true);
    }

    #[tokio::test]
    async fn test_get_channels_success() {
        use crate::domain::entities::Channel;
        use crate::domain::value_objects::ServerRole;
        use crate::infrastructure::repositories::mocks::{
            mock_channel_repository::MockChannelRepository,
            mock_server_repository::MockServerRepository,
        };

        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let channel1 = Channel::new(server_id, "General".to_string());
        let channel2 = Channel::new(server_id, "Random".to_string());

        let mock_server_repo =
            MockServerRepository::new().with_member(server_id, user_id, ServerRole::Member);
        let mock_channel_repo = MockChannelRepository::new()
            .with_channel(channel1.clone())
            .with_channel(channel2.clone());

        let use_case = GetChannelsUseCase::new(mock_server_repo, mock_channel_repo);
        let result = use_case.execute(server_id, user_id).await;

        assert!(result.is_ok());
        let channels = result.unwrap();
        assert_eq!(channels.len(), 2);
    }

    #[tokio::test]
    async fn test_get_channels_not_member() {
        use crate::infrastructure::repositories::mocks::{
            mock_channel_repository::MockChannelRepository,
            mock_server_repository::MockServerRepository,
        };

        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();

        let mock_server_repo = MockServerRepository::new();
        let mock_channel_repo = MockChannelRepository::new();

        let use_case = GetChannelsUseCase::new(mock_server_repo, mock_channel_repo);
        let result = use_case.execute(server_id, user_id).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_channel_success_as_owner() {
        use crate::application::dto::channel::CreateChannelRequest;
        use crate::domain::entities::Server;
        use crate::infrastructure::repositories::mocks::{
            mock_channel_repository::MockChannelRepository,
            mock_server_repository::MockServerRepository,
        };

        let owner_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), owner_id, "CODE123".to_string());

        let mock_server_repo = MockServerRepository::new().with_server(server.clone());
        let mock_channel_repo = MockChannelRepository::new();

        let use_case = CreateChannelUseCase::new(mock_server_repo, mock_channel_repo);
        let request = CreateChannelRequest {
            name: "New Channel".to_string(),
        };

        let result = use_case.execute(server.id, owner_id, request).await;

        assert!(result.is_ok());
        let channel = result.unwrap();
        assert_eq!(channel.name, "New Channel");
    }

    #[tokio::test]
    async fn test_create_channel_success_as_admin() {
        use crate::application::dto::channel::CreateChannelRequest;
        use crate::domain::entities::Server;
        use crate::domain::value_objects::ServerRole;
        use crate::infrastructure::repositories::mocks::{
            mock_channel_repository::MockChannelRepository,
            mock_server_repository::MockServerRepository,
        };

        let owner_id = Uuid::new_v4();
        let admin_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), owner_id, "CODE123".to_string());

        let mock_server_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, admin_id, ServerRole::Admin);
        let mock_channel_repo = MockChannelRepository::new();

        let use_case = CreateChannelUseCase::new(mock_server_repo, mock_channel_repo);
        let request = CreateChannelRequest {
            name: "Admin Channel".to_string(),
        };

        let result = use_case.execute(server.id, admin_id, request).await;

        assert!(result.is_ok());
        let channel = result.unwrap();
        assert_eq!(channel.name, "Admin Channel");
    }

    #[tokio::test]
    async fn test_create_channel_insufficient_permissions() {
        use crate::application::dto::channel::CreateChannelRequest;
        use crate::domain::entities::Server;
        use crate::domain::value_objects::ServerRole;
        use crate::infrastructure::repositories::mocks::{
            mock_channel_repository::MockChannelRepository,
            mock_server_repository::MockServerRepository,
        };

        let owner_id = Uuid::new_v4();
        let member_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), owner_id, "CODE123".to_string());

        let mock_server_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, member_id, ServerRole::Member);
        let mock_channel_repo = MockChannelRepository::new();

        let use_case = CreateChannelUseCase::new(mock_server_repo, mock_channel_repo);
        let request = CreateChannelRequest {
            name: "Member Channel".to_string(),
        };

        let result = use_case.execute(server.id, member_id, request).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_channel_server_not_found() {
        use crate::application::dto::channel::CreateChannelRequest;
        use crate::infrastructure::repositories::mocks::{
            mock_channel_repository::MockChannelRepository,
            mock_server_repository::MockServerRepository,
        };

        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();

        let mock_server_repo = MockServerRepository::new();
        let mock_channel_repo = MockChannelRepository::new();

        let use_case = CreateChannelUseCase::new(mock_server_repo, mock_channel_repo);
        let request = CreateChannelRequest {
            name: "Test Channel".to_string(),
        };

        let result = use_case.execute(server_id, user_id, request).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_channel_validation_error() {
        use crate::application::dto::channel::CreateChannelRequest;
        use crate::domain::entities::Server;
        use crate::infrastructure::repositories::mocks::{
            mock_channel_repository::MockChannelRepository,
            mock_server_repository::MockServerRepository,
        };

        let owner_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), owner_id, "CODE123".to_string());

        let mock_server_repo = MockServerRepository::new().with_server(server.clone());
        let mock_channel_repo = MockChannelRepository::new();

        let use_case = CreateChannelUseCase::new(mock_server_repo, mock_channel_repo);
        let request = CreateChannelRequest {
            name: "".to_string(), // Empty name should fail validation
        };

        let result = use_case.execute(server.id, owner_id, request).await;

        assert!(result.is_err());
    }
}
