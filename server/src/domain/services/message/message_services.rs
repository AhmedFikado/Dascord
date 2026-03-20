use crate::application::dto::message::MessageDto;
use crate::domain::entities::message::Message;
use crate::infrastructure::repositories::ServerRepository;
use crate::infrastructure::repositories::message::MessageRepository;
use crate::infrastructure::repositories::channel::ChannelRepository;
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
            reactions: created.reactions,
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

    pub async fn execute(&self, channel_id: Uuid, user_id: Uuid) -> AppResult<Vec<MessageDto>> {
        let channel = self.channel_repo.find_by_id(channel_id).await?
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

        let messages = self.message_repo.find_by_channel(&channel_id.to_string()).await?;
        
        Ok(messages.into_iter().map(|m| MessageDto {
            id: m.id.map(|id| id.to_string()),
            channel_id: m.channel_id,
            user_id: m.user_id,
            username: m.username,
            content: m.content,
            reactions: m.reactions,
            created_at: m.created_at.to_rfc3339(),
        }).collect())
    }
}

pub struct DeleteMessageUseCase<MR: MessageRepository, CR: ChannelRepository, SR: ServerRepository> {
    message_repo: MR,
    channel_repo: CR,
    server_repo: SR,
}

impl<MR: MessageRepository, CR: ChannelRepository, SR: ServerRepository> DeleteMessageUseCase<MR, CR, SR> {
    pub fn new(message_repo: MR, channel_repo: CR, server_repo: SR) -> Self {
        Self { message_repo, channel_repo, server_repo }
    }

    pub async fn execute(&self, message_id: String, user_id: Uuid) -> AppResult<String> {
        // Récupérer le message
        let message = self.message_repo.find_by_id(&message_id).await?
            .ok_or_else(|| AppError::NotFound("Message not found".to_string()))?;

        // Conserver le channel_id avant suppression
        let channel_id = message.channel_id.clone();

        //  Vérifier si l'utilisateur est le propriétaire du message
        let message_owner_id = Uuid::parse_str(&message.user_id)
            .map_err(|_| AppError::InternalServerError("Invalid user ID in message".to_string()))?;
        
        let is_message_owner = message_owner_id == user_id;

        //  Si ce n'est pas le propriétaire, vérifier s'il est Admin ou Owner du serveur
        if !is_message_owner {
            // Récupérer le channel pour obtenir le server_id
            let channel_uuid = Uuid::parse_str(&message.channel_id)
                .map_err(|_| AppError::InternalServerError("Invalid channel ID in message".to_string()))?;
            
            let channel = self.channel_repo.find_by_id(channel_uuid).await?
                .ok_or_else(|| AppError::NotFound("Channel not found".to_string()))?;

            // Vérifier le rôle de l'utilisateur
            let user_role = self.server_repo.get_member_role(channel.server_id, user_id).await?;
            
            match user_role {
                Some(role) if role.can_have_permissions() => {
                    // L'utilisateur est Admin ou Owner, il peut supprimer
                }
                _ => {
                    return Err(AppError::Forbidden(
                        "You don't have permission to delete this message".to_string()
                    ));
                }
            }
        }

        // Supprimer le message
        self.message_repo.delete(&message_id).await?;
        
        // Retourner le channel_id pour la diffusion WebSocket
        Ok(channel_id)
    }
}

pub struct UpdateMessageUseCase<MR: MessageRepository, CR: ChannelRepository, SR: ServerRepository> {
    message_repo: MR,
    channel_repo: CR,
    server_repo: SR,
}

impl<MR: MessageRepository, CR: ChannelRepository, SR: ServerRepository> UpdateMessageUseCase<MR, CR, SR> {
    pub fn new(message_repo: MR, channel_repo: CR, server_repo: SR) -> Self {
        Self { message_repo, channel_repo, server_repo }
    }

