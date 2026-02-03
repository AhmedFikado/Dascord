use crate::domain::entities::User;
use crate::infrastructure::repositories::user_repository::{UserRepository, PostgresUserRepository};
use crate::infrastructure::security::password::PasswordService;
use crate::utils::error::{AppError, AppResult};
use uuid::Uuid;

#[derive(Clone)]
pub struct UserService {
    user_repo: PostgresUserRepository,
    password_service: PasswordService,
}

impl UserService {
    pub fn new(user_repo: PostgresUserRepository) -> Self {
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
    ) -> AppResult<User> {
        let password_hash = self.password_service.hash(&password)?;

        let user = User {
            id: Uuid::new_v4(),
            username,
            email,
            password_hash,
            status: "OFFLINE".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
        };

        self.user_repo.create(user).await
    }

    pub async fn authenticate(&self, email: &str, password: &str) -> AppResult<User> {
        let user = self.user_repo
            .find_by_email(email)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;

        let is_valid = self.password_service.verify(password, &user.password_hash)?;

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
}
