use crate::application::dto::auth::LogoutResponse;
use crate::infrastructure::repositories::UserRepository;
use crate::infrastructure::security::jwt::JWTService;
use crate::infrastructure::services::UserService;
use crate::utils::error::{AppError, AppResult};
use uuid::Uuid;

pub struct LogoutUseCase<R: UserRepository> {
    user_service: UserService<R>,
    jwt_service: JWTService,
}

impl<R: UserRepository> LogoutUseCase<R> {
    pub fn new(user_service: UserService<R>, jwt_service: JWTService) -> Self {
        Self {
            user_service,
            jwt_service,
        }
    }

    pub async fn execute(&self, token: String) -> AppResult<LogoutResponse> {
        let claims = self.jwt_service.verify_token(&token)?;
        let user_id = Uuid::parse_str(&claims.sub_id)
            .map_err(|_| AppError::ValidationError("Invalid user ID in token".to_string()))?;

        let user = self
            .user_service
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        if user.status == "OFFLINE" {
            return Err(AppError::Unauthorized("User is not connected".to_string()));
        }

        self.user_service.update_status(user_id, "OFFLINE").await?;

        Ok(LogoutResponse {
            message: "Vous êtes bien déconnecté".to_string(),
        })
    }
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::User;
    use crate::infrastructure::repositories::mocks::mock_user_repository::MockUserRepository;
    use crate::infrastructure::security::PasswordService;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_logout_invalid_token() {
        let mock_repo = MockUserRepository::new();
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let logout_uc = LogoutUseCase::new(user_service, jwt_service);

        let result = logout_uc.execute("invalid_token".to_string()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_logout_user_not_found() {
        let mock_repo = MockUserRepository::new();
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let logout_uc = LogoutUseCase::new(user_service, jwt_service.clone());

        let user_id = Uuid::new_v4();
        let token = jwt_service.create_token(user_id).unwrap();
        let result = logout_uc.execute(token).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_logout_success() {
        let password_service = PasswordService::new();
        let user_id = Uuid::new_v4();
        let user = User {
            id: user_id,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: password_service.hash("password").unwrap(),
            status: "ONLINE".to_string(),
            created_at: chrono::Utc::now(),
        };

        let mock_repo = MockUserRepository::new().with_user(user);
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let logout_uc = LogoutUseCase::new(user_service, jwt_service.clone());

        let token = jwt_service.create_token(user_id).unwrap();
        let result = logout_uc.execute(token).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.message, "Vous êtes bien déconnecté");
    }

    #[tokio::test]
    async fn test_logout_user_already_offline() {
        let password_service = PasswordService::new();
        let user_id = Uuid::new_v4();
        let user = User {
            id: user_id,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: password_service.hash("password").unwrap(),
            status: "OFFLINE".to_string(),
            created_at: chrono::Utc::now(),
        };

        let mock_repo = MockUserRepository::new().with_user(user);
        let user_service = UserService::new(mock_repo);
        let jwt_service = JWTService::new("test_secret".to_string());
        let logout_uc = LogoutUseCase::new(user_service, jwt_service.clone());

        let token = jwt_service.create_token(user_id).unwrap();
        let result = logout_uc.execute(token).await;

        assert!(result.is_err());
    }
}
