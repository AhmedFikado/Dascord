use crate::application::dto::message_dto::MessageDto;
use crate::domain::entities::Message;
use crate::infrastructure::repositories::{ChannelRepository, MessageRepository, ServerRepository};
use crate::utils::error::{AppError, AppResult};
use uuid::Uuid;

pub struct SendMessageUseCase<MR: MessageRepository, CR: ChannelRepository, SR: ServerRepository> {
    message_repo: MR,
    channel_repo: CR,
    server_repo: SR,
}

impl<MR: MessageRepository, CR: ChannelRepository, SR: ServerRepository>
    SendMessageUseCase<MR, CR, SR>
{
    pub fn new(message_repo: MR, channel_repo: CR, server_repo: SR) -> Self {
        Self {
            message_repo,
            channel_repo,
            server_repo,
        }
    }

    pub async fn execute(
        &self,
        channel_id: Uuid,
        user_id: Uuid,
        username: String,
        content: String,
    ) -> AppResult<MessageDto> {
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

        let message = Message::new(
            channel_id.to_string(),
            user_id.to_string(),
            username.clone(),
            content.clone(),
        );

        let created = self.message_repo.create(message).await?;

        Ok(MessageDto {
            id: created.id.map(|id| id.to_string()),
            channel_id: created.channel_id,
            user_id: created.user_id,
            username: created.username,
            content: created.content,
            created_at: created.created_at.to_rfc3339(),
        })
    }
}

pub struct GetMessageHistoryUseCase<
    MR: MessageRepository,
    CR: ChannelRepository,
    SR: ServerRepository,
> {
    message_repo: MR,
    channel_repo: CR,
    server_repo: SR,
}

impl<MR: MessageRepository, CR: ChannelRepository, SR: ServerRepository>
    GetMessageHistoryUseCase<MR, CR, SR>
{
    pub fn new(message_repo: MR, channel_repo: CR, server_repo: SR) -> Self {
        Self {
            message_repo,
            channel_repo,
            server_repo,
        }
    }

    pub async fn execute(
        &self,
        channel_id: Uuid,
        user_id: Uuid,
        limit: i64,
    ) -> AppResult<Vec<MessageDto>> {
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

        let messages = self
            .message_repo
            .find_by_channel(&channel_id.to_string(), limit)
            .await?;

        Ok(messages
            .into_iter()
            .map(|m| MessageDto {
                id: m.id.map(|id| id.to_string()),
                channel_id: m.channel_id,
                user_id: m.user_id,
                username: m.username,
                content: m.content,
                created_at: m.created_at.to_rfc3339(),
            })
            .collect())
    }
}

pub struct DeleteMessageUseCase<MR: MessageRepository> {
    message_repo: MR,
}

impl<MR: MessageRepository> DeleteMessageUseCase<MR> {
    pub fn new(message_repo: MR) -> Self {
        Self { message_repo }
    }

    pub async fn execute(&self, message_id: String, _user_id: Uuid) -> AppResult<()> {
        self.message_repo.delete(&message_id).await
    }
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::{Channel, Message};
    use crate::domain::value_objects::ServerRole;
    use crate::infrastructure::repositories::mocks::{
        mock_channel_repository::MockChannelRepository,
        mock_message_repository::MockMessageRepository,
        mock_server_repository::MockServerRepository,
    };

    #[tokio::test]
    async fn test_send_message_success() {
        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let channel = Channel::new(server_id, "General".to_string());

        let mock_message_repo = MockMessageRepository::new();
        let mock_channel_repo = MockChannelRepository::new().with_channel(channel.clone());
        let mock_server_repo =
            MockServerRepository::new().with_member(server_id, user_id, ServerRole::Member);

        let use_case =
            SendMessageUseCase::new(mock_message_repo, mock_channel_repo, mock_server_repo);
        let result = use_case
            .execute(
                channel.id,
                user_id,
                "TestUser".to_string(),
                "Hello World".to_string(),
            )
            .await;

        assert!(result.is_ok());
        let message = result.unwrap();
        assert_eq!(message.content, "Hello World");
        assert_eq!(message.username, "TestUser");
    }

