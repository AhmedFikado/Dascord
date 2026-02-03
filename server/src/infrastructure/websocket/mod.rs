pub mod connection;
pub mod manager;
pub mod message;
pub mod router;

pub use connection::{handle_socket, Connection};
pub use manager::ConnectionManager;
pub use message::{ClientMessage, ServerMessage};
pub use router::{create_ws_router, WebSocketState};

use crate::infrastructure::security::jwt::JWTService;
use crate::utils::error::{AppError, AppResult};
use uuid::Uuid;

/// Recup et valide le token JWT depuis la query string
pub fn extract_and_verify_token(
    query: &str,
    jwt_service: &JWTService,
) -> AppResult<(Uuid, String)> {
    // Parser la query string pour récupérer le token
    // Format: ?token=xxx
    let token = query
        .strip_prefix("token=")
        .ok_or_else(|| AppError::Unauthorized("Missing token parameter".to_string()))?;
    
    // Vérif le token JWT
    let claims = jwt_service.verify_token(token)?;
    
    // Recup le user_id
    let user_id = Uuid::parse_str(&claims.sub_id)
        .map_err(|_| AppError::Unauthorized("Invalid user_id in token".to_string()))?;
    
    // Utilise juste l'ID
    let username = format!("User_{}", &claims.sub_id[..8]);
    
    Ok((user_id, username))
}
