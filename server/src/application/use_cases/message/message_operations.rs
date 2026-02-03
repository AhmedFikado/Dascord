use crate::application::dto::message_dto::MessageDto;
use crate::infrastructure::repositories::{MessageRepository, ChannelRepository, ServerRepository};
use crate::domain::entities::Message;
use crate::utils::error::{AppError, AppResult};
use uuid::Uuid;

pub struct SendMessageUseCase<MR: MessageRepository, CR: ChannelRepository, SR: ServerRepository> {
    message_repo: MR,
    channel_repo: CR,
    server_repo: SR,
}

impl<MR: MessageRepository, CR: ChannelRepository, SR: ServerRepository> SendMessageUseCase<MR, CR, SR> {
    pub fn new(message_repo: MR, channel_repo: CR, server_repo: SR) -> Self {
        Self { message_repo, channel_repo, server_repo }
    }

    pub async fn execute(&self, channel_id: Uuid, user_id: Uuid, username: String, content: String) -> AppResult<MessageDto> {
        let channel = self.channel_repo.find_by_id(channel_id).await?
            .ok_or_else(|| AppError::NotFound("Channel not found".to_string()))?;
        
        let is_member = self.server_repo.is_member(channel.server_id, user_id).await?;
        if !is_member {
            return Err(AppError::Unauthorized("Not a member of this server".to_string()));
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

pub struct GetMessageHistoryUseCase<MR: MessageRepository, CR: ChannelRepository, SR: ServerRepository> {
    message_repo: MR,
    channel_repo: CR,
    server_repo: SR,
}

impl<MR: MessageRepository, CR: ChannelRepository, SR: ServerRepository> GetMessageHistoryUseCase<MR, CR, SR> {
    pub fn new(message_repo: MR, channel_repo: CR, server_repo: SR) -> Self {
        Self { message_repo, channel_repo, server_repo }
    }

    pub async fn execute(&self, channel_id: Uuid, user_id: Uuid, limit: i64) -> AppResult<Vec<MessageDto>> {
        let channel = self.channel_repo.find_by_id(channel_id).await?
            .ok_or_else(|| AppError::NotFound("Channel not found".to_string()))?;
        
        let is_member = self.server_repo.is_member(channel.server_id, user_id).await?;
        if !is_member {
            return Err(AppError::Unauthorized("Not a member of this server".to_string()));
        }

        let messages = self.message_repo.find_by_channel(&channel_id.to_string(), limit).await?;
        
        Ok(messages.into_iter().map(|m| MessageDto {
            id: m.id.map(|id| id.to_string()),
            channel_id: m.channel_id,
            user_id: m.user_id,
            username: m.username,
            content: m.content,
            created_at: m.created_at.to_rfc3339(),
        }).collect())
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
