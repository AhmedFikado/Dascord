use crate::application::dto::channel::ChannelResponse;
use crate::infrastructure::repositories::{ChannelRepository, ServerRepository};
use crate::utils::error::{AppError, AppResult};
use uuid::Uuid;

pub struct GetChannelInfoUseCase<CR: ChannelRepository, SR: ServerRepository> {
    channel_repo: CR,
    server_repo: SR,
}

impl<CR: ChannelRepository, SR: ServerRepository> GetChannelInfoUseCase<CR, SR> {
    pub fn new(channel_repo: CR, server_repo: SR) -> Self {
        Self { channel_repo, server_repo }
    }

    pub async fn execute(&self, channel_id: Uuid, user_id: Uuid) -> AppResult<ChannelResponse> {
        let channel = self.channel_repo.find_by_id(channel_id).await?
            .ok_or_else(|| AppError::NotFound("Channel not found".to_string()))?;
        
        let is_member = self.server_repo.is_member(channel.server_id, user_id).await?;
        if !is_member {
            return Err(AppError::Unauthorized("Not a member of this server".to_string()));
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
        Self { channel_repo, server_repo }
    }

    pub async fn execute(&self, channel_id: Uuid, user_id: Uuid, name: String) -> AppResult<ChannelResponse> {
        let mut channel = self.channel_repo.find_by_id(channel_id).await?
            .ok_or_else(|| AppError::NotFound("Channel not found".to_string()))?;
        
        let role = self.server_repo.get_member_role(channel.server_id, user_id).await?
            .ok_or_else(|| AppError::Unauthorized("Not a member".to_string()))?;
        
        if !role.can_have_permissions() {
            return Err(AppError::Unauthorized("Insufficient permissions".to_string()));
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
        Self { channel_repo, server_repo }
    }

    pub async fn execute(&self, channel_id: Uuid, user_id: Uuid) -> AppResult<()> {
        let channel = self.channel_repo.find_by_id(channel_id).await?
            .ok_or_else(|| AppError::NotFound("Channel not found".to_string()))?;
        
        let role = self.server_repo.get_member_role(channel.server_id, user_id).await?
            .ok_or_else(|| AppError::Unauthorized("Not a member".to_string()))?;
        
        if !role.can_have_permissions() {
            return Err(AppError::Unauthorized("Insufficient permissions".to_string()));
        }

        self.channel_repo.delete(channel_id).await
    }
}
