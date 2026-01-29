// src/application/use_cases/auth/signup.rs
use crate::application::dto::auth::{SignupRequest, SignupResponse};
use crate::mocks::MockUserService;
use crate::infrastructure::security::jwt::JWTService;
use crate::utils::error::{AppError, AppResult};
use validator::Validate;


pub struct SignupUseCase {
    user_service: MockUserService,
    jwt_service: JWTService,
}

impl SignupUseCase {
    pub fn new(user_service: MockUserService, jwt_service: JWTService) -> Self {
        Self { user_service, jwt_service }
    }

    pub async fn execute(&self, request: SignupRequest) -> AppResult<SignupResponse> {
        request.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

        if self.user_service.email_exists(&request.email).await? {
            return Err(AppError::Conflict("Email already exists".to_string()));
        }

        if self.user_service.username_exists(&request.username).await? {
            return Err(AppError::Conflict("Username already taken".to_string()));
        }

        let user = self.user_service.create_user(request.username, request.email, request.password).await?;
        let token = self.jwt_service.create_token(user.id)?;

        Ok(SignupResponse {
            user: user.into(),
            token,
        })
    }
}
