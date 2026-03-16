use crate::application::dto::auth::{LoginRequest, LoginResponse};
use crate::infrastructure::repositories::UserRepository;
use crate::infrastructure::security::jwt::JWTService;
use crate::domain::services::user::UserService;
use crate::infrastructure::websocket::ConnectionManager;
use crate::utils::error::{AppError, AppResult};
use std::sync::Arc;
use validator::Validate;

pub struct LoginUseCase<R: UserRepository> {
    user_service: UserService<R>,
    jwt_service: JWTService,
    ws_manager: Option<Arc<ConnectionManager>>,
}

impl<R: UserRepository> LoginUseCase<R> {
    pub fn new(user_service: UserService<R>, jwt_service: JWTService) -> Self {
        Self {
            user_service,
            jwt_service,
            ws_manager: None,
        }
    }

    pub fn with_ws_manager(mut self, ws_manager: Arc<ConnectionManager>) -> Self {
        self.ws_manager = Some(ws_manager);
        self
    }

    pub async fn execute(&self, request: LoginRequest) -> AppResult<LoginResponse> {
        request
            .validate()
            .map_err(|e| AppError::ValidationError(e.to_string()))?;

        let user = self
            .user_service
            .authenticate(&request.email, &request.password)
            .await?;
        let user = self.user_service.update_status(user.id, "ONLINE").await?;

        if let Some(ws_manager) = &self.ws_manager {
            ws_manager
                .broadcast_status_change(user.id, "ONLINE".to_string())
                .await;
        }

        let token = self.jwt_service.create_token(user.id)?;

        let message = format!("Vous êtes bien connecté avec {}", user.username);

        Ok(LoginResponse {
            message,
            user: crate::application::dto::auth::UserResponse {
                id: user.id.to_string(),
                username: user.username,
                email: user.email,
                status: user.status,
                language: user.language,
            },
            token,
        })
    }
}

// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::repositories::mocks::mock_user_repository::MockUserRepository;

    #[tokio::test]
    async fn test_login_validation_error() {
        let mock_repo = MockUserRepository::new();
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let login_uc = LoginUseCase::new(user_service, jwt_service);

        let request = LoginRequest {
            email: "invalid-email".to_string(),
            password: "password123".to_string(),
        };

        let result = login_uc.execute(request).await;
        assert!(result.is_err());
    }
}
