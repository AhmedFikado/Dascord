use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct LogoutResponse {
    pub message: String,
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logout_response() {
        let logout_response = LogoutResponse {
            message: "Successfully logged out".to_string(),
        };

        assert_eq!(logout_response.message, "Successfully logged out");
    }

    #[test]
    fn test_logout_response_serialization() {
        let response = LogoutResponse {
            message: "Test".to_string(),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("Test"));
    }
}
