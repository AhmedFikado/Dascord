use crate::application::dto::auth::LogoutResponse;
use crate::mocks::MockUserService;
use crate::mocks::mock_user_repo::UserRepository;
use crate::infrastructure::security::jwt::JWTService;
use crate::domain::value_objects::user_status::UserStatus;
use crate::utils::error::{AppError, AppResult};
use uuid::Uuid;

pub struct LogoutUseCase {
    user_service: MockUserService,
    jwt_service: JWTService,
}

impl LogoutUseCase {
    pub fn new(user_service: MockUserService, jwt_service: JWTService) -> Self {
        Self { user_service, jwt_service }
    }

    pub async fn execute(&self, token: String) -> AppResult<LogoutResponse> {
        let claims = self.jwt_service.verify_token(&token)?;
        let user_id = Uuid::parse_str(&claims.sub_id)
            .map_err(|_| AppError::ValidationError("Invalid user ID in token".to_string()))?;

        let user = self.user_service.repo()
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        if user.status == UserStatus::Offline {
            return Err(AppError::Unauthorized("User is not connected".to_string()));
        }

        self.user_service.update_status(user_id, UserStatus::Offline).await?;

        Ok(LogoutResponse {
            message: "Vous êtes bien déconnecté".to_string(),
        })
    }
}
