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
        let ws_event = crate::infrastructure::websocket::ServerMessage::NewMessage {
            channel_id: channel_id.to_string(),
            message_id: message.id.clone().unwrap_or_default(),
            user_id: message.user_id.clone(),
            username: message.username.clone(),
            content: message.content.clone(),
            created_at: chrono::Utc::now(),
        };
        // Broadcast à tous les clients qui ont rejoint ce channel (en train de le regarder)
        ws_manager.broadcast_to_channel(&channel_id.to_string(), ws_event.clone()).await;
        // Envoyer aussi directement au destinataire (même s'il ne regarde pas ce channel)
        if let Ok(Some(channel)) = controller.services.get_channel(channel_id).await {
            let recipient_id = if channel.user1 == user_id { channel.user2 } else { channel.user1 };
            ws_manager.send_to_user(recipient_id, ws_event).await;
        }
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


#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::channel::PrivateChannel;
    use crate::domain::entities::message::Message;
    use crate::domain::entities::User;
    use crate::infrastructure::repositories::mocks::mock_message_repository::MockMessageRepository;
    use crate::infrastructure::repositories::mocks::mock_private_channel_repository::MockPrivateChannelRepository;
    use crate::infrastructure::repositories::mocks::mock_user_repository::MockUserRepository;
    use crate::infrastructure::security::JWTService;
    use crate::infrastructure::websocket::ConnectionManager;
    use axum::http::{HeaderName, HeaderValue};
    use axum::routing::{delete, post};
    use axum::Router;
    use axum_test::TestServer;
    use serde_json::json;
    use uuid::Uuid;

    fn make_jwt() -> JWTService {
        JWTService::new("test_secret_key_for_tests".to_string())
    }

    fn make_user(user_id: Uuid) -> User {
        User {
            id: user_id,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            language: "fr".to_string(),
            password_hash: "hash".to_string(),
            status: "ONLINE".to_string(),
            created_at: chrono::Utc::now(),
        }
    }

    fn make_channel(channel_id: Uuid, user_id: Uuid) -> PrivateChannel {
        PrivateChannel {
            id: channel_id,
            user1: user_id,
            user2: Uuid::new_v4(),
            created_at: chrono::Utc::now(),
        }
    }

    fn make_message(channel_id: Uuid, user_id: Uuid) -> Message {
        Message::new(
            channel_id.to_string(),
            user_id.to_string(),
            "testuser".to_string(),
            "Hello".to_string(),
        )
    }

    fn bearer(jwt: &JWTService, user_id: Uuid) -> HeaderValue {
        let token = jwt.create_token(user_id).unwrap();
        HeaderValue::from_str(&format!("Bearer {}", token)).unwrap()
    }

    type MockController =
        PrivateMessageController<MockMessageRepository, MockPrivateChannelRepository, MockUserRepository>;

    fn build_controller(
        jwt: JWTService,
        msg_repo: MockMessageRepository,
        channel_repo: MockPrivateChannelRepository,
        user_repo: MockUserRepository,
    ) -> Arc<MockController> {
        Arc::new(PrivateMessageController::new(jwt, msg_repo, channel_repo, user_repo))
    }

    fn build_app(controller: Arc<MockController>) -> TestServer {
        let app = Router::new()
            .route(
                "/channels/:channel_id/messages/private",
                post(send_private_message::<MockMessageRepository, MockPrivateChannelRepository, MockUserRepository>)
                    .get(get_private_message_history::<MockMessageRepository, MockPrivateChannelRepository, MockUserRepository>),
            )
            .route(
                "/messages/private/:id",
                delete(delete_private_message::<MockMessageRepository, MockPrivateChannelRepository, MockUserRepository>)
                    .put(update_private_message::<MockMessageRepository, MockPrivateChannelRepository, MockUserRepository>),
            )
            .route(
                "/messages/private/:id/reactions",
                post(add_private_reaction::<MockMessageRepository, MockPrivateChannelRepository, MockUserRepository>),
            )
            .route(
                "/messages/private/:id/reactions/:reaction",
                delete(remove_private_reaction::<MockMessageRepository, MockPrivateChannelRepository, MockUserRepository>),
            )
            .with_state(controller);
        TestServer::new(app).unwrap()
    }

    // --- Controller struct tests ---

    #[test]
    fn test_new_controller_has_no_ws_manager() {
        let controller = PrivateMessageController::new(
            make_jwt(),
            MockMessageRepository::new(),
            MockPrivateChannelRepository::new(),
            MockUserRepository::new(),
        );
        assert!(controller.ws_manager.is_none());
    }

    #[test]
    fn test_with_ws_manager_sets_manager() {
        let controller = PrivateMessageController::new(
            make_jwt(),
            MockMessageRepository::new(),
            MockPrivateChannelRepository::new(),
            MockUserRepository::new(),
        );
        let ws_manager = Arc::new(ConnectionManager::new());
        let controller = controller.with_ws_manager(ws_manager);
        assert!(controller.ws_manager.is_some());
    }

    // --- send_private_message tests ---

    #[tokio::test]
    async fn test_send_private_message_success() {
        let jwt = make_jwt();
        let user_id = Uuid::new_v4();
        let channel_id = Uuid::new_v4();
        let controller = build_controller(
            make_jwt(),
            MockMessageRepository::new(),
            MockPrivateChannelRepository::with_channels(vec![make_channel(channel_id, user_id)]),
            MockUserRepository::new().with_user(make_user(user_id)),
        );
        let server = build_app(controller);

        let resp = server
            .post(&format!("/channels/{}/messages/private", channel_id))
            .add_header(HeaderName::from_static("authorization"), bearer(&jwt, user_id))
            .json(&json!({ "content": "Hello" }))
            .await;

        assert_eq!(resp.status_code(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_send_private_message_missing_token() {
        let user_id = Uuid::new_v4();
        let channel_id = Uuid::new_v4();
        let controller = build_controller(
            make_jwt(),
            MockMessageRepository::new(),
            MockPrivateChannelRepository::with_channels(vec![make_channel(channel_id, user_id)]),
            MockUserRepository::new().with_user(make_user(user_id)),
        );
        let server = build_app(controller);

        let resp = server
            .post(&format!("/channels/{}/messages/private", channel_id))
            .json(&json!({ "content": "Hello" }))
            .await;

        assert_eq!(resp.status_code(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_send_private_message_missing_content() {
        let jwt = make_jwt();
        let user_id = Uuid::new_v4();
        let channel_id = Uuid::new_v4();
        let controller = build_controller(
            make_jwt(),
            MockMessageRepository::new(),
            MockPrivateChannelRepository::with_channels(vec![make_channel(channel_id, user_id)]),
            MockUserRepository::new().with_user(make_user(user_id)),
        );
        let server = build_app(controller);

        let resp = server
            .post(&format!("/channels/{}/messages/private", channel_id))
            .add_header(HeaderName::from_static("authorization"), bearer(&jwt, user_id))
            .json(&json!({}))
            .await;

        assert_eq!(resp.status_code(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_send_private_message_user_not_found() {
        let jwt = make_jwt();
        let user_id = Uuid::new_v4();
        let channel_id = Uuid::new_v4();
        let controller = build_controller(
            make_jwt(),
            MockMessageRepository::new(),
            MockPrivateChannelRepository::with_channels(vec![make_channel(channel_id, user_id)]),
            MockUserRepository::new(),
        );
        let server = build_app(controller);

        let resp = server
            .post(&format!("/channels/{}/messages/private", channel_id))
            .add_header(HeaderName::from_static("authorization"), bearer(&jwt, user_id))
            .json(&json!({ "content": "Hello" }))
            .await;

        assert_eq!(resp.status_code(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_send_private_message_channel_not_found() {
        let jwt = make_jwt();
        let user_id = Uuid::new_v4();
        let channel_id = Uuid::new_v4();
        let other_channel_id = Uuid::new_v4();
        let controller = build_controller(
            make_jwt(),
            MockMessageRepository::new(),
            MockPrivateChannelRepository::with_channels(vec![make_channel(other_channel_id, user_id)]),
            MockUserRepository::new().with_user(make_user(user_id)),
        );
        let server = build_app(controller);

        let resp = server
            .post(&format!("/channels/{}/messages/private", channel_id))
            .add_header(HeaderName::from_static("authorization"), bearer(&jwt, user_id))
            .json(&json!({ "content": "Hello" }))
            .await;

        assert_eq!(resp.status_code(), StatusCode::NOT_FOUND);
    }

    // --- get_private_message_history tests ---

    #[tokio::test]
    async fn test_get_private_message_history_success() {
        let jwt = make_jwt();
        let user_id = Uuid::new_v4();
        let channel_id = Uuid::new_v4();
        let controller = build_controller(
            make_jwt(),
            MockMessageRepository::new().with_message(make_message(channel_id, user_id)),
            MockPrivateChannelRepository::with_channels(vec![make_channel(channel_id, user_id)]),
            MockUserRepository::new().with_user(make_user(user_id)),
        );
        let server = build_app(controller);

        let resp = server
            .get(&format!("/channels/{}/messages/private", channel_id))
            .add_header(HeaderName::from_static("authorization"), bearer(&jwt, user_id))
            .await;

        assert_eq!(resp.status_code(), StatusCode::OK);
        let messages: Vec<serde_json::Value> = resp.json();
        assert_eq!(messages.len(), 1);
    }

    #[tokio::test]
    async fn test_get_private_message_history_missing_token() {
        let user_id = Uuid::new_v4();
        let channel_id = Uuid::new_v4();
        let controller = build_controller(
            make_jwt(),
            MockMessageRepository::new(),
            MockPrivateChannelRepository::with_channels(vec![make_channel(channel_id, user_id)]),
            MockUserRepository::new().with_user(make_user(user_id)),
        );
        let server = build_app(controller);

        let resp = server
            .get(&format!("/channels/{}/messages/private", channel_id))
            .await;

        assert_eq!(resp.status_code(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_get_private_message_history_not_participant() {
        let jwt = make_jwt();
        let user_id = Uuid::new_v4();
        let channel_id = Uuid::new_v4();
        let channel = PrivateChannel {
            id: channel_id,
            user1: Uuid::new_v4(),
            user2: Uuid::new_v4(),
            created_at: chrono::Utc::now(),
        };
        let controller = build_controller(
            make_jwt(),
            MockMessageRepository::new(),
            MockPrivateChannelRepository::with_channels(vec![channel]),
            MockUserRepository::new().with_user(make_user(user_id)),
        );
        let server = build_app(controller);

        let resp = server
            .get(&format!("/channels/{}/messages/private", channel_id))
            .add_header(HeaderName::from_static("authorization"), bearer(&jwt, user_id))
            .await;

        assert_eq!(resp.status_code(), StatusCode::UNAUTHORIZED);
    }

    // --- delete_private_message tests ---

    #[tokio::test]
    async fn test_delete_private_message_success() {
        let jwt = make_jwt();
        let user_id = Uuid::new_v4();
        let channel_id = Uuid::new_v4();
        let controller = build_controller(
            make_jwt(),
            MockMessageRepository::new().with_message(make_message(channel_id, user_id)),
            MockPrivateChannelRepository::with_channels(vec![make_channel(channel_id, user_id)]),
            MockUserRepository::new().with_user(make_user(user_id)),
        );
        let server = build_app(controller);

        let resp = server
            .delete("/messages/private/msg_1")
            .add_header(HeaderName::from_static("authorization"), bearer(&jwt, user_id))
            .await;

        assert_eq!(resp.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_delete_private_message_missing_token() {
        let user_id = Uuid::new_v4();
        let channel_id = Uuid::new_v4();
        let controller = build_controller(
            make_jwt(),
            MockMessageRepository::new().with_message(make_message(channel_id, user_id)),
            MockPrivateChannelRepository::with_channels(vec![make_channel(channel_id, user_id)]),
            MockUserRepository::new().with_user(make_user(user_id)),
        );
        let server = build_app(controller);

        let resp = server.delete("/messages/private/msg_1").await;

        assert_eq!(resp.status_code(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_delete_private_message_not_found() {
        let jwt = make_jwt();
        let user_id = Uuid::new_v4();
        let channel_id = Uuid::new_v4();
        let controller = build_controller(
            make_jwt(),
            MockMessageRepository::new(),
            MockPrivateChannelRepository::with_channels(vec![make_channel(channel_id, user_id)]),
            MockUserRepository::new().with_user(make_user(user_id)),
        );
        let server = build_app(controller);

        let resp = server
            .delete("/messages/private/nonexistent_msg")
            .add_header(HeaderName::from_static("authorization"), bearer(&jwt, user_id))
            .await;

        assert_eq!(resp.status_code(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_delete_private_message_not_owner() {
        let jwt = make_jwt();
        let owner_id = Uuid::new_v4();
        let other_user_id = Uuid::new_v4();
        let channel_id = Uuid::new_v4();
        let channel = PrivateChannel {
            id: channel_id,
            user1: other_user_id,
            user2: owner_id,
            created_at: chrono::Utc::now(),
        };
        let controller = build_controller(
            make_jwt(),
            MockMessageRepository::new().with_message(make_message(channel_id, owner_id)),
            MockPrivateChannelRepository::with_channels(vec![channel]),
            MockUserRepository::new().with_user(make_user(other_user_id)),
        );
        let server = build_app(controller);

        let resp = server
            .delete("/messages/private/msg_1")
            .add_header(HeaderName::from_static("authorization"), bearer(&jwt, other_user_id))
            .await;

        assert_eq!(resp.status_code(), StatusCode::FORBIDDEN);
    }

    // --- update_private_message tests ---

    #[tokio::test]
    async fn test_update_private_message_success() {
        let jwt = make_jwt();
        let user_id = Uuid::new_v4();
        let channel_id = Uuid::new_v4();
        let controller = build_controller(
            make_jwt(),
            MockMessageRepository::new().with_message(make_message(channel_id, user_id)),
            MockPrivateChannelRepository::with_channels(vec![make_channel(channel_id, user_id)]),
            MockUserRepository::new().with_user(make_user(user_id)),
        );
        let server = build_app(controller);

        let resp = server
            .put("/messages/private/msg_1")
            .add_header(HeaderName::from_static("authorization"), bearer(&jwt, user_id))
            .json(&json!({ "content": "Updated content" }))
            .await;

        assert_eq!(resp.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_update_private_message_missing_token() {
        let user_id = Uuid::new_v4();
        let channel_id = Uuid::new_v4();
        let controller = build_controller(
            make_jwt(),
            MockMessageRepository::new().with_message(make_message(channel_id, user_id)),
            MockPrivateChannelRepository::with_channels(vec![make_channel(channel_id, user_id)]),
            MockUserRepository::new().with_user(make_user(user_id)),
        );
        let server = build_app(controller);

        let resp = server
            .put("/messages/private/msg_1")
            .json(&json!({ "content": "Updated" }))
            .await;

        assert_eq!(resp.status_code(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_update_private_message_missing_content() {
        let jwt = make_jwt();
        let user_id = Uuid::new_v4();
        let channel_id = Uuid::new_v4();
        let controller = build_controller(
            make_jwt(),
            MockMessageRepository::new().with_message(make_message(channel_id, user_id)),
            MockPrivateChannelRepository::with_channels(vec![make_channel(channel_id, user_id)]),
            MockUserRepository::new().with_user(make_user(user_id)),
        );
        let server = build_app(controller);

        let resp = server
            .put("/messages/private/msg_1")
            .add_header(HeaderName::from_static("authorization"), bearer(&jwt, user_id))
            .json(&json!({}))
            .await;

        assert_eq!(resp.status_code(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_update_private_message_not_owner() {
        let jwt = make_jwt();
        let owner_id = Uuid::new_v4();
        let other_user_id = Uuid::new_v4();
        let channel_id = Uuid::new_v4();
        let channel = PrivateChannel {
            id: channel_id,
            user1: other_user_id,
            user2: owner_id,
            created_at: chrono::Utc::now(),
        };
        let controller = build_controller(
            make_jwt(),
            MockMessageRepository::new().with_message(make_message(channel_id, owner_id)),
            MockPrivateChannelRepository::with_channels(vec![channel]),
            MockUserRepository::new().with_user(make_user(other_user_id)),
        );
        let server = build_app(controller);

        let resp = server
            .put("/messages/private/msg_1")
            .add_header(HeaderName::from_static("authorization"), bearer(&jwt, other_user_id))
            .json(&json!({ "content": "Hacked" }))
            .await;

        assert_eq!(resp.status_code(), StatusCode::FORBIDDEN);
    }

    // --- add_private_reaction tests ---

    #[tokio::test]
    async fn test_add_private_reaction_success() {
        let jwt = make_jwt();
        let user_id = Uuid::new_v4();
        let channel_id = Uuid::new_v4();
        let controller = build_controller(
            make_jwt(),
            MockMessageRepository::new().with_message(make_message(channel_id, user_id)),
            MockPrivateChannelRepository::with_channels(vec![make_channel(channel_id, user_id)]),
            MockUserRepository::new().with_user(make_user(user_id)),
        );
        let server = build_app(controller);

        let resp = server
            .post("/messages/private/msg_1/reactions")
            .add_header(HeaderName::from_static("authorization"), bearer(&jwt, user_id))
            .json(&json!({ "reaction": "+1" }))
            .await;

        assert_eq!(resp.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_add_private_reaction_missing_token() {
        let user_id = Uuid::new_v4();
        let channel_id = Uuid::new_v4();
        let controller = build_controller(
            make_jwt(),
            MockMessageRepository::new().with_message(make_message(channel_id, user_id)),
            MockPrivateChannelRepository::with_channels(vec![make_channel(channel_id, user_id)]),
            MockUserRepository::new().with_user(make_user(user_id)),
        );
        let server = build_app(controller);

        let resp = server
            .post("/messages/private/msg_1/reactions")
            .json(&json!({ "reaction": "+1" }))
            .await;

        assert_eq!(resp.status_code(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_add_private_reaction_missing_reaction_field() {
        let jwt = make_jwt();
        let user_id = Uuid::new_v4();
        let channel_id = Uuid::new_v4();
        let controller = build_controller(
            make_jwt(),
            MockMessageRepository::new().with_message(make_message(channel_id, user_id)),
            MockPrivateChannelRepository::with_channels(vec![make_channel(channel_id, user_id)]),
            MockUserRepository::new().with_user(make_user(user_id)),
        );
        let server = build_app(controller);

        let resp = server
            .post("/messages/private/msg_1/reactions")
            .add_header(HeaderName::from_static("authorization"), bearer(&jwt, user_id))
            .json(&json!({}))
            .await;

        assert_eq!(resp.status_code(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_add_private_reaction_message_not_found() {
        let jwt = make_jwt();
        let user_id = Uuid::new_v4();
        let channel_id = Uuid::new_v4();
        let controller = build_controller(
            make_jwt(),
            MockMessageRepository::new(),
            MockPrivateChannelRepository::with_channels(vec![make_channel(channel_id, user_id)]),
            MockUserRepository::new().with_user(make_user(user_id)),
        );
        let server = build_app(controller);

        let resp = server
            .post("/messages/private/nonexistent/reactions")
            .add_header(HeaderName::from_static("authorization"), bearer(&jwt, user_id))
            .json(&json!({ "reaction": "+1" }))
            .await;

        assert_eq!(resp.status_code(), StatusCode::NOT_FOUND);
    }

    // --- remove_private_reaction tests ---

    #[tokio::test]
    async fn test_remove_private_reaction_success() {
        let jwt = make_jwt();
        let user_id = Uuid::new_v4();
        let channel_id = Uuid::new_v4();
        let mut message = make_message(channel_id, user_id);
        message.reactions.insert("+1".to_string(), vec![user_id.to_string()]);
        let controller = build_controller(
            make_jwt(),
            MockMessageRepository::new().with_message(message),
            MockPrivateChannelRepository::with_channels(vec![make_channel(channel_id, user_id)]),
            MockUserRepository::new().with_user(make_user(user_id)),
        );
        let server = build_app(controller);

        let resp = server
            .delete("/messages/private/msg_1/reactions/%2B1")
            .add_header(HeaderName::from_static("authorization"), bearer(&jwt, user_id))
            .await;

        assert_eq!(resp.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_remove_private_reaction_missing_token() {
        let user_id = Uuid::new_v4();
        let channel_id = Uuid::new_v4();
        let controller = build_controller(
            make_jwt(),
            MockMessageRepository::new().with_message(make_message(channel_id, user_id)),
            MockPrivateChannelRepository::with_channels(vec![make_channel(channel_id, user_id)]),
            MockUserRepository::new().with_user(make_user(user_id)),
        );
        let server = build_app(controller);

        let resp = server
            .delete("/messages/private/msg_1/reactions/%2B1")
            .await;

        assert_eq!(resp.status_code(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_remove_private_reaction_message_not_found() {
        let jwt = make_jwt();
        let user_id = Uuid::new_v4();
        let channel_id = Uuid::new_v4();
        let controller = build_controller(
            make_jwt(),
            MockMessageRepository::new(),
            MockPrivateChannelRepository::with_channels(vec![make_channel(channel_id, user_id)]),
            MockUserRepository::new().with_user(make_user(user_id)),
        );
        let server = build_app(controller);

        let resp = server
            .delete("/messages/private/nonexistent/reactions/%2B1")
            .add_header(HeaderName::from_static("authorization"), bearer(&jwt, user_id))
            .await;

        assert_eq!(resp.status_code(), StatusCode::NOT_FOUND);
    }
}
