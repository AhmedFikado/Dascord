use crate::application::dto::server::{CreateServerRequest, ServerResponse};
use crate::infrastructure::repositories::server_repository::ServerRepository;
use crate::domain::entities::Server;
use crate::utils::error::{AppError, AppResult};
use uuid::Uuid;
use validator::Validate;

pub struct CreateServerUseCase<R: ServerRepository> {
    server_repo: R,
}

impl<R: ServerRepository> CreateServerUseCase<R> {
    pub fn new(server_repo: R) -> Self {
        Self { server_repo }
    }

    pub async fn execute(&self, request: CreateServerRequest, owner_id: Uuid) -> AppResult<ServerResponse> {
        request.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

        let invitation_code = Self::generate_invitation_code();
        let server = Server::new(request.name, owner_id, invitation_code);

        let created_server = self.server_repo.create(server).await?;
        
        Ok(ServerResponse::from(created_server))
    }

    fn generate_invitation_code() -> String {
        use rand::Rng;
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let mut rng = rand::thread_rng();
        (0..8)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }
}
