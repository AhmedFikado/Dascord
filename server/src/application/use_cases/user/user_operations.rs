use crate::application::dto::user_dto::UserDto;
use crate::domain::value_objects::user_status::UserStatus;
use crate::infrastructure::repositories::UserRepository;
use crate::infrastructure::services::UserService;
use crate::infrastructure::websocket::ConnectionManager;
use crate::utils::error::{AppError, AppResult};
use std::sync::Arc;
use uuid::Uuid;

pub struct GetUserInfoUseCase<R: UserRepository> {
    pub user_service: UserService<R>,
}

impl<R: UserRepository> GetUserInfoUseCase<R> {
    pub fn new(user_service: UserService<R>) -> Self {
        Self { user_service }
    }

    pub async fn execute(&self, user_id: Uuid) -> AppResult<UserDto> {
        let user = self
            .user_service
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        Ok(UserDto {
            id: user.id.to_string(),
            username: user.username,
            email: user.email,
            status: user.status,
        })
    }
}

#[derive(Clone)]
pub struct UpdateUserStatusUseCase<R: UserRepository> {
    pub user_service: UserService<R>,
    ws_manager: Option<Arc<ConnectionManager>>,
}

impl<R: UserRepository> UpdateUserStatusUseCase<R> {
    pub fn new(user_service: UserService<R>) -> Self {
        Self {
            user_service,
            ws_manager: None,
        }
    }

    pub fn with_ws_manager(mut self, ws_manager: Arc<ConnectionManager>) -> Self {
        self.ws_manager = Some(ws_manager);
        self
    }

    pub async fn execute(&self, user_id: Uuid, status: String) -> AppResult<UserDto> {
        let _: UserStatus = serde_json::from_str(&format!(r#""{}""#, status))
            .map_err(|_| AppError::ValidationError("Invalid status".to_string()))?;

        let user = self.user_service.update_status(user_id, &status).await?;
        
        if let Some(ws_manager) = &self.ws_manager {
            ws_manager.broadcast_status_change(user_id, status.clone()).await;
        }

        Ok(UserDto {
            id: user.id.to_string(),
            username: user.username,
            email: user.email,
            status: user.status,
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
    async fn test_get_user_info_success() {
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
        let use_case = GetUserInfoUseCase::new(user_service);

        let result = use_case.execute(user_id).await;
        assert!(result.is_ok());
        let user_dto = result.unwrap();
        assert_eq!(user_dto.username, "testuser");
        assert_eq!(user_dto.email, "test@example.com");
        assert_eq!(user_dto.status, "ONLINE");
    }

    #[tokio::test]
    async fn test_get_user_info_not_found() {
        let mock_repo = MockUserRepository::new();
        let user_service = UserService::new(mock_repo);
        let use_case = GetUserInfoUseCase::new(user_service);

        let result = use_case.execute(Uuid::new_v4()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_user_status_success() {
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
        let use_case = UpdateUserStatusUseCase::new(user_service);

        let result = use_case.execute(user_id, "ONLINE".to_string()).await;
        assert!(result.is_ok());
        let user_dto = result.unwrap();
        assert_eq!(user_dto.status, "ONLINE");
    }

    #[tokio::test]
    async fn test_update_user_status_invalid_status() {
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
        let use_case = UpdateUserStatusUseCase::new(user_service);

        let result = use_case.execute(user_id, "INVALID".to_string()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_user_status_user_not_found() {
        let mock_repo = MockUserRepository::new();
        let user_service = UserService::new(mock_repo);
        let use_case = UpdateUserStatusUseCase::new(user_service);

        let result = use_case.execute(Uuid::new_v4(), "ONLINE".to_string()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_user_status_offline() {
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
        let use_case = UpdateUserStatusUseCase::new(user_service);

        let result = use_case.execute(user_id, "OFFLINE".to_string()).await;
        assert!(result.is_ok());
        let user_dto = result.unwrap();
        assert_eq!(user_dto.status, "OFFLINE");
    }

    #[tokio::test]
    async fn test_update_user_status_with_ws_manager() {
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
        let ws_manager = Arc::new(ConnectionManager::new());
        let use_case = UpdateUserStatusUseCase::new(user_service).with_ws_manager(ws_manager);

        let result = use_case.execute(user_id, "ONLINE".to_string()).await;
        assert!(result.is_ok());
    }
}
