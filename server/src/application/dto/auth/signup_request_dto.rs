use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SignupRequest {
    #[validate(length(
        min = 3,
        max = 50,
        message = "Username must be between 3 and 50 characters"
    ))]
    pub username: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    #[validate(length(
        min = 2,
        max = 10,
        message = "Language must be between 2 and 10 characters"
    ))]
    pub language: String,
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_signup_request() {
        let request = SignupRequest {
            username: "validuser".to_string(),
            email: "valid@example.com".to_string(),
            password: "validpassword".to_string(),
            language: "en".to_string(),
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_invalid_email_format() {
        let request = SignupRequest {
            username: "validuser".to_string(),
            email: "invalid-email".to_string(),
            password: "validpassword".to_string(),
            language: "en".to_string(),
        };

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_short_username() {
        let request = SignupRequest {
            username: "ab".to_string(),
            email: "valid@example.com".to_string(),
            password: "validpassword".to_string(),
            language: "en".to_string(),
        };

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_short_password() {
        let request = SignupRequest {
            username: "validuser".to_string(),
            email: "valid@example.com".to_string(),
            password: "2short".to_string(),
            language: "en".to_string(),
        };

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_long_username() {
        let request = SignupRequest {
            username: "a".repeat(51),
            email: "valid@example.com".to_string(),
            password: "validpassword".to_string(),
            language: "en".to_string(),
        };

        assert!(request.validate().is_err());
    }
}
