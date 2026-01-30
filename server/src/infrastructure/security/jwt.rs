use crate::application::dto::JWTToken;
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

    /// Créer un token JWT
    pub fn create_token(&self, user_id: Uuid) -> AppResult<String> {
        // Calculer la date d'expiration (1 jour)
        let expiration = Utc::now()
            .checked_add_signed(Duration::days(1))
            .expect("Valid timestamp")
            .timestamp();

        // Claims pour le token
        let token = JWTToken {
            sub_id: user_id.to_string(),
            exp: expiration as usize,
            iat: Utc::now().timestamp() as usize,
        };

        // Créer le token
        encode(
            &Header::default(),
            &token,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| AppError::InternalServerError(format!("JWT creation failed: {}", e)))
    }

    /// Vérifier et décoder un token JWT
    pub fn verify_token(&self, token: &str) -> AppResult<JWTToken> {
        decode::<JWTToken>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|_| AppError::Unauthorized("Invalid token".to_string()))
    }
}
