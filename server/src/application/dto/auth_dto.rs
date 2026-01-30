use serde::{Deserialize, Serialize};
use validator::Validate;

use super::UserDto;

/// DTO pour la requête de signup
#[derive(Debug, Deserialize, Validate)]
pub struct SignupRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 8))]
    pub password: String,
}

/// DTO pour la requête de login
#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 1))]
    pub password: String,
}

/// DTO pour la réponse d'authentification (login/signup)
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub message: String,
    pub user: UserDto,
    pub token: String,
}

/// DTO pour la réponse de logout
#[derive(Debug, Serialize)]
pub struct LogoutResponse {
    pub message: String,
}
