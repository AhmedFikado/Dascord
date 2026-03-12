use crate::application::use_cases::message::*;
use crate::infrastructure::repositories::{
    ChannelRepository, MessageRepository, ServerRepository, UserRepository,
};
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

#[derive(Clone)]
pub struct MessageHandler<
    MR: MessageRepository,
    CR: ChannelRepository,
    SR: ServerRepository,
    UR: UserRepository,
> {
    jwt_service: Arc<JWTService>,
    send_message_uc: Arc<SendMessageUseCase<MR, CR, SR>>,
    get_history_uc: Arc<GetMessageHistoryUseCase<MR, CR, SR>>,
    delete_message_uc: Arc<DeleteMessageUseCase<MR, CR, SR>>,
    update_message_uc: Arc<UpdateMessageUseCase<MR, CR, SR>>,
    user_repo: Arc<UR>,
    ws_manager: Option<Arc<ConnectionManager>>,
}

impl<MR: MessageRepository, CR: ChannelRepository, SR: ServerRepository, UR: UserRepository>
    MessageHandler<MR, CR, SR, UR>
{
    pub fn new(
        jwt_service: JWTService,
        message_repo: MR,
        channel_repo: CR,
        server_repo: SR,
        user_repo: UR,
    ) -> Self {
        Self {
            jwt_service: Arc::new(jwt_service),
            send_message_uc: Arc::new(SendMessageUseCase::new(
                message_repo.clone(),
                channel_repo.clone(),
                server_repo.clone(),
            )),
            get_history_uc: Arc::new(GetMessageHistoryUseCase::new(
                message_repo.clone(),
                channel_repo.clone(),
                server_repo.clone(),
            )),
            delete_message_uc: Arc::new(DeleteMessageUseCase::new(
                message_repo.clone(),
                channel_repo.clone(),
                server_repo.clone(),
            )),
            update_message_uc: Arc::new(UpdateMessageUseCase::new(
                message_repo,
                channel_repo,
                server_repo,
            )),
            user_repo: Arc::new(user_repo),
            ws_manager: None,
        }
    }

    pub fn with_ws_manager(mut self, ws_manager: Arc<ConnectionManager>) -> Self {
        self.ws_manager = Some(ws_manager);
        self
    }
}

/// - Envoyer un message dans un channel
#[utoipa::path(
    post,
    path = "/channels/{channel_id}/messages",
    tag = "messages",
    request_body = serde_json::Value,
    responses(
        (status = 201, description = "Message envoyé avec succès", body = MessageDto),
        (status = 401, description = "Non autorisé"),
        (status = 404, description = "Channel non trouvé")
    ),
    security(("bearer_auth" = []))
)]
pub async fn send_message<MR: MessageRepository, CR: ChannelRepository, SR: ServerRepository, UR: UserRepository>(
    State(handler): State<Arc<MessageHandler<MR, CR, SR, UR>>>,
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

    let claims = handler.jwt_service.verify_token(token)?;
    let user_id = Uuid::parse_str(&claims.sub_id)
        .map_err(|_| AppError::Unauthorized("Invalid user ID".to_string()))?;

    let content = payload
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::ValidationError("Missing content field".to_string()))?
        .to_string();

    let user = handler
        .user_repo
        .find_by_id(user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    let message = handler
        .send_message_uc
        .execute(channel_id, user_id, user.username, content)
        .await?;
    Ok((StatusCode::CREATED, Json(message)))
}

