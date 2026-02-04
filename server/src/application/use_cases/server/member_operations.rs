use crate::application::dto::server::MemberResponse;
use crate::infrastructure::repositories::server_repository::ServerRepository;
use crate::domain::value_objects::ServerRole;
use crate::utils::error::{AppError, AppResult};
use uuid::Uuid;

pub struct ListMembersUseCase<R: ServerRepository> {
    server_repo: R,
}

impl<R: ServerRepository> ListMembersUseCase<R> {
    pub fn new(server_repo: R) -> Self {
        Self { server_repo }
    }

    pub async fn execute(&self, server_id: Uuid, user_id: Uuid) -> AppResult<Vec<MemberResponse>> {
        let is_member = self.server_repo.is_member(server_id, user_id).await?;
        if !is_member {
            return Err(AppError::Unauthorized("Not a member of this server".to_string()));
        }

        let members = self.server_repo.get_members(server_id).await?;
        Ok(members.into_iter().map(MemberResponse::from).collect())
    }
}

pub struct UpdateMemberRoleUseCase<R: ServerRepository> {
    server_repo: R,
}

impl<R: ServerRepository> UpdateMemberRoleUseCase<R> {
    pub fn new(server_repo: R) -> Self {
        Self { server_repo }
    }

    pub async fn execute(&self, server_id: Uuid, target_user_id: Uuid, requester_id: Uuid, new_role: ServerRole) -> AppResult<()> {
        let server = self.server_repo.find_by_id(server_id).await?
            .ok_or_else(|| AppError::NotFound("Server not found".to_string()))?;

        if server.owner_id != requester_id {
            let requester_role = self.server_repo.get_member_role(server_id, requester_id).await?
                .ok_or_else(|| AppError::Unauthorized("Not a member".to_string()))?;
            
            if requester_role != ServerRole::Admin {
                return Err(AppError::Unauthorized("Only owner or admin can update roles".to_string()));
            }
        }

        if new_role == ServerRole::Owner {
            return Err(AppError::ValidationError("Cannot assign owner role".to_string()));
        }

        self.server_repo.update_member_role(server_id, target_user_id, new_role).await
    }
}