    pub async fn execute(&self, message_id: String, user_id: Uuid, new_content: String) -> AppResult<MessageDto> {
        // Récupérer le message
        let message = self.message_repo.find_by_id(&message_id).await?
            .ok_or_else(|| AppError::NotFound("Message not found".to_string()))?;

        // Vérifier si l'utilisateur est le propriétaire du message
        let message_owner_id = Uuid::parse_str(&message.user_id)
            .map_err(|_| AppError::InternalServerError("Invalid user ID in message".to_string()))?;
        
        if message_owner_id != user_id {
            return Err(AppError::Forbidden(
                "You can only edit your own messages".to_string()
            ));
        }

        // Vérifier que l'utilisateur est toujours membre du serveur
        let channel_id = Uuid::parse_str(&message.channel_id)
            .map_err(|_| AppError::InternalServerError("Invalid channel ID in message".to_string()))?;
        
        let channel = self.channel_repo.find_by_id(channel_id).await?
            .ok_or_else(|| AppError::NotFound("Channel not found".to_string()))?;

        let is_member = self.server_repo.is_member(channel.server_id, user_id).await?;
        if !is_member {
            return Err(AppError::Unauthorized(
                "Not a member of this server".to_string(),
            ));
        }

        // Mettre à jour le message
        let updated_message = self.message_repo.update(&message_id, new_content).await?;

        Ok(MessageDto {
            id: updated_message.id.map(|id| id.to_string()),
            channel_id: updated_message.channel_id,
            user_id: updated_message.user_id,
            username: updated_message.username,
            content: updated_message.content,
            reactions: updated_message.reactions,
            created_at: updated_message.created_at.to_rfc3339(),
        })
    }
}


pub struct AddReactionUseCase<MR: MessageRepository, CR: ChannelRepository, SR: ServerRepository> {
    message_repo: MR,
    channel_repo: CR,
    server_repo: SR,
}

impl<MR: MessageRepository, CR: ChannelRepository, SR: ServerRepository> AddReactionUseCase<MR, CR, SR> {
    pub fn new(message_repo: MR, channel_repo: CR, server_repo: SR) -> Self {
        Self { message_repo, server_repo, channel_repo }
    }

    pub async fn execute(&self, message_id: String, user_id: Uuid, reaction: String) -> AppResult<MessageDto> {
        let message = self.message_repo.find_by_id(&message_id).await?
            .ok_or_else(|| AppError::NotFound("Message not found".to_string()))?;

        let channel_id = Uuid::parse_str(&message.channel_id)
            .map_err(|_| AppError::InternalServerError("Invalid channel ID".to_string()))?;

        let channel = self.channel_repo.find_by_id(channel_id).await?
            .ok_or_else(|| AppError::NotFound("Channel not found".to_string()))?;

        let is_member = self.server_repo.is_member(channel.server_id, user_id).await?;
        if !is_member {
            return Err(AppError::Unauthorized("Not a member of this server".to_string()));
        }

        let updated = self.message_repo.add_reaction(&message_id, reaction, user_id.to_string()).await?;

        Ok(MessageDto {
            id: updated.id.map(|id| id.to_string()),
            channel_id: updated.channel_id,
            user_id: updated.user_id,
            username: updated.username,
            content: updated.content,
            reactions: updated.reactions,
            created_at: updated.created_at.to_rfc3339(),
        })
    }
}

pub struct RemoveReactionUseCase<MR: MessageRepository, CR: ChannelRepository, SR: ServerRepository> {
    message_repo: MR,
    channel_repo: CR,
    server_repo: SR,
}

impl<MR: MessageRepository, CR: ChannelRepository, SR: ServerRepository> RemoveReactionUseCase<MR, CR, SR> {
    pub fn new(message_repo: MR, channel_repo: CR, server_repo: SR) -> Self {
        Self { message_repo, server_repo, channel_repo }
    }

    pub async fn execute(&self, message_id: String, user_id: Uuid, reaction: String) -> AppResult<MessageDto> {
        let message = self.message_repo.find_by_id(&message_id).await?
            .ok_or_else(|| AppError::NotFound("Message not found".to_string()))?;

        let channel_id = Uuid::parse_str(&message.channel_id)
            .map_err(|_| AppError::InternalServerError("Invalid channel ID".to_string()))?;

        let channel = self.channel_repo.find_by_id(channel_id).await?
            .ok_or_else(|| AppError::NotFound("Channel not found".to_string()))?;

        let is_member = self.server_repo.is_member(channel.server_id, user_id).await?;
        if !is_member {
            return Err(AppError::Unauthorized("Not a member of this server".to_string()));
        }

        let updated = self.message_repo.remove_reaction(&message_id, reaction, user_id.to_string()).await?;

        Ok(MessageDto {
            id: updated.id.map(|id| id.to_string()),
            channel_id: updated.channel_id,
            user_id: updated.user_id,
            username: updated.username,
            content: updated.content,
            reactions: updated.reactions,
            created_at: updated.created_at.to_rfc3339(),
        })
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
        let result = use_case.execute(channel.id, user_id).await;

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
        let result = use_case.execute(channel_id, user_id).await;

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
        let result = use_case.execute(channel.id, user_id).await;

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
        let result = use_case.execute(channel.id, user_id).await;

        assert!(result.is_ok());
        let messages = result.unwrap();
        assert_eq!(messages.len(), 0);
    }

