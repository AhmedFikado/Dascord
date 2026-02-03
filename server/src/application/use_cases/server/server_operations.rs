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
        let server = self.server_repo.find_by_id(server_id).await?
            .ok_or_else(|| AppError::NotFound("Server not found".to_string()))?;
        
        let is_member = self.server_repo.is_member(server_id, user_id).await?;
        if !is_member {
            return Err(AppError::Unauthorized("Not a member of this server".to_string()));
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

    pub async fn execute(&self, server_id: Uuid, user_id: Uuid, name: String) -> AppResult<ServerResponse> {
        let mut server = self.server_repo.find_by_id(server_id).await?
            .ok_or_else(|| AppError::NotFound("Server not found".to_string()))?;

        if server.owner_id != user_id {
            return Err(AppError::Unauthorized("Only owner can update server".to_string()));
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
        let server = self.server_repo.find_by_id(server_id).await?
            .ok_or_else(|| AppError::NotFound("Server not found".to_string()))?;

        if server.owner_id != user_id {
            return Err(AppError::Unauthorized("Only owner can delete server".to_string()));
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

    pub async fn execute(&self, server_id: Uuid, user_id: Uuid) -> AppResult<()> {
        let _server = self.server_repo.find_by_id(server_id).await?
            .ok_or_else(|| AppError::NotFound("Server not found".to_string()))?;

        let is_member = self.server_repo.is_member(server_id, user_id).await?;
        if is_member {
            return Err(AppError::Conflict("Already a member".to_string()));
        }

        self.server_repo.add_member(server_id, user_id).await
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
        let server = self.server_repo.find_by_id(server_id).await?
            .ok_or_else(|| AppError::NotFound("Server not found".to_string()))?;

        if server.owner_id == user_id {
            return Err(AppError::ValidationError("Owner cannot leave server".to_string()));
        }

        self.server_repo.remove_member(server_id, user_id).await
    }
}
