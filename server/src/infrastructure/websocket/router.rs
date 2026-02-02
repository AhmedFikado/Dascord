use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        Query, State,
    },
    response::Response,
    routing::get,
    Router,
};
use std::sync::Arc;

use crate::infrastructure::security::jwt::JWTService;
use crate::infrastructure::websocket::{extract_and_verify_token, handle_socket, ConnectionManager};
use crate::utils::error::AppError;

/// État partagé pour les routes WebSocket
#[derive(Clone)]
pub struct WebSocketState {
    pub manager: Arc<ConnectionManager>,
    pub jwt_service: JWTService,
}

/// Handler pour la route WebSocket
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Query(params): Query<std::collections::HashMap<String, String>>,
    State(state): State<WebSocketState>,
) -> Result<Response, AppError> {
    // Récup et vérif le token
    let token = params
        .get("token")
        .ok_or_else(|| AppError::Unauthorized("Missing token parameter".to_string()))?;
    
    let query_string = format!("token={}", token);
    let (user_id, username) = extract_and_verify_token(&query_string, &state.jwt_service)?;
    
    // Accepter la connexion WebSocket
    Ok(ws.on_upgrade(move |socket| {
        handle_connection(socket, user_id, username, state.manager)
    }))
}

/// Gérer la connexion WebSocket après l'upgrade
async fn handle_connection(
    socket: WebSocket,
    user_id: uuid::Uuid,
    username: String,
    manager: Arc<ConnectionManager>,
) {
    tracing::info!("Nouvelle connexion WebSocket: user_id={}, username={}", user_id, username);
    handle_socket(socket, user_id, username, manager).await;
    tracing::info!("Connexion WebSocket fermée: user_id={}", user_id);
}

/// Créer le router WebSocket
pub fn create_ws_router(state: WebSocketState) -> Router {
    Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state)
}
