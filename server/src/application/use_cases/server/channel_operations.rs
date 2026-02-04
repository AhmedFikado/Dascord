use crate::application::dto::channel::{CreateChannelRequest, ChannelResponse};
use crate::infrastructure::repositories::{server_repository::ServerRepository, channel_repository::ChannelRepository};
use crate::domain::entities::Channel;
use crate::utils::error::{AppError, AppResult};
use uuid::Uuid;
use validator::Validate;

pub struct GetChannelsUseCase<SR: ServerRepository, CR: ChannelRepository> {
    server_repo: SR,
    channel_repo: CR,
}

impl<SR: ServerRepository, CR: ChannelRepository> GetChannelsUseCase<SR, CR> {
    pub fn new(server_repo: SR, channel_repo: CR) -> Self {
        Self { server_repo, channel_repo }
    }

    pub async fn execute(&self, server_id: Uuid, user_id: Uuid) -> AppResult<Vec<ChannelResponse>> {
        let is_member = self.server_repo.is_member(server_id, user_id).await?;
        if !is_member {
            return Err(AppError::Unauthorized("Not a member of this server".to_string()));
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
        Self { server_repo, channel_repo }
    }

    pub async fn execute(&self, server_id: Uuid, _user_id: Uuid, request: CreateChannelRequest) -> AppResult<ChannelResponse> {
        request.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

        let _server = self.server_repo.find_by_id(server_id).await?
            .ok_or_else(|| AppError::NotFound("Server not found".to_string()))?;

        if _server.owner_id != _user_id {
            let role = self.server_repo.get_member_role(server_id, _user_id).await?
                .ok_or_else(|| AppError::Unauthorized("Not a member".to_string()))?;
            
            if !role.can_have_permissions() {
                return Err(AppError::Unauthorized("Insufficient permissions".to_string()));
            }
        }

        let channel = Channel::new(server_id, request.name);
        let created = self.channel_repo.create(channel).await?;
        
        Ok(ChannelResponse::from(created))
    }
}
