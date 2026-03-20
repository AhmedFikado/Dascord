use crate::domain::services::message::*;
use crate::infrastructure::repositories::UserRepository;
use crate::infrastructure::repositories::message::MessageRepository;
use crate::infrastructure::repositories::channel::PrivateChannelRepository;
use crate::infrastructure::security::JWTService;
use crate::infrastructure::websocket::ConnectionManager;
use crate::utils::error::AppError;
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;


pub struct PrivateMessageController<
    MR: MessageRepository,
    PCR: PrivateChannelRepository,
    UR: UserRepository,
> {
    jwt_service: Arc<JWTService>,
    services: PrivateMessageServices<MR, PCR>,
    user_repo: UR,
    ws_manager: Option<Arc<ConnectionManager>>,
}

impl<MR: MessageRepository, PCR: PrivateChannelRepository, UR: UserRepository>
    PrivateMessageController<MR, PCR, UR>
{
    pub fn new(
        jwt_service: JWTService,
        message_repo: MR,
        channel_repo: PCR,
        user_repo: UR
    ) -> Self {
        let services = PrivateMessageServices::new(message_repo, channel_repo);
        
        Self {
            jwt_service: Arc::new(jwt_service),
            services,
            user_repo,
            ws_manager: None,
        }
    }

    pub fn with_ws_manager(mut self, ws_manager: Arc<ConnectionManager>) -> Self {
        self.ws_manager = Some(ws_manager);
        self
    }
}

/// - Envoyer un message privé dans un channel
#[utoipa::path(
    post,
    path = "/channels/{channel_id}/messages/private",
    tag = "messages",
    request_body = serde_json::Value,
    responses(
        (status = 201, description = "Message envoyé avec succès", body = MessageDto),
        (status = 401, description = "Non autorisé"),
        (status = 404, description = "Channel non trouvé")
    ),
    security(("bearer_auth" = []))
)]
pub async fn send_private_message<MR: MessageRepository, PCR: PrivateChannelRepository, UR: UserRepository>(
    State(controller): State<Arc<PrivateMessageController<MR, PCR, UR>>>,
    Path(channel_id): Path<Uuid>,
    headers: HeaderMap,
    Json(payload): Json<serde_json::Value>,
) -> Result<impl IntoResponse, AppError> {
    let token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| {
            AppError::Unauthorized("Missing or invalid Authorization header".to_string())
        })?;

    let claims = controller.jwt_service.verify_token(token)?;
    let user_id = Uuid::parse_str(&claims.sub_id)
        .map_err(|_| AppError::Unauthorized("Invalid user ID".to_string()))?;

    let content = payload
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::ValidationError("Missing content field".to_string()))?
        .to_string();

    let user = controller
        .user_repo
        .find_by_id(user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    let message = controller
    .services
    .send_message(channel_id, user_id, user.username, content).await?;

    // Diffuser le nouveau message à tous les clients du channel via WebSocket
    if let Some(ws_manager) = &controller.ws_manager {
        ws_manager.broadcast_to_channel(
            &channel_id.to_string(),
            crate::infrastructure::websocket::ServerMessage::NewMessage {
                channel_id: channel_id.to_string(),
                message_id: message.id.clone().unwrap_or_default(),
                user_id: message.user_id.clone(),
                username: message.username.clone(),
                content: message.content.clone(),
                created_at: chrono::Utc::now(),
            },
        ).await;
    }

    Ok((StatusCode::CREATED, Json(message)))
}

/// - Obtenir les messages d'un channel privé (historique)
#[utoipa::path(
    get,
    path = "/channels/{channel_id}/messages/private",
    tag = "messages",
    responses(
        (status = 201, description = "Historique des messages récupéré avec succès", body = MessageDto),
        (status = 401, description = "Non autorisé"),
        (status = 404, description = "Channel non trouvé")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_private_message_history<MR: MessageRepository, PCR: PrivateChannelRepository, UR: UserRepository>(
    State(controller): State<Arc<PrivateMessageController<MR, PCR, UR>>>,
    Path(channel_id): Path<Uuid>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    let token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| {
            AppError::Unauthorized("Missing or invalid Authorization header".to_string())
        })?;

    let claims = controller.jwt_service.verify_token(token)?;
    let user_id = Uuid::parse_str(&claims.sub_id)
        .map_err(|_| AppError::Unauthorized("Invalid user ID".to_string()))?;
    
    let messages = controller
    .services
    .get_message_history(channel_id, user_id).await?;

    Ok((StatusCode::OK, Json(messages)))
}

