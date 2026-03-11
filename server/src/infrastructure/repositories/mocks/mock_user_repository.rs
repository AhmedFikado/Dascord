use crate::domain::entities::User;
use crate::infrastructure::repositories::UserRepository;
use crate::utils::error::{AppError, AppResult};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Clone)]
pub struct MockUserRepository {
    users: Arc<Mutex<HashMap<Uuid, User>>>,
}

impl MockUserRepository {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn with_user(self, user: User) -> Self {
        {
            let mut users = self.users.lock().unwrap();
            users.insert(user.id, user);
        }
        self
    }
}

#[async_trait]
impl UserRepository for MockUserRepository {
    async fn create(&self, user: User) -> AppResult<User> {
        let mut users = self.users.lock().unwrap();

        if users.values().any(|u| u.email == user.email) {
            return Err(AppError::Conflict("Email already exists".to_string()));
        }

        if users.values().any(|u| u.username == user.username) {
            return Err(AppError::Conflict("Username already taken".to_string()));
        }

        users.insert(user.id, user.clone());
        Ok(user)
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<User>> {
        let users = self.users.lock().unwrap();
        Ok(users.get(&id).cloned())
    }

    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>> {
        let users = self.users.lock().unwrap();
        Ok(users.values().find(|u| u.email == email).cloned())
    }

    async fn find_by_username(&self, username: &str) -> AppResult<Option<User>> {
        let users = self.users.lock().unwrap();
        Ok(users.values().find(|u| u.username == username).cloned())
    }

    async fn update_status(&self, id: Uuid, status: &str) -> AppResult<()> {
        let mut users = self.users.lock().unwrap();

        if let Some(user) = users.get_mut(&id) {
            user.status = status.to_string();
            Ok(())
        } else {
            Err(AppError::NotFound("User not found".to_string()))
        }
    }

    async fn update_user(&self, user: User) -> AppResult<Option<User>> {
        let mut users = self.users.lock().unwrap();

        if let Some(existing_user) = users.get_mut(&user.id) {
            existing_user.username = user.username.clone();
            existing_user.email = user.email.clone();
            Ok(Some(existing_user.clone()))
        } else {
            Ok(None)
        }
}
}
