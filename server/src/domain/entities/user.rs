use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl User {
    pub fn new(username: String, email: String, password_hash: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            username,
            email,
            password_hash,
            status: "OFFLINE".to_string(),
            created_at: chrono::Utc::now(),
        }
    }
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new(
            "testuser".to_string(),
            "test@example.com".to_string(),
            "hashed_password".to_string(),
        );

        assert_eq!(user.username, "testuser");
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.password_hash, "hashed_password");
        assert_eq!(user.status, "OFFLINE");
    }

    #[test]
    fn test_user_default_status() {
        let user = User::new(
            "user".to_string(),
            "user@test.com".to_string(),
            "hash".to_string(),
        );

        assert_eq!(user.status, "OFFLINE");
    }

    #[test]
    fn test_user_has_unique_id() {
        let user1 = User::new(
            "user1".to_string(),
            "user1@test.com".to_string(),
            "hash".to_string(),
        );
        let user2 = User::new(
            "user2".to_string(),
            "user2@test.com".to_string(),
            "hash".to_string(),
        );

        assert_ne!(user1.id, user2.id);
    }
}
