use crate::application::dto::security::JWTToken;
use crate::utils::error::{AppError, AppResult};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use uuid::Uuid;

#[derive(Clone)]
pub struct JWTService {
    secret: String,
}

impl JWTService {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }

    pub fn create_token(&self, user_id: Uuid) -> AppResult<String> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::days(1))
            .expect("Valid timestamp")
            .timestamp();

        let token = JWTToken {
            sub_id: user_id.to_string(),
            exp: expiration as usize,
            iat: Utc::now().timestamp() as usize,
        };

        encode(
            &Header::default(),
            &token,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| AppError::InternalServerError(format!("JWT creation failed: {}", e)))
    }

    pub fn verify_token(&self, token: &str) -> AppResult<JWTToken> {
        decode::<JWTToken>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|_| AppError::Unauthorized("Invalid token".to_string()))
    }

    #[cfg(test)]
    pub fn create_expired_token(&self, user_id: Uuid) -> AppResult<String> {
        let expiration = Utc::now()
            .checked_sub_signed(Duration::days(1))
            .expect("Valid timestamp")
            .timestamp();

        let token = JWTToken {
            sub_id: user_id.to_string(),
            exp: expiration as usize,
            iat: Utc::now().timestamp() as usize,
        };

        encode(
            &Header::default(),
            &token,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| AppError::InternalServerError(format!("JWT creation failed: {}", e)))
    }
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_token_success() {
        let jwt_service = JWTService::new("test_secret".to_string());
        let user_id = Uuid::new_v4();

        let result = jwt_service.create_token(user_id);

        assert!(result.is_ok());
        let token = result.unwrap();
        assert!(!token.is_empty());
    }

    #[test]
    fn test_verify_token_success() {
        let jwt_service = JWTService::new("test_secret".to_string());
        let user_id = Uuid::new_v4();

        let token = jwt_service.create_token(user_id).unwrap();
        let result = jwt_service.verify_token(&token);

        assert!(result.is_ok());
        let claims = result.unwrap();
        assert_eq!(claims.sub_id, user_id.to_string());
    }

    #[test]
    fn test_verify_invalid_token() {
        let jwt_service = JWTService::new("test_secret".to_string());

        let result = jwt_service.verify_token("invalid_token");

        assert!(result.is_err());
    }

    #[test]
    fn test_verify_token_wrong_secret() {
        let jwt_service1 = JWTService::new("secret1".to_string());
        let jwt_service2 = JWTService::new("secret2".to_string());
        let user_id = Uuid::new_v4();

        let token = jwt_service1.create_token(user_id).unwrap();
        let result = jwt_service2.verify_token(&token);

        assert!(result.is_err());
    }

    #[test]
    fn test_verify_expired_token() {
        let jwt_service = JWTService::new("test_secret".to_string());
        let user_id = Uuid::new_v4();

        let expired_token = jwt_service.create_expired_token(user_id).unwrap();
        let result = jwt_service.verify_token(&expired_token);

        assert!(result.is_err());
    }

    #[test]
    fn test_token_contains_correct_claims() {
        let jwt_service = JWTService::new("test_secret".to_string());
        let user_id = Uuid::new_v4();

        let token = jwt_service.create_token(user_id).unwrap();
        let claims = jwt_service.verify_token(&token).unwrap();

        assert_eq!(claims.sub_id, user_id.to_string());
        assert!(claims.exp > claims.iat);
    }

    #[test]
    fn test_empty_secret() {
        let jwt_service = JWTService::new("".to_string());
        let user_id = Uuid::new_v4();

        let result = jwt_service.create_token(user_id);

        assert!(result.is_ok());
    }
}
