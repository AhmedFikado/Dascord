use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_valid_login_request() {
        let request = LoginRequest {
            email: "user@example.com".to_string(),
            password: "password123".to_string(),
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_invalid_email_format() {
        let request = LoginRequest {
            email: "invalid-email".to_string(),
            password: "password123".to_string(),
        };

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_empty_password() {
        let request = LoginRequest {
            email: "user@example.com".to_string(),
            password: "".to_string(),
        };

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_empty_email() {
        let request = LoginRequest {
            email: "".to_string(),
            password: "password".to_string(),
        };

        assert!(request.validate().is_err());
    }
}
