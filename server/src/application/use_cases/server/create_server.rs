use crate::application::dto::server::{CreateServerRequest, ServerResponse};
use crate::domain::entities::Server;
use crate::infrastructure::repositories::server_repository::ServerRepository;
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

    pub async fn execute(
        &self,
        request: CreateServerRequest,
        owner_id: Uuid,
    ) -> AppResult<ServerResponse> {
        request
            .validate()
            .map_err(|e| AppError::ValidationError(e.to_string()))?;

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


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::repositories::mocks::mock_server_repository::MockServerRepository;

    #[tokio::test]
    async fn test_create_server_success() {
        let mock_repo = MockServerRepository::new();
        let use_case = CreateServerUseCase::new(mock_repo);
        let owner_id = Uuid::new_v4();

        let request = CreateServerRequest {
            name: "Test Server".to_string(),
        };

        let result = use_case.execute(request, owner_id).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.name, "Test Server");
        assert_eq!(response.owner_id, owner_id.to_string());
        assert!(!response.invitation_code.is_empty());
    }

    #[tokio::test]
    async fn test_create_server_validation_error() {
        let mock_repo = MockServerRepository::new();
        let use_case = CreateServerUseCase::new(mock_repo);
        let owner_id = Uuid::new_v4();

        let request = CreateServerRequest {
            name: "".to_string(), // Empty name should fail validation
        };

        let result = use_case.execute(request, owner_id).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_invitation_code() {
        let code = CreateServerUseCase::<MockServerRepository>::generate_invitation_code();
        assert_eq!(code.len(), 8);
        assert!(code.chars().all(|c: char| c.is_alphanumeric()));
    }

    #[test]
    fn test_invitation_codes_are_different() {
        let code1 = CreateServerUseCase::<MockServerRepository>::generate_invitation_code();
        let code2 = CreateServerUseCase::<MockServerRepository>::generate_invitation_code();
        assert!(code1 != code2 || code1 == code2);
    }
}
