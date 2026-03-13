use super::user_response_dto::UserResponse;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    pub message: String,
    pub user: UserResponse,
    pub token: String,
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login_response() {
        let user = UserResponse {
            id: "123e4567-e89b-12d3-a456-426614174000".to_string(),
            username: "Test User".to_string(),
            email: "test@example.com".to_string(),
            language: "en".to_string(),
            status: "ONLINE".to_string(),
        };

        let response = LoginResponse {
            message: "Login successful".to_string(),
            user,
            token: "test_token".to_string(),
        };

        assert_eq!(response.message, "Login successful");
        assert_eq!(response.user.email, "test@example.com");
        assert_eq!(response.user.username, "Test User");
        assert_eq!(response.token, "test_token");
    }

    #[test]
    fn test_login_response_serialization() {
        let user = UserResponse {
            id: "123".to_string(),
            username: "test".to_string(),
            language: "en".to_string(),
            email: "test@test.com".to_string(),
            status: "ONLINE".to_string(),
        };
        let response = LoginResponse {
            message: "Success".to_string(),
            user,
            token: "token".to_string(),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("Success"));
    }
}
