use crate::mocks::mock_user_entitie::User;
use crate::domain::value_objects::user_status::UserStatus;
use crate::mocks::mock_user_repo::TestUserRepository;
use crate::infrastructure::security::password::PasswordService;
use crate::mocks::mock_user_repository::MockUserRepository;
use crate::utils::error::{AppError, AppResult};
use uuid::Uuid;

/// UserService utilisant le MockUserRepository
#[derive(Clone)]
pub struct MockUserService {
    user_repo: MockUserRepository,
    password_service: PasswordService,
}

impl MockUserService {
    pub fn new() -> Self {
        Self {
            user_repo: MockUserRepository::new(),
            password_service: PasswordService::new(),
        }
    }

    /// Créer un nouvel utilisateur
    pub async fn create_user(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> AppResult<User> {
        // Hash du mot de passe
        let password_hash = self.password_service.hash(&password)?;

        // Créer l'entité User
        let user = User {
            id: Uuid::new_v4(),
            username,
            email,
            password_hash,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            status: UserStatus::Online,
        };

        // Persister en mémoire
        self.user_repo.create(user).await
    }

    /// Authentifier un utilisateur
    pub async fn authenticate(&self, email: &str, password: &str) -> AppResult<User> {
        // 1. Récupérer l'utilisateur par email
        let user = self.user_repo
            .find_by_email(email)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;

        // 2. Vérifier le mot de passe
        let is_valid = self.password_service.verify(password, &user.password_hash)?;

        if !is_valid {
            return Err(AppError::Unauthorized("Invalid credentials".to_string()));
        }

        // 3. Retourner l'utilisateur authentifié
        Ok(user)
    }

    /// Vérifier si un email existe
    pub async fn email_exists(&self, email: &str) -> AppResult<bool> {
        Ok(self.user_repo.find_by_email(email).await?.is_some())
    }

    /// Vérifier si un username existe
    pub async fn username_exists(&self, username: &str) -> AppResult<bool> {
        Ok(self.user_repo.find_by_username(username).await?.is_some())
    }

    /// Mettre à jour le statut d'un utilisateur
    pub async fn update_status(&self, user_id: Uuid, status: UserStatus) -> AppResult<User> {
        let mut user = self.user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        user.status = status;
        user.updated_at = chrono::Utc::now();

        self.user_repo.update(user).await
    }

    /// Accès au repository (pour les tests)
    pub fn repo(&self) -> &MockUserRepository {
        &self.user_repo
    }
}