    #[tokio::test]
    async fn test_delete_message_success() {
        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let channel = Channel::new(server_id, "General".to_string());
        
        let message = Message::new(
            channel.id.to_string(),
            user_id.to_string(),
            "TestUser".to_string(),
            "Test message".to_string(),
        );
        
        let expected_channel_id = channel.id.to_string();
        
        let mock_message_repo = MockMessageRepository::new().with_message(message.clone());
        let mock_channel_repo = MockChannelRepository::new().with_channel(channel.clone());
        let mock_server_repo = MockServerRepository::new().with_member(server_id, user_id, ServerRole::Member);

        let use_case = DeleteMessageUseCase::new(mock_message_repo, mock_channel_repo, mock_server_repo);
        let result = use_case.execute("msg_1".to_string(), user_id).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_channel_id);
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

    #[tokio::test]
    async fn test_update_message_success() {
        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let channel = Channel::new(server_id, "General".to_string());
        
        let message = Message::new(
            channel.id.to_string(),
            user_id.to_string(),
            "TestUser".to_string(),
            "Original content".to_string(),
        );
        
        let mock_message_repo = MockMessageRepository::new().with_message(message.clone());
        let mock_channel_repo = MockChannelRepository::new().with_channel(channel.clone());
        let mock_server_repo = MockServerRepository::new().with_member(server_id, user_id, ServerRole::Member);

        let use_case = UpdateMessageUseCase::new(mock_message_repo, mock_channel_repo, mock_server_repo);
        let result = use_case.execute("msg_1".to_string(), user_id, "Updated content".to_string()).await;

        assert!(result.is_ok());
        let updated = result.unwrap();
        assert_eq!(updated.content, "Updated content");
    }

