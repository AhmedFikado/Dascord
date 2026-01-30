use crate::mocks::mock_user_entitie::User;
use crate::utils::error::AppResult;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: User) -> AppResult<User>;
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<User>>;
    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>>;
    async fn find_by_username(&self, username: &str) -> AppResult<Option<User>>;
    async fn update(&self, user: User) -> AppResult<User>;
    async fn delete(&self, id: Uuid) -> AppResult<()>;
    async fn list_all(&self) -> AppResult<Vec<User>>;
}