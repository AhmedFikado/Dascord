use super::user_response_dto::UserResponse;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SignupResponse {
    pub user: UserResponse,
    pub token: String,
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signup_response() {
        let user = UserResponse {
            id: "123e4567-e89b-12d3-a456-426614174000".to_string(),
            username: "New User".to_string(),
            email: "newuser@example.com".to_string(),
            status: "OFFLINE".to_string(),
        };
        let response = SignupResponse {
            user,
            token: "signup_token".to_string(),
        };

        assert_eq!(response.user.username, "New User");
        assert_eq!(response.user.email, "newuser@example.com");
        assert_eq!(response.user.id, "123e4567-e89b-12d3-a456-426614174000");
        assert_eq!(response.token, "signup_token");
        assert_eq!(response.user.status, "OFFLINE");
    }

    #[test]
    fn test_signup_response_serialization() {
        let user = UserResponse {
            id: "123".to_string(),
            username: "test".to_string(),
            email: "test@test.com".to_string(),
            status: "OFFLINE".to_string(),
        };
        let response = SignupResponse {
            user,
            token: "token".to_string(),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("token"));
    }
}