    #[tokio::test]
    async fn test_update_message_not_owner() {
        let owner_id = Uuid::new_v4();
        let other_user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let channel = Channel::new(server_id, "General".to_string());
        
        let message = Message::new(
            channel.id.to_string(),
            owner_id.to_string(),
            "Owner".to_string(),
            "Original content".to_string(),
        );
        
        let mock_message_repo = MockMessageRepository::new().with_message(message.clone());
        let mock_channel_repo = MockChannelRepository::new().with_channel(channel.clone());
        let mock_server_repo = MockServerRepository::new()
            .with_member(server_id, owner_id, ServerRole::Member)
            .with_member(server_id, other_user_id, ServerRole::Admin);

        let use_case = UpdateMessageUseCase::new(mock_message_repo, mock_channel_repo, mock_server_repo);
        let result = use_case.execute("msg_1".to_string(), other_user_id, "Hacked content".to_string()).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::Forbidden(_) => (),
            _ => panic!("Expected Forbidden error"),
        }
    }

    #[tokio::test]
    async fn test_add_reaction_success() {
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
        let mock_server_repo = MockServerRepository::new().with_member(server_id, user_id, ServerRole::Member);

        let use_case = AddReactionUseCase::new(mock_message_repo, mock_channel_repo, mock_server_repo);
        let result = use_case.execute("msg_1".to_string(), user_id, "👍".to_string()).await;

        assert!(result.is_ok());
        let updated = result.unwrap();
        assert!(updated.reactions.contains_key("👍"));
        assert!(updated.reactions["👍"].contains(&user_id.to_string()));
    }

    #[tokio::test]
    async fn test_add_reaction_not_member() {
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
        let mock_server_repo = MockServerRepository::new(); // pas de membre

        let use_case = AddReactionUseCase::new(mock_message_repo, mock_channel_repo, mock_server_repo);
        let result = use_case.execute("msg_1".to_string(), user_id, "👍".to_string()).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remove_reaction_success() {
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
        let mock_server_repo = MockServerRepository::new().with_member(server_id, user_id, ServerRole::Member);

        // D'abord ajouter une réaction
        let add_use_case = AddReactionUseCase::new(mock_message_repo.clone(), mock_channel_repo.clone(), mock_server_repo.clone());
        add_use_case.execute("msg_1".to_string(), user_id, "👍".to_string()).await.unwrap();

        // Puis la retirer
        let remove_use_case = RemoveReactionUseCase::new(mock_message_repo, mock_channel_repo, mock_server_repo);
        let result = remove_use_case.execute("msg_1".to_string(), user_id, "👍".to_string()).await;

        assert!(result.is_ok());
        let updated = result.unwrap();
        assert!(!updated.reactions.contains_key("👍"));
    }

    #[tokio::test]
    async fn test_update_message_not_found() {
        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let channel = Channel::new(server_id, "General".to_string());

        let mock_message_repo = MockMessageRepository::new();
        let mock_channel_repo = MockChannelRepository::new().with_channel(channel.clone());
        let mock_server_repo = MockServerRepository::new().with_member(server_id, user_id, ServerRole::Member);

        let use_case = UpdateMessageUseCase::new(mock_message_repo, mock_channel_repo, mock_server_repo);
        let result = use_case.execute("nonexistent".to_string(), user_id, "New content".to_string()).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::NotFound(_) => (),
            _ => panic!("Expected NotFound error"),
        }
    }

    // --- DeleteMessageUseCase (tests manquants) ---

    #[tokio::test]
    async fn test_delete_message_not_found() {
        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let channel = Channel::new(server_id, "General".to_string());

        let mock_message_repo = MockMessageRepository::new();
        let mock_channel_repo = MockChannelRepository::new().with_channel(channel.clone());
        let mock_server_repo = MockServerRepository::new().with_member(server_id, user_id, ServerRole::Member);

        let use_case = DeleteMessageUseCase::new(mock_message_repo, mock_channel_repo, mock_server_repo);
        let result = use_case.execute("nonexistent".to_string(), user_id).await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::NotFound(_)));
    }

    #[tokio::test]
    async fn test_delete_message_member_cannot_delete_others() {
        let owner_id = Uuid::new_v4();
        let other_user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let channel = Channel::new(server_id, "General".to_string());

        let message = Message::new(
            channel.id.to_string(),
            owner_id.to_string(),
            "Owner".to_string(),
            "Test message".to_string(),
        );

        let mock_message_repo = MockMessageRepository::new().with_message(message);
        let mock_channel_repo = MockChannelRepository::new().with_channel(channel.clone());
        let mock_server_repo = MockServerRepository::new()
            .with_member(server_id, owner_id, ServerRole::Member)
            .with_member(server_id, other_user_id, ServerRole::Member);

        let use_case = DeleteMessageUseCase::new(mock_message_repo, mock_channel_repo, mock_server_repo);
        let result = use_case.execute("msg_1".to_string(), other_user_id).await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::Forbidden(_)));
    }

    #[tokio::test]
    async fn test_delete_message_admin_can_delete_others() {
        let owner_id = Uuid::new_v4();
        let admin_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let channel = Channel::new(server_id, "General".to_string());

        let message = Message::new(
            channel.id.to_string(),
            owner_id.to_string(),
            "Owner".to_string(),
            "Test message".to_string(),
        );

        let mock_message_repo = MockMessageRepository::new().with_message(message);
        let mock_channel_repo = MockChannelRepository::new().with_channel(channel.clone());
        let mock_server_repo = MockServerRepository::new()
            .with_member(server_id, owner_id, ServerRole::Member)
            .with_member(server_id, admin_id, ServerRole::Admin);

        let use_case = DeleteMessageUseCase::new(mock_message_repo, mock_channel_repo, mock_server_repo);
        let result = use_case.execute("msg_1".to_string(), admin_id).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_message_server_owner_can_delete_others() {
        let member_id = Uuid::new_v4();
        let server_owner_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let channel = Channel::new(server_id, "General".to_string());

        let message = Message::new(
            channel.id.to_string(),
            member_id.to_string(),
            "Member".to_string(),
            "Test message".to_string(),
        );

        let mock_message_repo = MockMessageRepository::new().with_message(message);
        let mock_channel_repo = MockChannelRepository::new().with_channel(channel.clone());
        let mock_server_repo = MockServerRepository::new()
            .with_member(server_id, member_id, ServerRole::Member)
            .with_member(server_id, server_owner_id, ServerRole::Owner);

        let use_case = DeleteMessageUseCase::new(mock_message_repo, mock_channel_repo, mock_server_repo);
        let result = use_case.execute("msg_1".to_string(), server_owner_id).await;

        assert!(result.is_ok());
    }

    // --- UpdateMessageUseCase (tests manquants) ---

    #[tokio::test]
    async fn test_update_message_not_member() {
        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let channel = Channel::new(server_id, "General".to_string());

        let message = Message::new(
            channel.id.to_string(),
            user_id.to_string(),
            "TestUser".to_string(),
            "Original content".to_string(),
        );

        let mock_message_repo = MockMessageRepository::new().with_message(message);
        let mock_channel_repo = MockChannelRepository::new().with_channel(channel.clone());
        let mock_server_repo = MockServerRepository::new(); // user n'est pas membre

        let use_case = UpdateMessageUseCase::new(mock_message_repo, mock_channel_repo, mock_server_repo);
        let result = use_case.execute("msg_1".to_string(), user_id, "Updated".to_string()).await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::Unauthorized(_)));
    }

    // --- AddReactionUseCase (tests manquants) ---

    #[tokio::test]
    async fn test_add_reaction_message_not_found() {
        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let channel = Channel::new(server_id, "General".to_string());

        let mock_message_repo = MockMessageRepository::new();
        let mock_channel_repo = MockChannelRepository::new().with_channel(channel.clone());
        let mock_server_repo = MockServerRepository::new().with_member(server_id, user_id, ServerRole::Member);

        let use_case = AddReactionUseCase::new(mock_message_repo, mock_channel_repo, mock_server_repo);
        let result = use_case.execute("nonexistent".to_string(), user_id, "👍".to_string()).await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::NotFound(_)));
    }

    #[tokio::test]
    async fn test_add_reaction_multiple_users() {
        let user1_id = Uuid::new_v4();
        let user2_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let channel = Channel::new(server_id, "General".to_string());
        let message = Message::new(
            channel.id.to_string(),
            user1_id.to_string(),
            "TestUser".to_string(),
            "Hello".to_string(),
        );

        let mock_message_repo = MockMessageRepository::new().with_message(message);
        let mock_channel_repo = MockChannelRepository::new().with_channel(channel.clone());
        let mock_server_repo = MockServerRepository::new()
            .with_member(server_id, user1_id, ServerRole::Member)
            .with_member(server_id, user2_id, ServerRole::Member);

        let use_case = AddReactionUseCase::new(mock_message_repo.clone(), mock_channel_repo.clone(), mock_server_repo.clone());
        use_case.execute("msg_1".to_string(), user1_id, "👍".to_string()).await.unwrap();

        let use_case2 = AddReactionUseCase::new(mock_message_repo, mock_channel_repo, mock_server_repo);
        let result = use_case2.execute("msg_1".to_string(), user2_id, "👍".to_string()).await;

        assert!(result.is_ok());
        let updated = result.unwrap();
        assert_eq!(updated.reactions["👍"].len(), 2);
    }

    // --- RemoveReactionUseCase (tests manquants) ---

    #[tokio::test]
    async fn test_remove_reaction_not_member() {
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
        let mock_server_repo = MockServerRepository::new(); // pas de membre

        let use_case = RemoveReactionUseCase::new(mock_message_repo, mock_channel_repo, mock_server_repo);
        let result = use_case.execute("msg_1".to_string(), user_id, "👍".to_string()).await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::Unauthorized(_)));
    }

    #[tokio::test]
    async fn test_remove_reaction_message_not_found() {
        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let channel = Channel::new(server_id, "General".to_string());

        let mock_message_repo = MockMessageRepository::new();
        let mock_channel_repo = MockChannelRepository::new().with_channel(channel.clone());
        let mock_server_repo = MockServerRepository::new().with_member(server_id, user_id, ServerRole::Member);

        let use_case = RemoveReactionUseCase::new(mock_message_repo, mock_channel_repo, mock_server_repo);
        let result = use_case.execute("nonexistent".to_string(), user_id, "👍".to_string()).await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::NotFound(_)));
    }
}
