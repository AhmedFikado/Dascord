use crate::mocks::mock_user_entitie::User;
use crate::mocks::mock_user_repo::UserRepository;
use crate::utils::error::{AppError, AppResult};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// Repository en mémoire pour simuler la base de données
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

    /// Ajouter un user de test (utile pour les tests)
    pub fn add_test_user(&self, user: User) {
        let mut users = self.users.lock().unwrap();
        users.insert(user.id, user);
    }

    /// Vider la base de données (reset entre les tests)
    pub fn clear(&self) {
        let mut users = self.users.lock().unwrap();
        users.clear();
    }
}

#[async_trait]
impl UserRepository for MockUserRepository {
    async fn create(&self, user: User) -> AppResult<User> {
        let mut users = self.users.lock().unwrap();
        
        if users.contains_key(&user.id) {
            return Err(AppError::Conflict("User ID already exists".to_string()));
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

    async fn update(&self, user: User) -> AppResult<User> {
        let mut users = self.users.lock().unwrap();
        
        if !users.contains_key(&user.id) {
            return Err(AppError::NotFound("User not found".to_string()));
        }

        users.insert(user.id, user.clone());
        Ok(user)
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        let mut users = self.users.lock().unwrap();
        
        if users.remove(&id).is_none() {
            return Err(AppError::NotFound("User not found".to_string()));
        }

        Ok(())
    }

    async fn list_all(&self) -> AppResult<Vec<User>> {
        let users = self.users.lock().unwrap();
        Ok(users.values().cloned().collect())
    }
}