use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserDto {
    pub id: String,
    pub username: String,
    pub email: String,
    pub status: String,
}



// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_dto_creation() {
        let dto = UserDto {
            id: "123".to_string(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            status: "ONLINE".to_string(),
        };

        assert_eq!(dto.username, "testuser");
        assert_eq!(dto.email, "test@example.com");
    }

    #[test]
    fn test_user_dto_serialization() {
        let dto = UserDto {
            id: "123".to_string(),
            username: "test".to_string(),
            email: "test@test.com".to_string(),
            status: "OFFLINE".to_string(),
        };
        let json = serde_json::to_string(&dto).unwrap();
        assert!(json.contains("test"));
    }
}
