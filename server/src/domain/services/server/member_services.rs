use crate::application::dto::server_dto::{MemberResponse, UserInfo};
use crate::domain::entities::BanType;
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
                    avatar_id: user.avatar_id,
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

        // Empêcher l'owner de modifier son propre rôle sans transférer la propriété(sinon pb de sécurité)
        if server.owner_id == requester_id && target_user_id == requester_id && new_role != ServerRole::Owner {
            return Err(AppError::ValidationError(
                "Owner cannot change their own role. Transfer ownership to another member instead.".to_string(),
            ));
        }

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
pub struct BanMemberUseCase<R: ServerRepository> {
    server_repo: R,
}

impl<R: ServerRepository> BanMemberUseCase<R> {
    pub fn new(server_repo: R) -> Self {
        Self { server_repo }
    }

    pub async fn execute(&self, server_id: Uuid, target_user_id: Uuid, requester_id: Uuid, ban_type: BanType, expires_at: Option<chrono::DateTime<chrono::Utc>>) -> AppResult<()> {
        let server = self.server_repo.find_by_id(server_id).await?
            .ok_or_else(|| AppError::NotFound("Server not found".to_string()))?;

        if server.owner_id != requester_id {
            let requester_role = self
                .server_repo
                .get_member_role(server_id, requester_id)
                .await?
                .ok_or_else(|| AppError::Unauthorized("Not a member".to_string()))?;

            if requester_role != ServerRole::Admin {
                return Err(AppError::Unauthorized(
                    "Only owner or admin can ban members".to_string(),
                ));
            }
        }

        self.server_repo.ban_member(server_id, target_user_id, requester_id, ban_type, expires_at).await?;
        Ok(())
    }
}

pub struct ListBannedMembersUseCase<R: ServerRepository> {
    server_repo: R,
}

impl<R: ServerRepository> ListBannedMembersUseCase<R> {
    pub fn new(server_repo: R) -> Self {
        Self { server_repo }
    }

    pub async fn execute(&self, server_id: Uuid, requester_id: Uuid) -> AppResult<Vec<(Uuid, String, BanType, chrono::DateTime<chrono::Utc>, Option<chrono::DateTime<chrono::Utc>>)>> {
        let server = self.server_repo.find_by_id(server_id).await?
            .ok_or_else(|| AppError::NotFound("Server not found".to_string()))?;

        if server.owner_id != requester_id {
            let requester_role = self.server_repo.get_member_role(server_id, requester_id).await?
                .ok_or_else(|| AppError::Unauthorized("Not a member".to_string()))?;

            if requester_role != ServerRole::Admin {
                return Err(AppError::Unauthorized("Only owner or admin can view banned members".to_string()));
            }
        }

        self.server_repo.list_bans(server_id).await
    }
}

pub struct UnbanMemberUseCase<R: ServerRepository> {
    server_repo: R,
}

impl<R: ServerRepository> UnbanMemberUseCase<R> {
    pub fn new(server_repo: R) -> Self {
        Self { server_repo }
    }

    pub async fn execute(&self, server_id: Uuid, target_user_id: Uuid, requester_id: Uuid) -> AppResult<()> {
        let server = self.server_repo.find_by_id(server_id).await?
            .ok_or_else(|| AppError::NotFound("Server not found".to_string()))?;

        if server.owner_id != requester_id {
            let requester_role = self.server_repo.get_member_role(server_id, requester_id).await?
                .ok_or_else(|| AppError::Unauthorized("Not a member".to_string()))?;

            if requester_role != ServerRole::Admin {
                return Err(AppError::Unauthorized("Only owner or admin can unban members".to_string()));
            }
        }

        self.server_repo.unban_member(server_id, target_user_id).await
    }
}

pub struct KickMemberUseCase<R: ServerRepository> {
    server_repo: R,
}

impl<R: ServerRepository> KickMemberUseCase<R> {
        pub fn new(server_repo: R) -> Self {
            Self { server_repo }
        }

