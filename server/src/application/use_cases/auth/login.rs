use crate::application::dto::auth::{LoginRequest, LoginResponse};
use crate::infrastructure::services::UserService;
use crate::infrastructure::security::jwt::JWTService;
use crate::utils::error::{AppError, AppResult};
use validator::Validate;

pub struct LoginUseCase {
    user_service: UserService,
    jwt_service: JWTService,
}

impl LoginUseCase {
    pub fn new(user_service: UserService, jwt_service: JWTService) -> Self {
        Self { user_service, jwt_service }
    }

    pub async fn execute(&self, request: LoginRequest) -> AppResult<LoginResponse> {
        request.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
        
        let user = self.user_service.authenticate(&request.email, &request.password).await?;
        let user = self.user_service.update_status(user.id, "ONLINE").await?;
        let token = self.jwt_service.create_token(user.id)?;

        let message = format!("Vous êtes bien connecté avec {}", user.username);

        Ok(LoginResponse {
            message,
            user: crate::application::dto::auth::UserResponse {
                id: user.id.to_string(),
                username: user.username,
                email: user.email,
                status: user.status,
            },
            token,
        })
    }
}