/// - Supprimer un message privé
#[utoipa::path(
    delete,
    path = "/messages/private/{id}",
    tag = "messages",
    responses(
        (status = 201, description = "Message supprimé avec succès", body = serde_json::json),
        (status = 401, description = "Non autorisé"),
        (status = 404, description = "Message non trouvé")
    ),
    security(("bearer_auth" = []))
)]
pub async fn delete_private_message<MR: MessageRepository, PCR: PrivateChannelRepository, UR: UserRepository>(
    State(controller): State<Arc<PrivateMessageController<MR, PCR, UR>>>,
    Path(id): Path<String>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    let token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| {
            AppError::Unauthorized("Missing or invalid Authorization header".to_string())
        })?;

    let claims = controller.jwt_service.verify_token(token)?;
    let user_id = Uuid::parse_str(&claims.sub_id)
        .map_err(|_| AppError::Unauthorized("Invalid user ID".to_string()))?;

    let channel_id = controller.services.delete_message(id.clone(), user_id).await?;
    
    // Diffuser l'événement WebSocket à tous les clients du channel
    if let Some(ws_manager) = &controller.ws_manager {
        ws_manager.broadcast_to_channel(
            &channel_id,
            crate::infrastructure::websocket::ServerMessage::MessageDeleted {
                channel_id: channel_id.clone(),
                message_id: id,
            },
        ).await;
    }
    
    Ok((
        StatusCode::OK,
        Json(serde_json::json!({"message": "Message deleted"})),
    ))
}

/// - Modifier un message privé
#[utoipa::path(
    put,
    path = "/messages/private/{id}",
    tag = "messages",
    request_body = serde_json::Value,
    responses(
        (status = 201, description = "Message mis à jour avec succès", body = MessageDto),
        (status = 401, description = "Non autorisé"),
        (status = 404, description = "Message non trouvé")
    ),
    security(("bearer_auth" = []))
)]    
pub async fn update_private_message<MR: MessageRepository, PCR: PrivateChannelRepository, UR: UserRepository>(
    State(controller): State<Arc<PrivateMessageController<MR, PCR, UR>>>,
    Path(id): Path<String>,
    headers: HeaderMap,
    Json(payload): Json<serde_json::Value>,
) -> Result<impl IntoResponse, AppError> {
    let token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| {
            AppError::Unauthorized("Missing or invalid Authorization header".to_string())
        })?;

    let claims = controller.jwt_service.verify_token(token)?;
    let user_id = Uuid::parse_str(&claims.sub_id)
        .map_err(|_| AppError::Unauthorized("Invalid user ID".to_string()))?;

    let content = payload
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::ValidationError("Missing content field".to_string()))?
        .to_string();

    let updated_message = controller.services.update_private_message(id.clone(), user_id, content).await?;
    
    // Diffuser l'événement WebSocket à tous les clients du channel
    if let Some(ws_manager) = &controller.ws_manager {
        ws_manager.broadcast_to_channel(
            &updated_message.channel_id,
            crate::infrastructure::websocket::ServerMessage::MessageUpdated {
                channel_id: updated_message.channel_id.clone(),
                message_id: id,
                user_id: updated_message.user_id.clone(),
                content: updated_message.content.clone(),
            },
        ).await;
    }
    
    Ok((StatusCode::OK, Json(updated_message)))
}

/// - Ajouter une réaction à un message privé
pub async fn add_private_reaction<MR: MessageRepository, PCR: PrivateChannelRepository, UR: UserRepository>(
    State(controller): State<Arc<PrivateMessageController<MR, PCR, UR>>>,
    Path(id): Path<String>,
    headers: HeaderMap,
    Json(payload): Json<serde_json::Value>,
) -> Result<impl IntoResponse, AppError> {
    let token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".to_string()))?;

    let claims = controller.jwt_service.verify_token(token)?;
    let user_id = Uuid::parse_str(&claims.sub_id)
        .map_err(|_| AppError::Unauthorized("Invalid user ID".to_string()))?;

    let reaction = payload
        .get("reaction")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::ValidationError("Missing reaction field".to_string()))?
        .to_string();

    let updated_message = controller.services.add_reaction(id.clone(), user_id, reaction.clone()).await?;

    if let Some(ws_manager) = &controller.ws_manager {
        ws_manager.broadcast_to_channel(
            &updated_message.channel_id,
            crate::infrastructure::websocket::ServerMessage::ReactionAdded {
                channel_id: updated_message.channel_id.clone(),
                message_id: id,
                user_id: user_id.to_string(),
                reaction,
            },
        ).await;
    }

    Ok((StatusCode::OK, Json(updated_message)))
}

/// - Supprimer une réaction d'un message privé
pub async fn remove_private_reaction<MR: MessageRepository, PCR: PrivateChannelRepository, UR: UserRepository>(
    State(controller): State<Arc<PrivateMessageController<MR, PCR, UR>>>,
    Path((id, reaction)): Path<(String, String)>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    let token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".to_string()))?;

    let claims = controller.jwt_service.verify_token(token)?;
    let user_id = Uuid::parse_str(&claims.sub_id)
        .map_err(|_| AppError::Unauthorized("Invalid user ID".to_string()))?;

    let updated_message = controller.services.remove_reaction(id.clone(), user_id, reaction.clone()).await?;

    if let Some(ws_manager) = &controller.ws_manager {
        ws_manager.broadcast_to_channel(
            &updated_message.channel_id,
            crate::infrastructure::websocket::ServerMessage::ReactionRemoved {
                channel_id: updated_message.channel_id.clone(),
                message_id: id,
                user_id: user_id.to_string(),
                reaction,
            },
        ).await;
    }

    Ok((StatusCode::OK, Json(updated_message)))
}