        pub async fn execute(&self, server_id: Uuid, target_user_id: Uuid, requester_id: Uuid,) -> AppResult<()> {
            let server = self.server_repo.find_by_id(server_id).await?
                .ok_or_else(|| AppError::NotFound("Server not found".to_string()))?;

            if server.owner_id != requester_id {
                let requester_role = self
                    .server_repo
                    .get_member_role(server_id, requester_id)
                    .await?
                    .ok_or_else(|| AppError::Unauthorized("Not a member".to_string()))?;

                if requester_role != ServerRole::Admin {
                    return Err(AppError::Unauthorized(
                        "Only owner or admin can kick members".to_string(),
                    ));
                }
            }

            self.server_repo.remove_member(server_id, target_user_id).await?;
            Ok(())
        }
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::{Server, User};
    use crate::infrastructure::repositories::mocks::{
        mock_server_repository::MockServerRepository,mock_user_repository::MockUserRepository,
    };
    use crate::infrastructure::security::PasswordService;

    #[tokio::test]
    async fn test_list_members_not_member() {
        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();

        let mock_server_repo = MockServerRepository::new();
        let mock_user_repo = MockUserRepository::new();

        let use_case = ListMembersUseCase::new(mock_server_repo, mock_user_repo);
        let result = use_case.execute(server_id, user_id).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_member_role_server_not_found() {
        let server_id = Uuid::new_v4();
        let target_id = Uuid::new_v4();
        let requester_id = Uuid::new_v4();

        let mock_server_repo = MockServerRepository::new();
        let use_case = UpdateMemberRoleUseCase::new(mock_server_repo);

        let result = use_case.execute(server_id, target_id, requester_id, ServerRole::Admin).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_member_role_target_not_member() {
        let owner_id = Uuid::new_v4();
        let target_id = Uuid::new_v4();
        let server = Server::new("Test".to_string(), owner_id, "CODE".to_string());

        let mock_server_repo = MockServerRepository::new().with_server(server.clone());
        let use_case = UpdateMemberRoleUseCase::new(mock_server_repo);

        let result = use_case.execute(server.id, target_id, owner_id, ServerRole::Admin).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_member_role_requester_not_member() {
        let owner_id = Uuid::new_v4();
        let target_id = Uuid::new_v4();
        let requester_id = Uuid::new_v4();
        let server = Server::new("Test".to_string(), owner_id, "CODE".to_string());

        let mock_server_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, target_id, ServerRole::Member);
        let use_case = UpdateMemberRoleUseCase::new(mock_server_repo);

        let result = use_case.execute(server.id, target_id, requester_id, ServerRole::Admin).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_member_role_member_cannot_update() {
        let owner_id = Uuid::new_v4();
        let target_id = Uuid::new_v4();
        let requester_id = Uuid::new_v4();
        let server = Server::new("Test".to_string(), owner_id, "CODE".to_string());

        let mock_server_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, target_id, ServerRole::Member)
            .with_member(server.id, requester_id, ServerRole::Member);
        let use_case = UpdateMemberRoleUseCase::new(mock_server_repo);

        let result = use_case.execute(server.id, target_id, requester_id, ServerRole::Admin).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_member_role_admin_cannot_transfer_ownership() {
        let owner_id = Uuid::new_v4();
        let target_id = Uuid::new_v4();
        let admin_id = Uuid::new_v4();
        let server = Server::new("Test".to_string(), owner_id, "CODE".to_string());

        let mock_server_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, target_id, ServerRole::Member)
            .with_member(server.id, admin_id, ServerRole::Admin);
        let use_case = UpdateMemberRoleUseCase::new(mock_server_repo);

        let result = use_case.execute(server.id, target_id, admin_id, ServerRole::Owner).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_member_role_admin_cannot_modify_owner() {
        let owner_id = Uuid::new_v4();
        let admin_id = Uuid::new_v4();
        let server = Server::new("Test".to_string(), owner_id, "CODE".to_string());

        let mock_server_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, owner_id, ServerRole::Owner)
            .with_member(server.id, admin_id, ServerRole::Admin);
        let use_case = UpdateMemberRoleUseCase::new(mock_server_repo);

        let result = use_case.execute(server.id, owner_id, admin_id, ServerRole::Member).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_member_role_admin_cannot_modify_admin() {
        let owner_id = Uuid::new_v4();
        let admin1_id = Uuid::new_v4();
        let admin2_id = Uuid::new_v4();
        let server = Server::new("Test".to_string(), owner_id, "CODE".to_string());

        let mock_server_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, admin1_id, ServerRole::Admin)
            .with_member(server.id, admin2_id, ServerRole::Admin);
        let use_case = UpdateMemberRoleUseCase::new(mock_server_repo);

        let result = use_case.execute(server.id, admin2_id, admin1_id, ServerRole::Member).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_member_role_admin_can_promote_member() {
        let owner_id = Uuid::new_v4();
        let admin_id = Uuid::new_v4();
        let member_id = Uuid::new_v4();
        let server = Server::new("Test".to_string(), owner_id, "CODE".to_string());

        let mock_server_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, admin_id, ServerRole::Admin)
            .with_member(server.id, member_id, ServerRole::Member);
        let use_case = UpdateMemberRoleUseCase::new(mock_server_repo);

        let result = use_case.execute(server.id, member_id, admin_id, ServerRole::Admin).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_member_role_owner_transfers_ownership() {
        let owner_id = Uuid::new_v4();
        let member_id = Uuid::new_v4();
        let server = Server::new("Test".to_string(), owner_id, "CODE".to_string());

        let mock_server_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, owner_id, ServerRole::Owner)
            .with_member(server.id, member_id, ServerRole::Member);
        let use_case = UpdateMemberRoleUseCase::new(mock_server_repo);

        let result = use_case.execute(server.id, member_id, owner_id, ServerRole::Owner).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_member_role_owner_updates_member() {
        let owner_id = Uuid::new_v4();
        let member_id = Uuid::new_v4();
        let server = Server::new("Test".to_string(), owner_id, "CODE".to_string());

        let mock_server_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, owner_id, ServerRole::Owner)
            .with_member(server.id, member_id, ServerRole::Member);
        let use_case = UpdateMemberRoleUseCase::new(mock_server_repo);

        let result = use_case.execute(server.id, member_id, owner_id, ServerRole::Admin).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_member_role_owner_cannot_change_own_role() {
        let owner_id = Uuid::new_v4();
        let server = Server::new("Test".to_string(), owner_id, "CODE".to_string());

        let mock_server_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, owner_id, ServerRole::Owner);
        let use_case = UpdateMemberRoleUseCase::new(mock_server_repo);

        let result = use_case.execute(server.id, owner_id, owner_id, ServerRole::Admin).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_kick_member_success() {
        let owner_id = Uuid::new_v4();
        let member_id = Uuid::new_v4();
        let server = Server::new("Test".to_string(), owner_id, "CODE".to_string());

        let mock_server_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, member_id, ServerRole::Member);
        let use_case = KickMemberUseCase::new(mock_server_repo);

        let result = use_case.execute(server.id, member_id, owner_id).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_kick_member_unauthorized() {
        let owner_id = Uuid::new_v4();
        let member_id = Uuid::new_v4();
        let requester_id = Uuid::new_v4();
        let server = Server::new("Test".to_string(), owner_id, "CODE".to_string());

        let mock_server_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, member_id, ServerRole::Member)
            .with_member(server.id, requester_id, ServerRole::Member);
        let use_case = KickMemberUseCase::new(mock_server_repo);

        let result = use_case.execute(server.id, member_id, requester_id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_ban_member_success() {
        let owner_id = Uuid::new_v4();
        let member_id = Uuid::new_v4();
        let server = Server::new("Test".to_string(), owner_id, "CODE".to_string());

        let mock_server_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, member_id, ServerRole::Member);
        let use_case = BanMemberUseCase::new(mock_server_repo);

        let result = use_case.execute(server.id, member_id, owner_id, crate::domain::entities::BanType::Permanent, None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_ban_member_temporary() {
        let owner_id = Uuid::new_v4();
        let member_id = Uuid::new_v4();
        let server = Server::new("Test".to_string(), owner_id, "CODE".to_string());
        let expires_at = Some(chrono::Utc::now() + chrono::Duration::days(7));

        let mock_server_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, member_id, ServerRole::Member);
        let use_case = BanMemberUseCase::new(mock_server_repo);

        let result = use_case.execute(server.id, member_id, owner_id, crate::domain::entities::BanType::Temporary, expires_at).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_ban_member_unauthorized() {
        let owner_id = Uuid::new_v4();
        let member_id = Uuid::new_v4();
        let requester_id = Uuid::new_v4();
        let server = Server::new("Test".to_string(), owner_id, "CODE".to_string());

        let mock_server_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, member_id, ServerRole::Member)
            .with_member(server.id, requester_id, ServerRole::Member);
        let use_case = BanMemberUseCase::new(mock_server_repo);

        let result = use_case.execute(server.id, member_id, requester_id, crate::domain::entities::BanType::Permanent, None).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_banned_members_success() {
        let owner_id = Uuid::new_v4();
        let banned_id = Uuid::new_v4();
        let server = Server::new("Test".to_string(), owner_id, "CODE".to_string());

        let mock_server_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_ban(server.id, banned_id, "banned_user".to_string(), crate::domain::entities::BanType::Permanent, None);
        let use_case = ListBannedMembersUseCase::new(mock_server_repo);

        let result = use_case.execute(server.id, owner_id).await;
        assert!(result.is_ok());
        let bans = result.unwrap();
        assert_eq!(bans.len(), 1);
        assert_eq!(bans[0].0, banned_id);
    }

    #[tokio::test]
    async fn test_list_banned_members_unauthorized() {
        let owner_id = Uuid::new_v4();
        let requester_id = Uuid::new_v4();
        let server = Server::new("Test".to_string(), owner_id, "CODE".to_string());

        let mock_server_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, requester_id, ServerRole::Member);
        let use_case = ListBannedMembersUseCase::new(mock_server_repo);

        let result = use_case.execute(server.id, requester_id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_unban_member_success() {
        let owner_id = Uuid::new_v4();
        let banned_id = Uuid::new_v4();
        let server = Server::new("Test".to_string(), owner_id, "CODE".to_string());

        let mock_server_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_ban(server.id, banned_id, "banned_user".to_string(), crate::domain::entities::BanType::Permanent, None);
        let use_case = UnbanMemberUseCase::new(mock_server_repo);

        let result = use_case.execute(server.id, banned_id, owner_id).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_unban_member_unauthorized() {
        let owner_id = Uuid::new_v4();
        let banned_id = Uuid::new_v4();
        let requester_id = Uuid::new_v4();
        let server = Server::new("Test".to_string(), owner_id, "CODE".to_string());

        let mock_server_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, requester_id, ServerRole::Member)
            .with_ban(server.id, banned_id, "banned_user".to_string(), crate::domain::entities::BanType::Permanent, None);
        let use_case = UnbanMemberUseCase::new(mock_server_repo);

        let result = use_case.execute(server.id, banned_id, requester_id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_members_success() {
        let password_service = PasswordService::new();
        let owner_id = Uuid::new_v4();
        let server = Server::new("Test".to_string(), owner_id, "CODE".to_string());
        let user = User {
            id: owner_id,
            username: "owner".to_string(),
            email: "owner@test.com".to_string(),
            language: "en".to_string(),
            password_hash: password_service.hash("password").unwrap(),
            status: "ONLINE".to_string(),
            created_at: chrono::Utc::now(),
            avatar_id: None,
        };

        let mock_server_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, owner_id, ServerRole::Owner);
        let mock_user_repo = MockUserRepository::new().with_user(user);

        let use_case = ListMembersUseCase::new(mock_server_repo, mock_user_repo);
        let result = use_case.execute(server.id, owner_id).await;

        assert!(result.is_ok());
        let members = result.unwrap();
        assert_eq!(members.len(), 1);
    }
}
