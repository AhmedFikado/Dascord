pub mod auth_dto;
pub mod message_dto;
pub mod user_dto;

pub use auth_dto::*;
pub use message_dto::*;
pub use user_dto::*;

use serde::{Deserialize, Serialize};

/// Claims JWT pour l'authentification
#[derive(Debug, Serialize, Deserialize)]
pub struct JWTToken {
    pub sub_id: String,  // Subject: user ID
    pub exp: usize,      // Expiration timestamp
    pub iat: usize,      // Issued at timestamp
}
