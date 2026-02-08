pub mod connection;
pub mod manager;
pub mod message;
pub mod router;

pub use connection::{handle_socket, Connection};
pub use manager::ConnectionManager;
pub use message::{ClientMessage, ServerMessage};
pub use router::{create_ws_router, WebSocketState};

use crate::infrastructure::security::jwt::JWTService;
use crate::infrastructure::repositories::{UserRepository, PostgresUserRepository};
use crate::utils::error::{AppError, AppResult};
use uuid::Uuid;

/// Recup et valide le token JWT depuis la query string
pub async fn extract_and_verify_token(
    query: &str,
    jwt_service: &JWTService,
    user_repository: &PostgresUserRepository,
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
    
    // Récup le username depuis la db
    let user = user_repository.find_by_id(user_id).await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;
    
    Ok((user_id, user.username))
}
