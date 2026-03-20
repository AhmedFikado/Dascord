use crate::application::dto::message::MessageDto;
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
        let result = MessageDto::from(created);

        Ok(result)
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