/// - Obtenir les messages d'un channel (historique)
#[utoipa::path(
    get,
    path = "/channels/{channel_id}/messages",
    tag = "messages",
    responses(
        (status = 201, description = "Historique des messages récupéré avec succès", body = MessageDto),
        (status = 401, description = "Non autorisé"),
        (status = 404, description = "Channel non trouvé")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_message_history<MR: MessageRepository, CR: ChannelRepository, SR: ServerRepository, UR: UserRepository>(
    State(handler): State<Arc<MessageHandler<MR, CR, SR, UR>>>,
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

    let claims = handler.jwt_service.verify_token(token)?;
    let user_id = Uuid::parse_str(&claims.sub_id)
        .map_err(|_| AppError::Unauthorized("Invalid user ID".to_string()))?;
    
    let messages = handler.get_history_uc.execute(channel_id, user_id).await?;
    Ok((StatusCode::OK, Json(messages)))
}

/// - Supprimer un message
#[utoipa::path(
    delete,
    path = "/messages/{id}",
    tag = "messages",
    responses(
        (status = 201, description = "Message supprimé avec succès", body = serde_json::json),
        (status = 401, description = "Non autorisé"),
        (status = 404, description = "Message non trouvé")
    ),
    security(("bearer_auth" = []))
)]
pub async fn delete_message<MR: MessageRepository, CR: ChannelRepository, SR: ServerRepository, UR: UserRepository>(
    State(handler): State<Arc<MessageHandler<MR, CR, SR, UR>>>,
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

    let claims = handler.jwt_service.verify_token(token)?;
    let user_id = Uuid::parse_str(&claims.sub_id)
        .map_err(|_| AppError::Unauthorized("Invalid user ID".to_string()))?;

    let channel_id = handler.delete_message_uc.execute(id.clone(), user_id).await?;
    
    // Diffuser l'événement WebSocket à tous les clients du channel
    if let Some(ws_manager) = &handler.ws_manager {
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

/// - Modifier un message
#[utoipa::path(
    put,
    path = "/messages/{id}",
    tag = "messages",
    request_body = serde_json::Value,
    responses(
        (status = 201, description = "Message mis à jour avec succès", body = MessageDto),
        (status = 401, description = "Non autorisé"),
        (status = 404, description = "Message non trouvé")
    ),
    security(("bearer_auth" = []))
)]    
pub async fn update_message<MR: MessageRepository, CR: ChannelRepository, SR: ServerRepository, UR: UserRepository>(
    State(handler): State<Arc<MessageHandler<MR, CR, SR, UR>>>,
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

    let claims = handler.jwt_service.verify_token(token)?;
    let user_id = Uuid::parse_str(&claims.sub_id)
        .map_err(|_| AppError::Unauthorized("Invalid user ID".to_string()))?;

    let content = payload
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::ValidationError("Missing content field".to_string()))?
        .to_string();

    let updated_message = handler.update_message_uc.execute(id.clone(), user_id, content).await?;
    
    // Diffuser l'événement WebSocket à tous les clients du channel
    if let Some(ws_manager) = &handler.ws_manager {
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

/// - Envoyer un message de bienvenue dans un channel lorsqu'un utilisateur rejoint le serveur
#[utoipa::path(
    post,
    path = "/channels/{channel_id}/messages/welcome",
    tag = "messages",
    responses(
        (status = 201, description = "Message envoyé avec succès", body = MessageDto),
        (status = 401, description = "Non autorisé"),
        (status = 404, description = "Channel non trouvé")
    ),
    security(("bearer_auth" = []))
)]
pub async fn send_welcome_message<MR: MessageRepository, CR: ChannelRepository, SR: ServerRepository, UR: UserRepository>(
    State(handler): State<Arc<MessageHandler<MR, CR, SR, UR>>>,
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

    let claims = handler.jwt_service.verify_token(token)?;
    let user_id = Uuid::parse_str(&claims.sub_id)
        .map_err(|_| AppError::Unauthorized("Invalid user ID".to_string()))?;

    let user = handler
        .user_repo
        .find_by_id(user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    let content = format!("👋 Bienvenue {} ! Tu as trouvé le serveur ! On a des pokémons légendaires pour toi !"
    , user.username);
    let system_username = "Système".to_string();

    let message = handler
        .send_message_uc
        .execute(channel_id, user_id, system_username, content)
        .await?;
    
    if let Some(ws_manager) = &handler.ws_manager {
        ws_manager.broadcast_to_channel(
            &channel_id.to_string(),
            crate::infrastructure::websocket::ServerMessage::NewMessage {
                channel_id: channel_id.to_string(),
                message_id: message.id.clone().unwrap_or_default(),
                user_id: user_id.to_string(),
                username: "Système".to_string(),
                content: format!("👋 Bienvenue {} ! Tu as trouvé le serveur ! On a des pokémons légendaires pour toi !", user.username),
                created_at: chrono::Utc::now(),
            },
        ).await;
    }
    
    Ok((StatusCode::CREATED, Json(message)))
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::{Channel, User};
    use crate::domain::value_objects::ServerRole;
    use crate::infrastructure::repositories::mocks::{
        mock_channel_repository::MockChannelRepository,
        mock_message_repository::MockMessageRepository,
        mock_server_repository::MockServerRepository, mock_user_repository::MockUserRepository,
    };
    use crate::infrastructure::security::PasswordService;

    #[tokio::test]
    async fn test_send_message_success() {
        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let channel = Channel::new(server_id, "General".to_string());

        let password_service = PasswordService::new();
        let user = User {
            id: user_id,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: password_service.hash("password").unwrap(),
            status: "ONLINE".to_string(),
            created_at: chrono::Utc::now(),
        };

        let mock_message_repo = MockMessageRepository::new();
        let mock_channel_repo = MockChannelRepository::new().with_channel(channel.clone());
        let mock_server_repo =
            MockServerRepository::new().with_member(server_id, user_id, ServerRole::Member);
        let mock_user_repo = MockUserRepository::new().with_user(user);
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(MessageHandler::new(
            jwt_service.clone(),
            mock_message_repo,
            mock_channel_repo,
            mock_server_repo,
            mock_user_repo,
        ));

        let token = jwt_service.create_token(user_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let payload = serde_json::json!({"content": "Hello World"});

        let result =
            MessageHandler::send_message(State(handler), Path(channel.id), headers, Json(payload))
                .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_send_message_missing_token() {
        let channel_id = Uuid::new_v4();
        let mock_message_repo = MockMessageRepository::new();
        let mock_channel_repo = MockChannelRepository::new();
        let mock_server_repo = MockServerRepository::new();
        let mock_user_repo = MockUserRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(MessageHandler::new(
            jwt_service,
            mock_message_repo,
            mock_channel_repo,
            mock_server_repo,
            mock_user_repo,
        ));

        let headers = HeaderMap::new();
        let payload = serde_json::json!({"content": "Hello"});

        let result = MessageHandler::send_message(State(handler), Path(channel_id), headers, Json(payload)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_send_message_missing_content() {
        let user_id = Uuid::new_v4();
        let channel_id = Uuid::new_v4();
        let mock_message_repo = MockMessageRepository::new();
        let mock_channel_repo = MockChannelRepository::new();
        let mock_server_repo = MockServerRepository::new();
        let mock_user_repo = MockUserRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(MessageHandler::new(
            jwt_service.clone(),
            mock_message_repo,
            mock_channel_repo,
            mock_server_repo,
            mock_user_repo,
        ));

        let token = jwt_service.create_token(user_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());
        let payload = serde_json::json!({});

        let result = MessageHandler::send_message(State(handler), Path(channel_id), headers, Json(payload)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_message_history_success() {
        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let channel = Channel::new(server_id, "General".to_string());

        let mock_message_repo = MockMessageRepository::new();
        let mock_channel_repo = MockChannelRepository::new().with_channel(channel.clone());
        let mock_server_repo =
            MockServerRepository::new().with_member(server_id, user_id, ServerRole::Member);
        let mock_user_repo = MockUserRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(MessageHandler::new(
            jwt_service.clone(),
            mock_message_repo,
            mock_channel_repo,
            mock_server_repo,
            mock_user_repo,
        ));

        let token = jwt_service.create_token(user_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let result =
            MessageHandler::get_message_history(State(handler), Path(channel.id), headers).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_message_history_missing_token() {
        let channel_id = Uuid::new_v4();
        let mock_message_repo = MockMessageRepository::new();
        let mock_channel_repo = MockChannelRepository::new();
        let mock_server_repo = MockServerRepository::new();
        let mock_user_repo = MockUserRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(MessageHandler::new(
            jwt_service,
            mock_message_repo,
            mock_channel_repo,
            mock_server_repo,
            mock_user_repo,
        ));

        let headers = HeaderMap::new();
        let result = MessageHandler::get_message_history(State(handler), Path(channel_id), headers).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_message_success() {
        use crate::domain::entities::Message;
        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let channel = Channel::new(server_id, "General".to_string());
        
        let message = Message::new(
            channel.id.to_string(),
            user_id.to_string(),
            "TestUser".to_string(),
            "Test message".to_string(),
        );

        let mock_message_repo = MockMessageRepository::new().with_message(message);
        let mock_channel_repo = MockChannelRepository::new().with_channel(channel.clone());
        let mock_server_repo = MockServerRepository::new().with_member(server_id, user_id, ServerRole::Member);
        let mock_user_repo = MockUserRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(MessageHandler::new(
            jwt_service.clone(),
            mock_message_repo,
            mock_channel_repo,
            mock_server_repo,
            mock_user_repo,
        ));

        let token = jwt_service.create_token(user_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let result =
            MessageHandler::delete_message(State(handler), Path("msg_1".to_string()), headers)
                .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_message_success() {
        use crate::domain::entities::Message;
        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let channel = Channel::new(server_id, "General".to_string());
        
        let message = Message::new(
            channel.id.to_string(),
            user_id.to_string(),
            "TestUser".to_string(),
            "Original content".to_string(),
        );

        let mock_message_repo = MockMessageRepository::new().with_message(message);
        let mock_channel_repo = MockChannelRepository::new().with_channel(channel.clone());
        let mock_server_repo = MockServerRepository::new().with_member(server_id, user_id, ServerRole::Member);
        let mock_user_repo = MockUserRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(MessageHandler::new(
            jwt_service.clone(),
            mock_message_repo,
            mock_channel_repo,
            mock_server_repo,
            mock_user_repo,
        ));

        let token = jwt_service.create_token(user_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let payload = serde_json::json!({"content": "Updated message"});

        let result =
            MessageHandler::update_message(State(handler), Path("msg_1".to_string()), headers, Json(payload))
                .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_message_missing_token() {
        let mock_message_repo = MockMessageRepository::new();
        let mock_channel_repo = MockChannelRepository::new();
        let mock_server_repo = MockServerRepository::new();
        let mock_user_repo = MockUserRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(MessageHandler::new(
            jwt_service,
            mock_message_repo,
            mock_channel_repo,
            mock_server_repo,
            mock_user_repo,
        ));

        let headers = HeaderMap::new();
        let payload = serde_json::json!({"content": "Updated"});
        let result = MessageHandler::update_message(State(handler), Path("msg_1".to_string()), headers, Json(payload)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_message_missing_content() {
        let user_id = Uuid::new_v4();
        let mock_message_repo = MockMessageRepository::new();
        let mock_channel_repo = MockChannelRepository::new();
        let mock_server_repo = MockServerRepository::new();
        let mock_user_repo = MockUserRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(MessageHandler::new(
            jwt_service.clone(),
            mock_message_repo,
            mock_channel_repo,
            mock_server_repo,
            mock_user_repo,
        ));

        let token = jwt_service.create_token(user_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());
        let payload = serde_json::json!({});

        let result = MessageHandler::update_message(State(handler), Path("msg_1".to_string()), headers, Json(payload)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_send_message_invalid_token() {
        let channel_id = Uuid::new_v4();
        let mock_message_repo = MockMessageRepository::new();
        let mock_channel_repo = MockChannelRepository::new();
        let mock_server_repo = MockServerRepository::new();
        let mock_user_repo = MockUserRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(MessageHandler::new(
            jwt_service,
            mock_message_repo,
            mock_channel_repo,
            mock_server_repo,
            mock_user_repo,
        ));

        let mut headers = HeaderMap::new();
        headers.insert("Authorization", "Bearer invalid_token".parse().unwrap());
        let payload = serde_json::json!({"content": "Hello"});

        let result = MessageHandler::send_message(State(handler), Path(channel_id), headers, Json(payload)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_message_missing_token() {
        let mock_message_repo = MockMessageRepository::new();
        let mock_channel_repo = MockChannelRepository::new();
        let mock_server_repo = MockServerRepository::new();
        let mock_user_repo = MockUserRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(MessageHandler::new(
            jwt_service,
            mock_message_repo,
            mock_channel_repo,
            mock_server_repo,
            mock_user_repo,
        ));

        let headers = HeaderMap::new();
        let result = MessageHandler::delete_message(State(handler), Path("msg_1".to_string()), headers).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_send_welcome_message_success() {
        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let channel = Channel::new(server_id, "General".to_string());

        let password_service = PasswordService::new();
        let user = User {
            id: user_id,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: password_service.hash("password").unwrap(),
            status: "ONLINE".to_string(),
            created_at: chrono::Utc::now(),
        };

        let mock_message_repo = MockMessageRepository::new();
        let mock_channel_repo = MockChannelRepository::new().with_channel(channel.clone());
        let mock_server_repo = MockServerRepository::new().with_member(server_id, user_id, ServerRole::Member);
        let mock_user_repo = MockUserRepository::new().with_user(user);
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(MessageHandler::new(
            jwt_service.clone(),
            mock_message_repo,
            mock_channel_repo,
            mock_server_repo,
            mock_user_repo,
        ));

        let token = jwt_service.create_token(user_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

        let result = MessageHandler::send_welcome_message(State(handler), Path(channel.id), headers).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_send_welcome_message_with_ws_manager() {
        let user_id = Uuid::new_v4();
        let server_id = Uuid::new_v4();
        let channel = Channel::new(server_id, "General".to_string());

        let password_service = PasswordService::new();
        let user = User {
            id: user_id,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: password_service.hash("password").unwrap(),
            status: "ONLINE".to_string(),
            created_at: chrono::Utc::now(),
        };

        let mock_message_repo = MockMessageRepository::new();
        let mock_channel_repo = MockChannelRepository::new().with_channel(channel.clone());
        let mock_server_repo = MockServerRepository::new().with_member(server_id, user_id, ServerRole::Member);
        let mock_user_repo = MockUserRepository::new().with_user(user);
        let jwt_service = JWTService::new("test_secret".to_string());
        let ws_manager = Arc::new(ConnectionManager::new());

        let handler = Arc::new(MessageHandler::new(
            jwt_service.clone(),
            mock_message_repo,
            mock_channel_repo,
            mock_server_repo,
            mock_user_repo,
        ).with_ws_manager(ws_manager));

        let token = jwt_service.create_token(user_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

        let result = MessageHandler::send_welcome_message(State(handler), Path(channel.id), headers).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_send_welcome_message_missing_token() {
        let channel_id = Uuid::new_v4();
        let mock_message_repo = MockMessageRepository::new();
        let mock_channel_repo = MockChannelRepository::new();
        let mock_server_repo = MockServerRepository::new();
        let mock_user_repo = MockUserRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(MessageHandler::new(
            jwt_service,
            mock_message_repo,
            mock_channel_repo,
            mock_server_repo,
            mock_user_repo,
        ));

        let headers = HeaderMap::new();
        let result = MessageHandler::send_welcome_message(State(handler), Path(channel_id), headers).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_send_message_user_not_found() {
        let user_id = Uuid::new_v4();
        let channel_id = Uuid::new_v4();
        let mock_message_repo = MockMessageRepository::new();
        let mock_channel_repo = MockChannelRepository::new();
        let mock_server_repo = MockServerRepository::new();
        let mock_user_repo = MockUserRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(MessageHandler::new(
            jwt_service.clone(),
            mock_message_repo,
            mock_channel_repo,
            mock_server_repo,
            mock_user_repo,
        ));

        let token = jwt_service.create_token(user_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());
        let payload = serde_json::json!({"content": "Hello"});

        let result = MessageHandler::send_message(State(handler), Path(channel_id), headers, Json(payload)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_send_welcome_message_user_not_found() {
        let user_id = Uuid::new_v4();
        let channel_id = Uuid::new_v4();
        let mock_message_repo = MockMessageRepository::new();
        let mock_channel_repo = MockChannelRepository::new();
        let mock_server_repo = MockServerRepository::new();
        let mock_user_repo = MockUserRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(MessageHandler::new(
            jwt_service.clone(),
            mock_message_repo,
            mock_channel_repo,
            mock_server_repo,
            mock_user_repo,
        ));

        let token = jwt_service.create_token(user_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());

        let result = MessageHandler::send_welcome_message(State(handler), Path(channel_id), headers).await;
        assert!(result.is_err());
    }
}
