use crate::domain::entities::User;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub email: String,
    pub status: String,
    pub language: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_string(),
            username: user.username,
            email: user.email,
            status: user.status,
            language: user.language,
        }
    }
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_response_from_user() {
        let user = User {
            id: "123e4567-e89b-12d3-a456-426614174000".parse().unwrap(),
            username: "Test User".to_string(),
            email: "test@example.com".to_string(),
            status: "ONLINE".to_string(),
            created_at: chrono::Utc::now(),
            password_hash: "hashed_password".to_string(),
            language: "fr".to_string(),
        };

        let user_response = UserResponse::from(user);

        assert_eq!(user_response.id, "123e4567-e89b-12d3-a456-426614174000");
        assert_eq!(user_response.username, "Test User");
        assert_eq!(user_response.email, "test@example.com");
        assert_eq!(user_response.status, "ONLINE");
        assert_eq!(user_response.language, "fr");
    }

    #[test]
    fn test_user_response_serialization() {
        let response = UserResponse {
            id: "123".to_string(),
            username: "test".to_string(),
            email: "test@test.com".to_string(),
            status: "ONLINE".to_string(),
            language: "fr".to_string(),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("test"));
    }
}
