use crate::application::dto::server::MemberResponse;
use crate::domain::value_objects::ServerRole;
use crate::infrastructure::repositories::server_repository::ServerRepository;
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
            return Err(AppError::Unauthorized(
                "Not a member of this server".to_string(),
            ));
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

    pub async fn execute(
        &self,
        server_id: Uuid,
        target_user_id: Uuid,
        requester_id: Uuid,
        new_role: ServerRole,
    ) -> AppResult<()> {
        let server = self
            .server_repo
            .find_by_id(server_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Server not found".to_string()))?;

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
        }

        if new_role == ServerRole::Owner {
            return Err(AppError::ValidationError(
                "Cannot assign owner role".to_string(),
            ));
        }

        self.server_repo
            .update_member_role(server_id, target_user_id, new_role)
            .await
    }
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::Server;
    use crate::infrastructure::repositories::mocks::mock_server_repository::MockServerRepository;

    #[tokio::test]
    async fn test_list_members_success() {
        let owner_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), owner_id, "CODE123".to_string());

        let mock_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, owner_id, ServerRole::Owner)
            .with_member(server.id, user_id, ServerRole::Member);

        let use_case = ListMembersUseCase::new(mock_repo);
        let result = use_case.execute(server.id, owner_id).await;

        assert!(result.is_ok());
        let members = result.unwrap();
        assert_eq!(members.len(), 2);
    }

    #[tokio::test]
    async fn test_list_members_not_member() {
        let owner_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), owner_id, "CODE123".to_string());

        let mock_repo = MockServerRepository::new().with_server(server.clone());

        let use_case = ListMembersUseCase::new(mock_repo);
        let result = use_case.execute(server.id, user_id).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_members_empty_server() {
        let owner_id = Uuid::new_v4();
        let server = Server::new("Empty Server".to_string(), owner_id, "CODE123".to_string());

        let mock_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, owner_id, ServerRole::Owner);

        let use_case = ListMembersUseCase::new(mock_repo);
        let result = use_case.execute(server.id, owner_id).await;

        assert!(result.is_ok());
        let members = result.unwrap();
        assert_eq!(members.len(), 1);
        assert_eq!(members[0].role, ServerRole::Owner);
    }

    #[tokio::test]
    async fn test_update_member_role_by_owner() {
        let owner_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), owner_id, "CODE123".to_string());

        let mock_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, user_id, ServerRole::Member);

        let use_case = UpdateMemberRoleUseCase::new(mock_repo);
        let result = use_case
            .execute(server.id, user_id, owner_id, ServerRole::Admin)
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_member_role_by_admin() {
        let owner_id = Uuid::new_v4();
        let admin_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), owner_id, "CODE123".to_string());

        let mock_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, admin_id, ServerRole::Admin)
            .with_member(server.id, user_id, ServerRole::Member);

        let use_case = UpdateMemberRoleUseCase::new(mock_repo);
        let result = use_case
            .execute(server.id, user_id, admin_id, ServerRole::Admin)
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_member_role_cannot_assign_owner() {
        let owner_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), owner_id, "CODE123".to_string());

        let mock_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, user_id, ServerRole::Member);

        let use_case = UpdateMemberRoleUseCase::new(mock_repo);
        let result = use_case
            .execute(server.id, user_id, owner_id, ServerRole::Owner)
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_member_role_insufficient_permissions() {
        let owner_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let member_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), owner_id, "CODE123".to_string());

        let mock_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, user_id, ServerRole::Member)
            .with_member(server.id, member_id, ServerRole::Member);

        let use_case = UpdateMemberRoleUseCase::new(mock_repo);
        let result = use_case
            .execute(server.id, member_id, user_id, ServerRole::Admin)
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_member_role_server_not_found() {
        let owner_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();

        let mock_repo = MockServerRepository::new();

        let use_case = UpdateMemberRoleUseCase::new(mock_repo);
        let result = use_case
            .execute(server_id, user_id, owner_id, ServerRole::Admin)
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_member_role_requester_not_member() {
        let owner_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let non_member_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), owner_id, "CODE123".to_string());

        let mock_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, user_id, ServerRole::Member);

        let use_case = UpdateMemberRoleUseCase::new(mock_repo);
        let result = use_case
            .execute(server.id, user_id, non_member_id, ServerRole::Admin)
            .await;

        assert!(result.is_err());
    }
}