    #[tokio::test]
    async fn test_send_message_not_member() {
        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let channel = Channel::new(server_id, "General".to_string());

        let mock_message_repo = MockMessageRepository::new();
        let mock_channel_repo = MockChannelRepository::new().with_channel(channel.clone());
        let mock_server_repo = MockServerRepository::new();

        let use_case =
            SendMessageUseCase::new(mock_message_repo, mock_channel_repo, mock_server_repo);
        let result = use_case
            .execute(
                channel.id,
                user_id,
                "TestUser".to_string(),
                "Hello World".to_string(),
            )
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_send_message_channel_not_found() {
        let user_id = Uuid::new_v4();
        let channel_id = Uuid::new_v4();

        let mock_message_repo = MockMessageRepository::new();
        let mock_channel_repo = MockChannelRepository::new();
        let mock_server_repo = MockServerRepository::new();

        let use_case =
            SendMessageUseCase::new(mock_message_repo, mock_channel_repo, mock_server_repo);
        let result = use_case
            .execute(
                channel_id,
                user_id,
                "TestUser".to_string(),
                "Hello World".to_string(),
            )
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_message_history_success() {
        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let channel = Channel::new(server_id, "General".to_string());
        let message = Message::new(
            channel.id.to_string(),
            user_id.to_string(),
            "TestUser".to_string(),
            "Hello".to_string(),
        );

        let mock_message_repo = MockMessageRepository::new().with_message(message);
        let mock_channel_repo = MockChannelRepository::new().with_channel(channel.clone());
        let mock_server_repo =
            MockServerRepository::new().with_member(server_id, user_id, ServerRole::Member);

        let use_case =
            GetMessageHistoryUseCase::new(mock_message_repo, mock_channel_repo, mock_server_repo);
        let result = use_case.execute(channel.id, user_id, 50).await;

        assert!(result.is_ok());
        let messages = result.unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].content, "Hello");
    }

    #[tokio::test]
    async fn test_get_message_history_channel_not_found() {
        let user_id = Uuid::new_v4();
        let channel_id = Uuid::new_v4();

        let mock_message_repo = MockMessageRepository::new();
        let mock_channel_repo = MockChannelRepository::new();
        let mock_server_repo = MockServerRepository::new();

        let use_case =
            GetMessageHistoryUseCase::new(mock_message_repo, mock_channel_repo, mock_server_repo);
        let result = use_case.execute(channel_id, user_id, 50).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_message_history_not_member() {
        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let channel = Channel::new(server_id, "General".to_string());

        let mock_message_repo = MockMessageRepository::new();
        let mock_channel_repo = MockChannelRepository::new().with_channel(channel.clone());
        let mock_server_repo = MockServerRepository::new();

        let use_case =
            GetMessageHistoryUseCase::new(mock_message_repo, mock_channel_repo, mock_server_repo);
        let result = use_case.execute(channel.id, user_id, 50).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_message_history_empty() {
        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let channel = Channel::new(server_id, "General".to_string());

        let mock_message_repo = MockMessageRepository::new();
        let mock_channel_repo = MockChannelRepository::new().with_channel(channel.clone());
        let mock_server_repo =
            MockServerRepository::new().with_member(server_id, user_id, ServerRole::Member);

        let use_case =
            GetMessageHistoryUseCase::new(mock_message_repo, mock_channel_repo, mock_server_repo);
        let result = use_case.execute(channel.id, user_id, 50).await;

        assert!(result.is_ok());
        let messages = result.unwrap();
        assert_eq!(messages.len(), 0);
    }

    #[tokio::test]
    async fn test_delete_message_success() {
        let user_id = Uuid::new_v4();
        let mock_message_repo = MockMessageRepository::new();

        let use_case = DeleteMessageUseCase::new(mock_message_repo);
        let result = use_case.execute("msg_123".to_string(), user_id).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_send_message_with_empty_content() {
        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let channel = Channel::new(server_id, "General".to_string());

        let mock_message_repo = MockMessageRepository::new();
        let mock_channel_repo = MockChannelRepository::new().with_channel(channel.clone());
        let mock_server_repo =
            MockServerRepository::new().with_member(server_id, user_id, ServerRole::Member);

        let use_case =
            SendMessageUseCase::new(mock_message_repo, mock_channel_repo, mock_server_repo);
        let result = use_case
            .execute(channel.id, user_id, "TestUser".to_string(), "".to_string())
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_send_message_with_long_content() {
        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let channel = Channel::new(server_id, "General".to_string());
        let long_content = "a".repeat(2000);

        let mock_message_repo = MockMessageRepository::new();
        let mock_channel_repo = MockChannelRepository::new().with_channel(channel.clone());
        let mock_server_repo =
            MockServerRepository::new().with_member(server_id, user_id, ServerRole::Member);

        let use_case =
            SendMessageUseCase::new(mock_message_repo, mock_channel_repo, mock_server_repo);
        let result = use_case
            .execute(
                channel.id,
                user_id,
                "TestUser".to_string(),
                long_content.clone(),
            )
            .await;

        assert!(result.is_ok());
        let message = result.unwrap();
        assert_eq!(message.content, long_content);
    }
}
