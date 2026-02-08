use crate::application::dto::server::{MemberResponse, UserInfo};
use crate::domain::value_objects::ServerRole;
use crate::infrastructure::repositories::server_repository::ServerRepository;
use crate::infrastructure::repositories::user_repository::UserRepository;
use crate::utils::error::{AppError, AppResult};
use uuid::Uuid;

pub struct ListMembersUseCase<SR: ServerRepository, UR: UserRepository> {
    server_repo: SR,
    user_repo: UR,
}

impl<SR: ServerRepository, UR: UserRepository> ListMembersUseCase<SR, UR> {
    pub fn new(server_repo: SR, user_repo: UR) -> Self {
        Self { server_repo, user_repo }
    }

    pub async fn execute(&self, server_id: Uuid, user_id: Uuid) -> AppResult<Vec<MemberResponse>> {
        let is_member = self.server_repo.is_member(server_id, user_id).await?;
        if !is_member {
            return Err(AppError::Unauthorized(
                "Not a member of this server".to_string(),
            ));
        }

        let members = self.server_repo.get_members(server_id).await?;
        
        let mut responses = Vec::new();
        for (srv_id, usr_id, role, joined_at) in members {
            let user = self.user_repo.find_by_id(usr_id).await?
                .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;
            
            responses.push(MemberResponse {
                server_id: srv_id.to_string(),
                user_id: usr_id.to_string(),
                role,
                joined_at: joined_at.to_rfc3339(),
                user: UserInfo {
                    id: user.id.to_string(),
                    username: user.username,
                    email: user.email,
                    status: user.status,
                    created_at: user.created_at.to_rfc3339(),
                },
            });
        }
        
        Ok(responses)
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
        let mut server = self.server_repo.find_by_id(server_id).await?
            .ok_or_else(|| AppError::NotFound("Server not found".to_string()))?;

        let target_role = self.server_repo.get_member_role(server_id, target_user_id).await?
            .ok_or_else(|| AppError::NotFound("Target user is not a member".to_string()))?;

        if server.owner_id != requester_id {
            let requester_role = self
                .server_repo
                .get_member_role(server_id, requester_id)
                .await?
                .ok_or_else(|| AppError::Unauthorized("Not a member".to_string()))?;

            if requester_role != ServerRole::Admin {
                return Err(AppError::Unauthorized(
                    "Only owner or admin can update roles".to_string(),
                ));
            }
            
            if new_role == ServerRole::Owner {
                return Err(AppError::Unauthorized("Only owner can transfer ownership".to_string()));
            }
            
            if target_role == ServerRole::Owner || target_role == ServerRole::Admin {
                return Err(AppError::Unauthorized("Admins cannot modify owner or other admin roles".to_string()));
            }
            
            if new_role == ServerRole::Owner {
                return Err(AppError::Unauthorized("Only owner can transfer ownership".to_string()));
            }
            
            if target_role == ServerRole::Owner || target_role == ServerRole::Admin {
                return Err(AppError::Unauthorized("Admins cannot modify owner or other admin roles".to_string()));
            }
        }

        if new_role == ServerRole::Owner {
            if server.owner_id != requester_id {
                return Err(AppError::Unauthorized("Only current owner can transfer ownership".to_string()));
            }
            
            server.owner_id = target_user_id;
            self.server_repo.update(server).await?;
            self.server_repo.update_member_role(server_id, requester_id, ServerRole::Admin).await?;
            self.server_repo.update_member_role(server_id, target_user_id, ServerRole::Owner).await?;
        } else {
            self.server_repo.update_member_role(server_id, target_user_id, new_role).await?;
        }

        Ok(())
    }
}
