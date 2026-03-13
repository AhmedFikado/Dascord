use crate::domain::entities::User;
use crate::infrastructure::repositories::user_repository::UserRepository;
use crate::infrastructure::security::password::PasswordService;
use crate::utils::error::{AppError, AppResult};
use uuid::Uuid;

#[derive(Clone)]
pub struct UserService<R: UserRepository> {
    user_repo: R,
    password_service: PasswordService,
}

impl<R: UserRepository> UserService<R> {
    pub fn new(user_repo: R) -> Self {
        Self {
            user_repo,
            password_service: PasswordService::new(),
        }
    }

    pub async fn create_user(
        &self,
        username: String,
        email: String,
        password: String,
        language: String,
    ) -> AppResult<User> {
        let password_hash = self.password_service.hash(&password)?;

        let user = User {
            id: Uuid::new_v4(),
            username,
            email,
            password_hash,
            status: "OFFLINE".to_string(),
            language,
            created_at: chrono::Utc::now(),
        };

        self.user_repo.create(user).await
    }

    pub async fn authenticate(&self, email: &str, password: &str) -> AppResult<User> {
        let user = self
            .user_repo
            .find_by_email(email)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;

        let is_valid = self
            .password_service
            .verify(password, &user.password_hash)?;

        if !is_valid {
            return Err(AppError::Unauthorized("Invalid credentials".to_string()));
        }

        Ok(user)
    }

    pub async fn email_exists(&self, email: &str) -> AppResult<bool> {
        Ok(self.user_repo.find_by_email(email).await?.is_some())
    }

    pub async fn username_exists(&self, username: &str) -> AppResult<bool> {
        Ok(self.user_repo.find_by_username(username).await?.is_some())
    }

    pub async fn update_status(&self, user_id: Uuid, status: &str) -> AppResult<User> {
        self.user_repo.update_status(user_id, status).await?;

        self.user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))
    }

    pub async fn find_by_id(&self, user_id: Uuid) -> AppResult<Option<User>> {
        self.user_repo.find_by_id(user_id).await
    }

    pub async fn update_user(&self, user: User) -> AppResult<Option<User>> {
        self.user_repo.update_user(user).await
    }
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::repositories::mocks::mock_user_repository::MockUserRepository;

    #[tokio::test]
    async fn test_create_user() {
        let repo = MockUserRepository::new();
        let service = UserService::new(repo);

        let result = service
            .create_user(
                "testuser".to_string(),
                "test@example.com".to_string(),
                "password123".to_string(),
                "en".to_string(),
            )
            .await;

        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.username, "testuser");
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.status, "OFFLINE");
    }

    #[tokio::test]
    async fn test_authenticate_success() {
        let password_service = PasswordService::new();
        let user = User {
            id: Uuid::new_v4(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: password_service.hash("password123").unwrap(),
            language: "en".to_string(),
            status: "OFFLINE".to_string(),
            created_at: chrono::Utc::now(),
        };

        let repo = MockUserRepository::new().with_user(user);
        let service = UserService::new(repo);

        let result = service
            .authenticate("test@example.com", "password123")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_authenticate_wrong_password() {
        let password_service = PasswordService::new();
        let user = User {
            id: Uuid::new_v4(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: password_service.hash("password123").unwrap(),
            language: "en".to_string(),
            status: "OFFLINE".to_string(),
            created_at: chrono::Utc::now(),
        };

        let repo = MockUserRepository::new().with_user(user);
        let service = UserService::new(repo);

        let result = service
            .authenticate("test@example.com", "wrongpassword")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_email_exists() {
        let password_service = PasswordService::new();
        let user = User {
            id: Uuid::new_v4(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: password_service.hash("password").unwrap(),
            language: "en".to_string(),
            status: "OFFLINE".to_string(),
            created_at: chrono::Utc::now(),
        };

        let repo = MockUserRepository::new().with_user(user);
        let service = UserService::new(repo);

        let exists = service.email_exists("test@example.com").await.unwrap();
        assert!(exists);

        let not_exists = service.email_exists("notfound@example.com").await.unwrap();
        assert!(!not_exists);
    }

    #[tokio::test]
    async fn test_username_exists() {
        let password_service = PasswordService::new();
        let user = User {
            id: Uuid::new_v4(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: password_service.hash("password").unwrap(),
            language: "en".to_string(),
            status: "OFFLINE".to_string(),
            created_at: chrono::Utc::now(),
        };

        let repo = MockUserRepository::new().with_user(user);
        let service = UserService::new(repo);

        let exists = service.username_exists("testuser").await.unwrap();
        assert!(exists);

        let not_exists = service.username_exists("notfound").await.unwrap();
        assert!(!not_exists);
    }

    #[tokio::test]
    async fn test_create_user_duplicate_email() {
        let password_service = PasswordService::new();
        let existing_user = User {
            id: Uuid::new_v4(),
            username: "existing".to_string(),
            email: "test@example.com".to_string(),
            password_hash: password_service.hash("password").unwrap(),
            language: "en".to_string(),
            status: "OFFLINE".to_string(),
            created_at: chrono::Utc::now(),
        };

        let repo = MockUserRepository::new().with_user(existing_user);
        let service = UserService::new(repo);

        let result = service
            .create_user(
                "newuser".to_string(),
                "test@example.com".to_string(),
                "password123".to_string(),
                "en".to_string(),
            )
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_user_duplicate_username() {
        let password_service = PasswordService::new();
        let existing_user = User {
            id: Uuid::new_v4(),
            username: "testuser".to_string(),
            email: "existing@example.com".to_string(),
            password_hash: password_service.hash("password").unwrap(),
            language: "en".to_string(),
            status: "OFFLINE".to_string(),
            created_at: chrono::Utc::now(),
        };

        let repo = MockUserRepository::new().with_user(existing_user);
        let service = UserService::new(repo);

        let result = service
            .create_user(
                "testuser".to_string(),
                "new@example.com".to_string(),
                "password123".to_string(),
                "en".to_string(),
            )
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_status_user_not_found() {
        let repo = MockUserRepository::new();
        let service = UserService::new(repo);

        let result = service.update_status(Uuid::new_v4(), "ONLINE").await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_find_by_id_not_found() {
        let repo = MockUserRepository::new();
        let service = UserService::new(repo);

        let result = service.find_by_id(Uuid::new_v4()).await.unwrap();

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_update_user_not_found() {
        let repo = MockUserRepository::new();
        let service = UserService::new(repo);

        let user = User {
            id: Uuid::new_v4(),
            username: "nonexistent".to_string(),
            email: "nonexistent@example.com".to_string(),
            password_hash: String::new(),
            language: String::new(),
            status: String::new(),
            created_at: chrono::Utc::now(),
        };

        let result = service.update_user(user).await.unwrap();

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_update_status_success() {
        let password_service = PasswordService::new();
        let existing_user = User {
            id: Uuid::new_v4(),
            username: "testuser".to_string(),
            email: "existing@example.com".to_string(),
            password_hash: password_service.hash("password").unwrap(),
            language: "en".to_string(),
            status: "OFFLINE".to_string(),
            created_at: chrono::Utc::now(),
        };

        let user_id = existing_user.id;
        let repo = MockUserRepository::new().with_user(existing_user);
        let service = UserService::new(repo);

        let result = service.update_status(user_id, "ONLINE").await;

        assert!(result.is_ok());
    }
}