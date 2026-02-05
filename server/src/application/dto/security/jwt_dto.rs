use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JWTToken {
    pub sub_id: String, // User ID
    pub exp: usize,     // Expiration timestamp
    pub iat: usize,     // Issued at timestamp
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_token_creation() {
        let token = JWTToken {
            sub_id: "user123".to_string(),
            exp: 1234567890,
            iat: 1234567800,
        };

        assert_eq!(token.sub_id, "user123");
        assert_eq!(token.exp, 1234567890);
        assert_eq!(token.iat, 1234567800);
    }

    #[test]
    fn test_jwt_token_serialization() {
        let token = JWTToken {
            sub_id: "user123".to_string(),
            exp: 1234567890,
            iat: 1234567800,
        };

        let json = serde_json::to_string(&token).unwrap();
        assert!(json.contains("user123"));
    }

    #[test]
    fn test_jwt_token_deserialization() {
        let json = r#"{"sub_id":"user123","exp":1234567890,"iat":1234567800}"#;
        let token: JWTToken = serde_json::from_str(json).unwrap();
        assert_eq!(token.sub_id, "user123");
    }
}
