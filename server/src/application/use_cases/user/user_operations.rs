use crate::application::dto::user_dto::UserDto;
use crate::infrastructure::repositories::UserRepository;
use crate::infrastructure::services::UserService;
use crate::utils::error::{AppError, AppResult};
use uuid::Uuid;

pub struct GetUserInfoUseCase<R: UserRepository> {
    user_service: UserService<R>,
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
    }

    #[tokio::test]
    async fn test_get_user_info_not_found() {
        let mock_repo = MockUserRepository::new();
        let user_service = UserService::new(mock_repo);
        let use_case = GetUserInfoUseCase::new(user_service);

        let result = use_case.execute(Uuid::new_v4()).await;
        assert!(result.is_err());
    }
}
