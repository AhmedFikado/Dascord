use crate::utils::error::{AppError, AppResult};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

#[derive(Clone)]
pub struct PasswordService;

impl PasswordService {
    pub fn new() -> Self {
        Self
    }

    pub fn hash(&self, password: &str) -> AppResult<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| AppError::InternalServerError(format!("Password hashing failed: {}", e)))
        // LCOV_EXCL_LINE
    }

    pub fn verify(&self, password: &str, hash: &str) -> AppResult<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| AppError::InternalServerError(format!("Invalid hash: {}", e)))?;

        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let service = PasswordService::new();
        let password = "test_password123";

        let hash = service.hash(password).expect("Failed to hash password");

        assert!(!hash.is_empty());
        assert_ne!(hash, password);
    }

    #[test]
    fn test_password_verification_success() {
        let service = PasswordService::new();
        let password = "secure_password";

        let hash = service.hash(password).expect("Failed to hash");
        let is_valid = service.verify(password, &hash).expect("Failed to verify");

        assert!(is_valid);
    }

    #[test]
    fn test_password_verification_failure() {
        let service = PasswordService::new();
        let password = "correct_password";
        let wrong_password = "wrong_password";

        let hash = service.hash(password).expect("Failed to hash");
        let is_valid = service
            .verify(wrong_password, &hash)
            .expect("Failed to verify");

        assert!(!is_valid);
    }

    #[test]
    fn test_verify_with_invalid_hash() {
        let service = PasswordService::new();
        let invalid_hash = "not_a_valid_hash";

        let result = service.verify("any_password", invalid_hash);

        assert!(result.is_err());
    }

    #[test]
    fn test_hash_empty_password() {
        let service = PasswordService::new();
        let empty_password = "";

        let result = service.hash(empty_password);

        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_empty_password() {
        let service = PasswordService::new();
        let empty_password = "";

        let hash = service
            .hash(empty_password)
            .expect("Failed to hash empty password");
        let is_valid = service
            .verify(empty_password, &hash)
            .expect("Failed to verify empty password");

        assert!(is_valid);
    }

    #[test]
    fn test_hash_long_password() {
        let service = PasswordService::new();
        let long_password = "a".repeat(1000);

        let result = service.hash(&long_password);

        assert!(result.is_ok());
        let hash = result.unwrap();
        assert!(!hash.is_empty());
    }

    #[test]
    fn test_hash_special_characters() {
        let service = PasswordService::new();
        let special_password = "p@ssw0rd!#$%^&*()";

        let hash = service
            .hash(special_password)
            .expect("Failed to hash special password");
        let is_valid = service
            .verify(special_password, &hash)
            .expect("Failed to verify special password");

        assert!(is_valid);
    }
}
