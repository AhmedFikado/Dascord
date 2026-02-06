use crate::application::dto::server::ServerResponse;
use crate::infrastructure::repositories::server_repository::ServerRepository;
use crate::utils::error::{AppError, AppResult};
use uuid::Uuid;

pub struct GetUserServersUseCase<R: ServerRepository> {
    server_repo: R,
}

impl<R: ServerRepository> GetUserServersUseCase<R> {
    pub fn new(server_repo: R) -> Self {
        Self { server_repo }
    }

    pub async fn execute(&self, user_id: Uuid) -> AppResult<Vec<ServerResponse>> {
        let servers = self.server_repo.find_by_user(user_id).await?;
        Ok(servers.into_iter().map(ServerResponse::from).collect())
    }
}

pub struct GetServerInfoUseCase<R: ServerRepository> {
    server_repo: R,
}

impl<R: ServerRepository> GetServerInfoUseCase<R> {
    pub fn new(server_repo: R) -> Self {
        Self { server_repo }
    }

    pub async fn execute(&self, server_id: Uuid, user_id: Uuid) -> AppResult<ServerResponse> {
        let server = self
            .server_repo
            .find_by_id(server_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Server not found".to_string()))?;

        let is_member = self.server_repo.is_member(server_id, user_id).await?;
        if !is_member {
            return Err(AppError::Unauthorized(
                "Not a member of this server".to_string(),
            ));
        }

        Ok(ServerResponse::from(server))
    }
}

pub struct UpdateServerUseCase<R: ServerRepository> {
    server_repo: R,
}

impl<R: ServerRepository> UpdateServerUseCase<R> {
    pub fn new(server_repo: R) -> Self {
        Self { server_repo }
    }

    pub async fn execute(
        &self,
        server_id: Uuid,
        user_id: Uuid,
        name: String,
    ) -> AppResult<ServerResponse> {
        let mut server = self
            .server_repo
            .find_by_id(server_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Server not found".to_string()))?;

        if server.owner_id != user_id {
            return Err(AppError::Unauthorized(
                "Only owner can update server".to_string(),
            ));
        }

        server.name = name;
        let updated = self.server_repo.update(server).await?;
        Ok(ServerResponse::from(updated))
    }
}

pub struct DeleteServerUseCase<R: ServerRepository> {
    server_repo: R,
}

impl<R: ServerRepository> DeleteServerUseCase<R> {
    pub fn new(server_repo: R) -> Self {
        Self { server_repo }
    }

    pub async fn execute(&self, server_id: Uuid, user_id: Uuid) -> AppResult<()> {
        let server = self
            .server_repo
            .find_by_id(server_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Server not found".to_string()))?;

        if server.owner_id != user_id {
            return Err(AppError::Unauthorized(
                "Only owner can delete server".to_string(),
            ));
        }

        self.server_repo.delete(server_id).await
    }
}

pub struct JoinServerUseCase<R: ServerRepository> {
    server_repo: R,
}

impl<R: ServerRepository> JoinServerUseCase<R> {
    pub fn new(server_repo: R) -> Self {
        Self { server_repo }
    }

    pub async fn execute(&self, invitation_code: &str, user_id: Uuid) -> AppResult<()> {
        let server = self
            .server_repo
            .find_by_invitation_code(invitation_code)
            .await?
            .ok_or_else(|| {
                AppError::NotFound("Server not found with this invitation code".to_string())
            })?;

        let is_member = self.server_repo.is_member(server.id, user_id).await?;
        if is_member {
            return Err(AppError::Conflict("Already a member".to_string()));
        }

        self.server_repo.add_member(server.id, user_id).await
    }
}

pub struct LeaveServerUseCase<R: ServerRepository> {
    server_repo: R,
}

impl<R: ServerRepository> LeaveServerUseCase<R> {
    pub fn new(server_repo: R) -> Self {
        Self { server_repo }
    }

    pub async fn execute(&self, server_id: Uuid, user_id: Uuid) -> AppResult<()> {
        let server = self
            .server_repo
            .find_by_id(server_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Server not found".to_string()))?;

        if server.owner_id == user_id {
            return Err(AppError::ValidationError(
                "Owner cannot leave server".to_string(),
            ));
        }

        self.server_repo.remove_member(server_id, user_id).await
    }
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::Server;
    use crate::domain::value_objects::ServerRole;
    use crate::infrastructure::repositories::mocks::mock_server_repository::MockServerRepository;

    #[tokio::test]
    async fn test_get_user_servers_success() {
        let user_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), user_id, "CODE123".to_string());

        let mock_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, user_id, ServerRole::Owner);

        let use_case = GetUserServersUseCase::new(mock_repo);
        let result = use_case.execute(user_id).await;

        assert!(result.is_ok());
        let servers = result.unwrap();
        assert_eq!(servers.len(), 1);
        assert_eq!(servers[0].name, "Test Server");
    }

