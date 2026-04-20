use crate::application::dto::message::MessageDto;
use crate::domain::entities::channel::PrivateChannel;
use crate::domain::entities::message::Message;
use crate::infrastructure::repositories::channel::PrivateChannelRepository;
use crate::infrastructure::repositories::message::MessageRepository;
use crate::utils::error::{AppError, AppResult};
use uuid::Uuid;

pub struct PrivateMessageServices<
    MR: MessageRepository,
    PCR: PrivateChannelRepository,
> {
    message_repo: MR,
    private_channel_repo: PCR,
}

impl<MR: MessageRepository, PCR: PrivateChannelRepository>
    PrivateMessageServices<MR, PCR>
{
    pub fn new(
        message_repo: MR,
        private_channel_repo: PCR,
    ) -> Self {
        Self {
            message_repo,
            private_channel_repo,
        }
    }

    pub async fn send_message(
        &self,
        channel_id: Uuid,
        user_id: Uuid,
        username: String,
        content: String,
    ) -> AppResult<MessageDto> {
        let _channel = self
            .private_channel_repo
            .get_by_id(channel_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Channel not found".to_string()))?;

        let message = Message::new(
            channel_id.to_string(),
            user_id.to_string(),
            username.clone(),
            content.clone(),
        );

        let created = self.message_repo.create(message).await?;

        self.private_channel_repo
            .update_last_message_at(channel_id, user_id, created.created_at)
            .await?;

        Ok(MessageDto::from(created))
    }

    pub async fn get_channel(&self, channel_id: Uuid) -> AppResult<Option<PrivateChannel>> {
        self.private_channel_repo.get_by_id(channel_id).await
    }

    pub async fn get_message_history(
        &self,
        channel_id: Uuid,
        user_id: Uuid,
    ) -> AppResult<Vec<MessageDto>> {
        let channel = self
            .private_channel_repo
            .get_by_id(channel_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Channel not found".to_string()))?;

        // Check si le user est un des participants du channel
        let is_participant = channel.user1 == user_id || channel.user2 == user_id;
        if !is_participant {
            return Err(AppError::Unauthorized(
                "You are not a participant of this private channel".to_string(),
            ));
        }

        let messages = self
            .message_repo
            .find_by_channel(&channel_id.to_string())
            .await?;

        Ok(messages
            .into_iter()
            .map(|m| MessageDto::from(m))
            .collect())
    }


    pub async fn delete_message(&self, message_id: String, user_id: Uuid) -> AppResult<String> {
        // Récupérer le message
        let message = self
            .message_repo
            .find_by_id(&message_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Message not found".to_string()))?;

        // Conserver le channel_id avant suppression
        let channel_id = message.channel_id.clone();

        //  Vérifier si l'utilisateur est le propriétaire du message
        let message_owner_id = Uuid::parse_str(&message.user_id)
            .map_err(|_| AppError::InternalServerError("Invalid user ID in message".to_string()))?;

        let is_message_owner = message_owner_id == user_id;
        if !is_message_owner {
            return Err(AppError::Forbidden(
                "You can only delete your own messages".to_string(),
            ));
        }

        // Check si le user est un des participants du channel
        let channel = self
            .private_channel_repo
            .get_by_id(Uuid::parse_str(&channel_id).map_err(|_| AppError::InternalServerError("Invalid channel ID".to_string()))?)
            .await?
            .ok_or_else(|| AppError::NotFound("Channel not found".to_string()))?;

        let is_participant = channel.user1 == user_id || channel.user2 == user_id;
        if !is_participant {
            return Err(AppError::Unauthorized(
                "You are not a participant of this private channel".to_string(),
            ));
        }
        
        // Supprimer le message
        self.message_repo.delete(&message_id).await?;

        // Retourner le channel_id pour la diffusion WebSocket
        Ok(channel_id)
    }

    pub async fn update_private_message(
        &self,
        message_id: String,
        user_id: Uuid,
        new_content: String,
    ) -> AppResult<MessageDto> {
        // Récupérer le message
        let message = self
            .message_repo
            .find_by_id(&message_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Message not found".to_string()))?;

        // Vérifier si l'utilisateur est le propriétaire du message
        let message_owner_id = Uuid::parse_str(&message.user_id)
            .map_err(|_| AppError::InternalServerError("Invalid user ID in message".to_string()))?;

        if message_owner_id != user_id {
            return Err(AppError::Forbidden(
                "You can only edit your own messages".to_string(),
            ));
        }

        // Check si le user est un des participants du channel
        let channel = self
            .private_channel_repo
            .get_by_id(Uuid::parse_str(&message.channel_id).map_err(|_| AppError::InternalServerError("Invalid channel ID".to_string()))?)
            .await?
            .ok_or_else(|| AppError::NotFound("Channel not found".to_string()))?;

        let is_participant = channel.user1 == user_id || channel.user2 == user_id;
        if !is_participant {
            return Err(AppError::Unauthorized(
                "You are not a participant of this private channel".to_string(),
            ));
        }

        // Mettre à jour le message
        let updated_message = self.message_repo.update(&message_id, new_content).await?;

        Ok(MessageDto::from(updated_message))
    }
    pub async fn add_reaction(
        &self,
        message_id: String,
        user_id: Uuid,
        reaction: String,
    ) -> AppResult<MessageDto> {
        let message = self
            .message_repo
            .find_by_id(&message_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Message not found".to_string()))?;

        let channel = self
            .private_channel_repo
            .get_by_id(Uuid::parse_str(&message.channel_id).map_err(|_| AppError::InternalServerError("Invalid channel ID".to_string()))?)
            .await?
            .ok_or_else(|| AppError::NotFound("Channel not found".to_string()))?;

        let is_participant = channel.user1 == user_id || channel.user2 == user_id;
        if !is_participant {
            return Err(AppError::Unauthorized(
                "You are not a participant of this private channel".to_string(),
            ));
        }

        let updated = self.message_repo.add_reaction(&message_id, reaction, user_id.to_string()).await?;
        Ok(MessageDto::from(updated))
    }

    pub async fn remove_reaction(
        &self,
        message_id: String,
        user_id: Uuid,
        reaction: String,
    ) -> AppResult<MessageDto> {
        let message = self
            .message_repo
            .find_by_id(&message_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Message not found".to_string()))?;

        let channel = self
            .private_channel_repo
            .get_by_id(Uuid::parse_str(&message.channel_id).map_err(|_| AppError::InternalServerError("Invalid channel ID".to_string()))?)
            .await?
            .ok_or_else(|| AppError::NotFound("Channel not found".to_string()))?;

        let is_participant = channel.user1 == user_id || channel.user2 == user_id;
        if !is_participant {
            return Err(AppError::Unauthorized(
                "You are not a participant of this private channel".to_string(),
            ));
        }

        let updated = self.message_repo.remove_reaction(&message_id, reaction, user_id.to_string()).await?;
        Ok(MessageDto::from(updated))
    }
}



// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::channel::PrivateChannel;
    use crate::domain::entities::message::Message;
    use crate::infrastructure::repositories::mocks::mock_message_repository::MockMessageRepository;
    use crate::infrastructure::repositories::mocks::mock_private_channel_repository::MockPrivateChannelRepository;

    fn make_channel(user_id: Uuid) -> PrivateChannel {
        PrivateChannel {
            id: Uuid::new_v4(),
            user1: user_id,
            user2: Uuid::new_v4(),
            created_at: chrono::Utc::now(),
        }
    }

    fn make_message(channel_id: Uuid, user_id: Uuid) -> Message {
        Message::new(
            channel_id.to_string(),
            user_id.to_string(),
            "testuser".to_string(),
            "Hello".to_string(),
        )
    }

    fn make_service(
        msg_repo: MockMessageRepository,
        channel_repo: MockPrivateChannelRepository,
    ) -> PrivateMessageServices<MockMessageRepository, MockPrivateChannelRepository> {
        PrivateMessageServices::new(msg_repo, channel_repo)
    }

    // --- send_message ---

    #[tokio::test]
    async fn test_send_message_success() {
        let user_id = Uuid::new_v4();
        let channel = make_channel(user_id);
        let channel_id = channel.id;
        let service = make_service(
            MockMessageRepository::new(),
            MockPrivateChannelRepository::with_channels(vec![channel]),
        );

        let result = service
            .send_message(channel_id, user_id, "testuser".to_string(), "Hello".to_string())
            .await;

        assert!(result.is_ok());
        let dto = result.unwrap();
        assert_eq!(dto.content, "Hello");
        assert_eq!(dto.username, "testuser");
        assert_eq!(dto.channel_id, channel_id.to_string());
    }

    #[tokio::test]
    async fn test_send_message_channel_not_found() {
        let user_id = Uuid::new_v4();
        let service = make_service(
            MockMessageRepository::new(),
            MockPrivateChannelRepository::new(),
        );

        let result = service
            .send_message(Uuid::new_v4(), user_id, "testuser".to_string(), "Hello".to_string())
            .await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, AppError::NotFound(_)));
    }

    // --- get_message_history ---

    #[tokio::test]
    async fn test_get_message_history_success_empty() {
        let user_id = Uuid::new_v4();
        let channel = make_channel(user_id);
        let channel_id = channel.id;
        let service = make_service(
            MockMessageRepository::new(),
            MockPrivateChannelRepository::with_channels(vec![channel]),
        );

        let result = service.get_message_history(channel_id, user_id).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_get_message_history_success_with_messages() {
        let user_id = Uuid::new_v4();
        let channel = make_channel(user_id);
        let channel_id = channel.id;
        let message = make_message(channel_id, user_id);
        let service = make_service(
            MockMessageRepository::new().with_message(message),
            MockPrivateChannelRepository::with_channels(vec![channel]),
        );

        let result = service.get_message_history(channel_id, user_id).await;

        assert!(result.is_ok());
        let messages = result.unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].content, "Hello");
    }

    #[tokio::test]
    async fn test_get_message_history_channel_not_found() {
        let user_id = Uuid::new_v4();
        let service = make_service(
            MockMessageRepository::new(),
            MockPrivateChannelRepository::new(),
        );

        let result = service.get_message_history(Uuid::new_v4(), user_id).await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::NotFound(_)));
    }

    #[tokio::test]
    async fn test_get_message_history_not_participant() {
        let user_id = Uuid::new_v4();
        let other_user = Uuid::new_v4();
        let channel = PrivateChannel {
            id: Uuid::new_v4(),
            user1: Uuid::new_v4(),
            user2: Uuid::new_v4(),
            created_at: chrono::Utc::now(),
        };
        let channel_id = channel.id;
        let service = make_service(
            MockMessageRepository::new(),
            MockPrivateChannelRepository::with_channels(vec![channel]),
        );

        let result = service.get_message_history(channel_id, user_id).await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::Unauthorized(_)));

        // also verify the other_user (not in channel either) gets same error
        let channel2 = PrivateChannel {
            id: Uuid::new_v4(),
            user1: Uuid::new_v4(),
            user2: Uuid::new_v4(),
            created_at: chrono::Utc::now(),
        };
        let channel2_id = channel2.id;
        let service2 = make_service(
            MockMessageRepository::new(),
            MockPrivateChannelRepository::with_channels(vec![channel2]),
        );
        let result2 = service2.get_message_history(channel2_id, other_user).await;
        assert!(result2.is_err());
    }

    // --- delete_message ---

    #[tokio::test]
    async fn test_delete_message_success() {
        let user_id = Uuid::new_v4();
        let channel = make_channel(user_id);
        let channel_id = channel.id;
        let message = make_message(channel_id, user_id);
        let service = make_service(
            MockMessageRepository::new().with_message(message),
            MockPrivateChannelRepository::with_channels(vec![channel]),
        );

        let result = service.delete_message("msg_1".to_string(), user_id).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), channel_id.to_string());
    }

    #[tokio::test]
    async fn test_delete_message_not_found() {
        let user_id = Uuid::new_v4();
        let service = make_service(
            MockMessageRepository::new(),
            MockPrivateChannelRepository::new(),
        );

        let result = service.delete_message("nonexistent".to_string(), user_id).await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::NotFound(_)));
    }

    #[tokio::test]
    async fn test_delete_message_not_owner() {
        let owner_id = Uuid::new_v4();
        let other_user_id = Uuid::new_v4();
        let channel = PrivateChannel {
            id: Uuid::new_v4(),
            user1: other_user_id,
            user2: owner_id,
            created_at: chrono::Utc::now(),
        };
        let channel_id = channel.id;
        let message = make_message(channel_id, owner_id);
        let service = make_service(
            MockMessageRepository::new().with_message(message),
            MockPrivateChannelRepository::with_channels(vec![channel]),
        );

        let result = service.delete_message("msg_1".to_string(), other_user_id).await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::Forbidden(_)));
    }

    #[tokio::test]
    async fn test_delete_message_not_participant() {
        let owner_id = Uuid::new_v4();
        let outsider_id = Uuid::new_v4();
        // Channel does not include outsider_id
        let channel = PrivateChannel {
            id: Uuid::new_v4(),
            user1: owner_id,
            user2: Uuid::new_v4(),
            created_at: chrono::Utc::now(),
        };
        let channel_id = channel.id;
        let message = make_message(channel_id, outsider_id);
        let service = make_service(
            MockMessageRepository::new().with_message(message),
            MockPrivateChannelRepository::with_channels(vec![channel]),
        );

        // outsider_id owns the message but is not in the channel
        let result = service.delete_message("msg_1".to_string(), outsider_id).await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::Unauthorized(_)));
    }

    // --- update_private_message ---

    #[tokio::test]
    async fn test_update_private_message_success() {
        let user_id = Uuid::new_v4();
        let channel = make_channel(user_id);
        let channel_id = channel.id;
        let message = make_message(channel_id, user_id);
        let service = make_service(
            MockMessageRepository::new().with_message(message),
            MockPrivateChannelRepository::with_channels(vec![channel]),
        );

        let result = service
            .update_private_message("msg_1".to_string(), user_id, "Updated".to_string())
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().content, "Updated");
    }

    #[tokio::test]
    async fn test_update_private_message_not_found() {
        let user_id = Uuid::new_v4();
        let service = make_service(
            MockMessageRepository::new(),
            MockPrivateChannelRepository::new(),
        );

        let result = service
            .update_private_message("nonexistent".to_string(), user_id, "Updated".to_string())
            .await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::NotFound(_)));
    }

    #[tokio::test]
    async fn test_update_private_message_not_owner() {
        let owner_id = Uuid::new_v4();
        let other_user_id = Uuid::new_v4();
        let channel = PrivateChannel {
            id: Uuid::new_v4(),
            user1: other_user_id,
            user2: owner_id,
            created_at: chrono::Utc::now(),
        };
        let channel_id = channel.id;
        let message = make_message(channel_id, owner_id);
        let service = make_service(
            MockMessageRepository::new().with_message(message),
            MockPrivateChannelRepository::with_channels(vec![channel]),
        );

        let result = service
            .update_private_message("msg_1".to_string(), other_user_id, "Hacked".to_string())
            .await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::Forbidden(_)));
    }

    #[tokio::test]
    async fn test_update_private_message_not_participant() {
        let owner_id = Uuid::new_v4();
        // Channel does not include owner_id
        let channel = PrivateChannel {
            id: Uuid::new_v4(),
            user1: Uuid::new_v4(),
            user2: Uuid::new_v4(),
            created_at: chrono::Utc::now(),
        };
        let channel_id = channel.id;
        let message = make_message(channel_id, owner_id);
        let service = make_service(
            MockMessageRepository::new().with_message(message),
            MockPrivateChannelRepository::with_channels(vec![channel]),
        );

        let result = service
            .update_private_message("msg_1".to_string(), owner_id, "Updated".to_string())
            .await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::Unauthorized(_)));
    }

    // --- add_reaction ---

    #[tokio::test]
    async fn test_add_reaction_success() {
        let user_id = Uuid::new_v4();
        let channel = make_channel(user_id);
        let channel_id = channel.id;
        let message = make_message(channel_id, user_id);
        let service = make_service(
            MockMessageRepository::new().with_message(message),
            MockPrivateChannelRepository::with_channels(vec![channel]),
        );

        let result = service
            .add_reaction("msg_1".to_string(), user_id, "+1".to_string())
            .await;

        assert!(result.is_ok());
        let dto = result.unwrap();
        assert!(dto.reactions.contains_key("+1"));
        assert!(dto.reactions["+1"].contains(&user_id.to_string()));
    }

    #[tokio::test]
    async fn test_add_reaction_message_not_found() {
        let user_id = Uuid::new_v4();
        let service = make_service(
            MockMessageRepository::new(),
            MockPrivateChannelRepository::new(),
        );

        let result = service
            .add_reaction("nonexistent".to_string(), user_id, "+1".to_string())
            .await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::NotFound(_)));
    }

    #[tokio::test]
    async fn test_add_reaction_not_participant() {
        let owner_id = Uuid::new_v4();
        let outsider_id = Uuid::new_v4();
        let channel = PrivateChannel {
            id: Uuid::new_v4(),
            user1: owner_id,
            user2: Uuid::new_v4(),
            created_at: chrono::Utc::now(),
        };
        let channel_id = channel.id;
        let message = make_message(channel_id, owner_id);
        let service = make_service(
            MockMessageRepository::new().with_message(message),
            MockPrivateChannelRepository::with_channels(vec![channel]),
        );

        let result = service
            .add_reaction("msg_1".to_string(), outsider_id, "+1".to_string())
            .await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::Unauthorized(_)));
    }

    // --- remove_reaction ---

    #[tokio::test]
    async fn test_remove_reaction_success() {
        let user_id = Uuid::new_v4();
        let channel = make_channel(user_id);
        let channel_id = channel.id;
        let mut message = make_message(channel_id, user_id);
        message.reactions.insert("+1".to_string(), vec![user_id.to_string()]);
        let service = make_service(
            MockMessageRepository::new().with_message(message),
            MockPrivateChannelRepository::with_channels(vec![channel]),
        );

        let result = service
            .remove_reaction("msg_1".to_string(), user_id, "+1".to_string())
            .await;

        assert!(result.is_ok());
        let dto = result.unwrap();
        assert!(!dto.reactions.contains_key("+1"));
    }

    #[tokio::test]
    async fn test_remove_reaction_message_not_found() {
        let user_id = Uuid::new_v4();
        let service = make_service(
            MockMessageRepository::new(),
            MockPrivateChannelRepository::new(),
        );

        let result = service
            .remove_reaction("nonexistent".to_string(), user_id, "+1".to_string())
            .await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::NotFound(_)));
    }

    #[tokio::test]
    async fn test_remove_reaction_not_participant() {
        let owner_id = Uuid::new_v4();
        let outsider_id = Uuid::new_v4();
        let channel = PrivateChannel {
            id: Uuid::new_v4(),
            user1: owner_id,
            user2: Uuid::new_v4(),
            created_at: chrono::Utc::now(),
        };
        let channel_id = channel.id;
        let mut message = make_message(channel_id, owner_id);
        message.reactions.insert("+1".to_string(), vec![owner_id.to_string()]);
        let service = make_service(
            MockMessageRepository::new().with_message(message),
            MockPrivateChannelRepository::with_channels(vec![channel]),
        );

        let result = service
            .remove_reaction("msg_1".to_string(), outsider_id, "+1".to_string())
            .await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::Unauthorized(_)));
    }
}
