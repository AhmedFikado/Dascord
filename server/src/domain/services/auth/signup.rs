use crate::application::dto::auth::{SignupRequest, SignupResponse};
use crate::infrastructure::repositories::UserRepository;
use crate::infrastructure::security::jwt::JWTService;
use crate::domain::services::user::UserService;
use crate::utils::error::{AppError, AppResult};
use validator::Validate;

pub struct SignupUseCase<R: UserRepository> {
    user_service: UserService<R>,
    jwt_service: JWTService,
}

impl<R: UserRepository> SignupUseCase<R> {
    pub fn new(user_service: UserService<R>, jwt_service: JWTService) -> Self {
        Self {
            user_service,
            jwt_service,
        }
    }

    pub async fn execute(&self, request: SignupRequest) -> AppResult<SignupResponse> {
        request
            .validate()
            .map_err(|e| AppError::ValidationError(e.to_string()))?;

        if self.user_service.email_exists(&request.email).await? {
            return Err(AppError::Conflict("Email already exists".to_string()));
        }

        if self.user_service.username_exists(&request.username).await? {
            return Err(AppError::Conflict("Username already taken".to_string()));
        }

        let user = self
            .user_service
            .create_user(request.username, request.email, request.password, request.language)
            .await?;

        let user = self.user_service.update_status(user.id, "ONLINE").await?;
        let token = self.jwt_service.create_token(user.id)?;

        Ok(SignupResponse {
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
    async fn test_signup_validation_error() {
        let mock_repo = MockUserRepository::new();
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let signup_uc = SignupUseCase::new(user_service, jwt_service);

        let request = SignupRequest {
            username: "ab".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            language: "en".to_string(),
        };

        let result = signup_uc.execute(request).await;
        assert!(result.is_err());
    }
}