    #[tokio::test]
    async fn test_get_user_servers_empty() {
        let user_id = Uuid::new_v4();
        let mock_repo = MockServerRepository::new();

        let use_case = GetUserServersUseCase::new(mock_repo);
        let result = use_case.execute(user_id).await;

        assert!(result.is_ok());
        let servers = result.unwrap();
        assert_eq!(servers.len(), 0);
    }

    #[tokio::test]
    async fn test_get_user_servers_multiple() {
        let user_id = Uuid::new_v4();
        let server1 = Server::new("Server 1".to_string(), user_id, "CODE1".to_string());
        let server2 = Server::new("Server 2".to_string(), user_id, "CODE2".to_string());

        let mock_repo = MockServerRepository::new()
            .with_server(server1.clone())
            .with_server(server2.clone())
            .with_member(server1.id, user_id, ServerRole::Owner)
            .with_member(server2.id, user_id, ServerRole::Member);

        let use_case = GetUserServersUseCase::new(mock_repo);
        let result = use_case.execute(user_id).await;

        assert!(result.is_ok());
        let servers = result.unwrap();
        assert_eq!(servers.len(), 2);
    }

    #[tokio::test]
    async fn test_get_server_info_success() {
        let user_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), user_id, "CODE123".to_string());

        let mock_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, user_id, ServerRole::Owner);

        let use_case = GetServerInfoUseCase::new(mock_repo);
        let result = use_case.execute(server.id, user_id).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.name, "Test Server");
    }

    #[tokio::test]
    async fn test_get_server_info_not_member() {
        let owner_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), owner_id, "CODE123".to_string());

        let mock_repo = MockServerRepository::new().with_server(server.clone());

        let use_case = GetServerInfoUseCase::new(mock_repo);
        let result = use_case.execute(server.id, user_id).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_server_info_server_not_found() {
        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let mock_repo = MockServerRepository::new();

        let use_case = GetServerInfoUseCase::new(mock_repo);
        let result = use_case.execute(server_id, user_id).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_server_success() {
        let owner_id = Uuid::new_v4();
        let server = Server::new("Old Name".to_string(), owner_id, "CODE123".to_string());

        let mock_repo = MockServerRepository::new().with_server(server.clone());

        let use_case = UpdateServerUseCase::new(mock_repo);
        let result = use_case
            .execute(server.id, owner_id, "New Name".to_string())
            .await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.name, "New Name");
    }

    #[tokio::test]
    async fn test_update_server_not_owner() {
        let owner_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), owner_id, "CODE123".to_string());

        let mock_repo = MockServerRepository::new().with_server(server.clone());

        let use_case = UpdateServerUseCase::new(mock_repo);
        let result = use_case
            .execute(server.id, user_id, "New Name".to_string())
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_server_server_not_found() {
        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let mock_repo = MockServerRepository::new();

        let use_case = UpdateServerUseCase::new(mock_repo);
        let result = use_case
            .execute(server_id, user_id, "New Name".to_string())
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_server_success() {
        let owner_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), owner_id, "CODE123".to_string());

        let mock_repo = MockServerRepository::new().with_server(server.clone());

        let use_case = DeleteServerUseCase::new(mock_repo);
        let result = use_case.execute(server.id, owner_id).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_server_not_owner() {
        let owner_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), owner_id, "CODE123".to_string());

        let mock_repo = MockServerRepository::new().with_server(server.clone());

        let use_case = DeleteServerUseCase::new(mock_repo);
        let result = use_case.execute(server.id, user_id).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_server_server_not_found() {
        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let mock_repo = MockServerRepository::new();

        let use_case = DeleteServerUseCase::new(mock_repo);
        let result = use_case.execute(server_id, user_id).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_join_server_success() {
        let owner_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), owner_id, "CODE123".to_string());

        let mock_repo = MockServerRepository::new().with_server(server.clone());

        let use_case = JoinServerUseCase::new(mock_repo);
        let result = use_case.execute("CODE123", user_id).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_join_server_already_member() {
        let owner_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), owner_id, "CODE123".to_string());

        let mock_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, user_id, ServerRole::Member);

        let use_case = JoinServerUseCase::new(mock_repo);
        let result = use_case.execute("CODE123", user_id).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_join_server_invalid_code() {
        let user_id = Uuid::new_v4();
        let mock_repo = MockServerRepository::new();

        let use_case = JoinServerUseCase::new(mock_repo);
        let result = use_case.execute("INVALID", user_id).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_leave_server_success() {
        let owner_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), owner_id, "CODE123".to_string());

        let mock_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, user_id, ServerRole::Member);

        let use_case = LeaveServerUseCase::new(mock_repo);
        let result = use_case.execute(server.id, user_id).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_leave_server_owner_cannot_leave() {
        let owner_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), owner_id, "CODE123".to_string());

        let mock_repo = MockServerRepository::new().with_server(server.clone());

        let use_case = LeaveServerUseCase::new(mock_repo);
        let result = use_case.execute(server.id, owner_id).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_leave_server_server_not_found() {
        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let mock_repo = MockServerRepository::new();

        let use_case = LeaveServerUseCase::new(mock_repo);
        let result = use_case.execute(server_id, user_id).await;

        assert!(result.is_err());
    }
}
