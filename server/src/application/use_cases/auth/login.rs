use crate::application::dto::auth::{LoginRequest, LoginResponse}; // import DTOs nécessaires pour le login
use crate::mocks::MockUserService;
use crate::infrastructure::security::jwt::JWTService;
use crate::utils::error::{AppError, AppResult};
use validator::Validate;

pub struct LoginUseCase {
    user_service: MockUserService,
    jwt_service: JWTService,
}

impl LoginUseCase {
    pub fn new(user_service: MockUserService, jwt_service: JWTService) -> Self {
        Self { user_service, jwt_service }
    }

    pub async fn execute(&self, request: LoginRequest) -> AppResult<LoginResponse> {
        request.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
        
        let user = self.user_service.authenticate(&request.email, &request.password).await?;
        let user = self.user_service.update_status(user.id, crate::domain::value_objects::user_status::UserStatus::Online).await?;
        let token = self.jwt_service.create_token(user.id)?;

        let message = format!("Vous êtes bien connecté avec {}", user.username);

        Ok(LoginResponse {
            message,
            user: user.into(),
            token,
        })
    }
}
