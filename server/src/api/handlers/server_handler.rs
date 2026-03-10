use crate::application::dto::server::CreateServerRequest;
use crate::application::dto::server::JoinServerRequest;
use crate::application::use_cases::server::*;
use crate::infrastructure::repositories::{ChannelRepository, ServerRepository, UserRepository};
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
pub struct ServerHandler<SR: ServerRepository, CR: ChannelRepository, UR: UserRepository> {
    jwt_service: Arc<JWTService>,
    create_server_uc: Arc<CreateServerUseCase<SR>>,
    get_user_servers_uc: Arc<GetUserServersUseCase<SR>>,
    get_server_info_uc: Arc<GetServerInfoUseCase<SR>>,
    update_server_uc: Arc<UpdateServerUseCase<SR>>,
    delete_server_uc: Arc<DeleteServerUseCase<SR>>,
    join_server_uc: Arc<JoinServerUseCase<SR>>,
    leave_server_uc: Arc<LeaveServerUseCase<SR>>,
    list_members_uc: Arc<ListMembersUseCase<SR, UR>>,
    update_member_role_uc: Arc<UpdateMemberRoleUseCase<SR>>,
    get_channels_uc: Arc<GetChannelsUseCase<SR, CR>>,
    create_channel_uc: Arc<CreateChannelUseCase<SR, CR>>,
    user_repo: Arc<UR>,
    ws_manager: Option<Arc<ConnectionManager>>,
}

impl<SR: ServerRepository, CR: ChannelRepository, UR: UserRepository> ServerHandler<SR, CR, UR> {
    pub fn new(
        jwt_service: JWTService,
        server_repo: SR,
        channel_repo: CR,
        user_repo: UR,
    ) -> Self {
        Self {
            jwt_service: Arc::new(jwt_service),
            create_server_uc: Arc::new(CreateServerUseCase::new(server_repo.clone())),
            get_user_servers_uc: Arc::new(GetUserServersUseCase::new(server_repo.clone())),
            get_server_info_uc: Arc::new(GetServerInfoUseCase::new(server_repo.clone())),
            update_server_uc: Arc::new(UpdateServerUseCase::new(server_repo.clone())),
            delete_server_uc: Arc::new(DeleteServerUseCase::new(server_repo.clone())),
            join_server_uc: Arc::new(JoinServerUseCase::new(server_repo.clone())),
            leave_server_uc: Arc::new(LeaveServerUseCase::new(server_repo.clone())),
            list_members_uc: Arc::new(ListMembersUseCase::new(server_repo.clone(), user_repo.clone())),
            update_member_role_uc: Arc::new(UpdateMemberRoleUseCase::new(server_repo.clone())),
            get_channels_uc: Arc::new(GetChannelsUseCase::new(
                server_repo.clone(),
                channel_repo.clone(),
            )),
            create_channel_uc: Arc::new(CreateChannelUseCase::new(server_repo, channel_repo)),
            user_repo: Arc::new(user_repo),
            ws_manager: None,
        }
    }

    pub fn with_ws_manager(mut self, ws_manager: Arc<ConnectionManager>) -> Self {
        self.ws_manager = Some(ws_manager);
        self
    }

    pub async fn create_server(
        State(handler): State<Arc<Self>>,
        headers: HeaderMap,
        Json(request): Json<CreateServerRequest>,
    ) -> Result<impl IntoResponse, AppError> {
        let token = headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or_else(|| {
                AppError::Unauthorized("Missing or invalid Authorization header".to_string())
            })?;

        let claims = handler.jwt_service.verify_token(token)?;
        let owner_id = Uuid::parse_str(&claims.sub_id)
            .map_err(|_| AppError::Unauthorized("Invalid user ID".to_string()))?;

        let response = handler.create_server_uc.execute(request, owner_id).await?;
        Ok((StatusCode::CREATED, Json(response)))
    }

    pub async fn get_user_servers(
        State(handler): State<Arc<Self>>,
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

        let servers = handler.get_user_servers_uc.execute(user_id).await?;
        Ok((StatusCode::OK, Json(servers)))
    }

    pub async fn get_server_info(
        State(handler): State<Arc<Self>>,
        Path(id): Path<Uuid>,
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

        let server = handler.get_server_info_uc.execute(id, user_id).await?;
        Ok((StatusCode::OK, Json(server)))
    }

    pub async fn update_server(
        State(handler): State<Arc<Self>>,
        Path(id): Path<Uuid>,
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

        let name = payload
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AppError::ValidationError("Missing name field".to_string()))?
            .to_string();

        let server = handler.update_server_uc.execute(id, user_id, name).await?;
        Ok((StatusCode::OK, Json(server)))
    }

    pub async fn delete_server(
        State(handler): State<Arc<Self>>,
        Path(id): Path<Uuid>,
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

        handler.delete_server_uc.execute(id, user_id).await?;
        Ok((
            StatusCode::OK,
            Json(serde_json::json!({"message": "Server deleted"})),
        ))
    }

    pub async fn join_server(
        State(handler): State<Arc<Self>>,
        headers: HeaderMap,
        Json(request): Json<JoinServerRequest>,
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

        let server_id = handler
            .join_server_uc
            .execute(&request.invitation_code, user_id)
            .await?;
        
        // Récupérer les infos de l'utilisateur pour la diffusion
        let user = handler
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;
        
        // Diffuser l'événement WebSocket à tous les clients connectés
        if let Some(ws_manager) = &handler.ws_manager {
            ws_manager.broadcast_to_all(
                crate::infrastructure::websocket::ServerMessage::ServerMemberJoined {
                    server_id: server_id.to_string(),
                    user_id: user_id.to_string(),
                    username: user.username,
                },
            ).await;
        }
        
        Ok((
            StatusCode::OK,
            Json(serde_json::json!({"message": "Joined server"})),
        ))
    }

    pub async fn leave_server(
        State(handler): State<Arc<Self>>,
        Path(id): Path<Uuid>,
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

        handler.leave_server_uc.execute(id, user_id).await?;
        
        // Diffuser l'événement WebSocket à tous les clients connectés
        if let Some(ws_manager) = &handler.ws_manager {
            ws_manager.broadcast_to_all(
                crate::infrastructure::websocket::ServerMessage::ServerMemberLeft {
                    server_id: id.to_string(),
                    user_id: user_id.to_string(),
                },
            ).await;
        }
        
        Ok((
            StatusCode::OK,
            Json(serde_json::json!({"message": "Left server"})),
        ))
    }

    pub async fn list_members(
        State(handler): State<Arc<Self>>,
        Path(id): Path<Uuid>,
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

        let members = handler.list_members_uc.execute(id, user_id).await?;
        Ok((StatusCode::OK, Json(members)))
    }

    pub async fn update_member_role(
        State(handler): State<Arc<Self>>,
        Path((id, target_user_id)): Path<(Uuid, Uuid)>,
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
        let requester_id = Uuid::parse_str(&claims.sub_id)
            .map_err(|_| AppError::Unauthorized("Invalid user ID".to_string()))?;

        let role = serde_json::from_value(payload.get("role").cloned().unwrap_or_default())
            .map_err(|_| AppError::ValidationError("Invalid role".to_string()))?;

        handler
            .update_member_role_uc
            .execute(id, target_user_id, requester_id, role)
            .await?;
        Ok((
            StatusCode::OK,
            Json(serde_json::json!({"message": "Member role updated"})),
        ))
    }

    pub async fn get_channels(
        State(handler): State<Arc<Self>>,
        Path(id): Path<Uuid>,
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

        let channels = handler.get_channels_uc.execute(id, user_id).await?;
        Ok((StatusCode::OK, Json(channels)))
    }

    pub async fn create_channel(
        State(handler): State<Arc<Self>>,
        Path(id): Path<Uuid>,
        headers: HeaderMap,
        Json(request): Json<crate::application::dto::channel::CreateChannelRequest>,
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

        let channel = handler
            .create_channel_uc
            .execute(id, user_id, request)
            .await?;
        Ok((StatusCode::CREATED, Json(channel)))
    }
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::dto::channel::CreateChannelRequest;
    use crate::application::dto::server::{CreateServerRequest, JoinServerRequest};
    use crate::domain::entities::Server;
    use crate::domain::value_objects::ServerRole;
    use crate::infrastructure::repositories::mocks::{
        mock_channel_repository::MockChannelRepository,
        mock_server_repository::MockServerRepository,
        mock_user_repository::MockUserRepository,
    };

    #[tokio::test]
    async fn test_create_server_success() {
        let owner_id = Uuid::new_v4();

        let mock_server_repo = MockServerRepository::new();
        let mock_channel_repo = MockChannelRepository::new();
        let mock_user_repo = MockUserRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(ServerHandler::new(
            jwt_service.clone(),
            mock_server_repo,
            mock_channel_repo,
            mock_user_repo,
        ));

        let token = jwt_service.create_token(owner_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let request = CreateServerRequest {
            name: "Test Server".to_string(),
        };

        let result = ServerHandler::create_server(State(handler), headers, Json(request)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_user_servers_success() {
        let user_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), user_id, "CODE123".to_string());

        let mock_server_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, user_id, ServerRole::Owner);
        let mock_channel_repo = MockChannelRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(ServerHandler::new(
            jwt_service.clone(),
            mock_server_repo,
            mock_channel_repo,
        MockUserRepository::new(),));

        let token = jwt_service.create_token(user_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let result = ServerHandler::get_user_servers(State(handler), headers).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_join_server_success() {
        use crate::domain::entities::User;
        use crate::infrastructure::security::PasswordService;

        let owner_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), owner_id, "CODE123".to_string());

        let password_service = PasswordService::new();
        let user = User {
            id: user_id,
            username: "testuser".to_string(),
            email: "test@test.com".to_string(),
            password_hash: password_service.hash("password").unwrap(),
            status: "ONLINE".to_string(),
            created_at: chrono::Utc::now(),
        };

        let mock_server_repo = MockServerRepository::new().with_server(server.clone());
        let mock_channel_repo = MockChannelRepository::new();
        let mock_user_repo = MockUserRepository::new().with_user(user);
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(ServerHandler::new(
            jwt_service.clone(),
            mock_server_repo,
            mock_channel_repo,
            mock_user_repo,
        ));

        let token = jwt_service.create_token(user_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let request = JoinServerRequest {
            invitation_code: "CODE123".to_string(),
        };

        let result = ServerHandler::join_server(State(handler), headers, Json(request)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_channel_success() {
        let owner_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), owner_id, "CODE123".to_string());

        let mock_server_repo = MockServerRepository::new().with_server(server.clone());
        let mock_channel_repo = MockChannelRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(ServerHandler::new(
            jwt_service.clone(),
            mock_server_repo,
            mock_channel_repo,
        MockUserRepository::new(),));

        let token = jwt_service.create_token(owner_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let request = CreateChannelRequest {
            name: "General".to_string(),
        };

        let result =
            ServerHandler::create_channel(State(handler), Path(server.id), headers, Json(request))
                .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_missing_authorization_header() {
        let mock_server_repo = MockServerRepository::new();
        let mock_channel_repo = MockChannelRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(ServerHandler::new(
            jwt_service,
            mock_server_repo,
            mock_channel_repo,
        MockUserRepository::new(),));
        let headers = HeaderMap::new();

        let result = ServerHandler::get_user_servers(State(handler), headers).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_server_info_success() {
        let owner_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), owner_id, "CODE123".to_string());

        let mock_server_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, owner_id, ServerRole::Owner);
        let mock_channel_repo = MockChannelRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(ServerHandler::new(
            jwt_service.clone(),
            mock_server_repo,
            mock_channel_repo,
        MockUserRepository::new(),));
        let token = jwt_service.create_token(owner_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let result = ServerHandler::get_server_info(State(handler), Path(server.id), headers).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_server_success() {
        let owner_id = Uuid::new_v4();
        let server = Server::new("Old Name".to_string(), owner_id, "CODE123".to_string());

        let mock_server_repo = MockServerRepository::new().with_server(server.clone());
        let mock_channel_repo = MockChannelRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(ServerHandler::new(
            jwt_service.clone(),
            mock_server_repo,
            mock_channel_repo,
        MockUserRepository::new(),));
        let token = jwt_service.create_token(owner_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let payload = serde_json::json!({"name": "New Name"});
        let result =
            ServerHandler::update_server(State(handler), Path(server.id), headers, Json(payload)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_server_success() {
        let owner_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), owner_id, "CODE123".to_string());

        let mock_server_repo = MockServerRepository::new().with_server(server.clone());
        let mock_channel_repo = MockChannelRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(ServerHandler::new(
            jwt_service.clone(),
            mock_server_repo,
            mock_channel_repo,
        MockUserRepository::new(),));
        let token = jwt_service.create_token(owner_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let result = ServerHandler::delete_server(State(handler), Path(server.id), headers).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_leave_server_success() {
        let owner_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), owner_id, "CODE123".to_string());

        let mock_server_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, user_id, ServerRole::Member);
        let mock_channel_repo = MockChannelRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(ServerHandler::new(
            jwt_service.clone(),
            mock_server_repo,
            mock_channel_repo,
        MockUserRepository::new(),));
        let token = jwt_service.create_token(user_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let result = ServerHandler::leave_server(State(handler), Path(server.id), headers).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_list_members_success() {
        use crate::domain::entities::User;
        use crate::infrastructure::security::PasswordService;

        let owner_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), owner_id, "CODE123".to_string());

        let password_service = PasswordService::new();
        let owner = User {
            id: owner_id,
            username: "owner".to_string(),
            email: "owner@test.com".to_string(),
            password_hash: password_service.hash("password").unwrap(),
            status: "ONLINE".to_string(),
            created_at: chrono::Utc::now(),
        };

        let mock_server_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, owner_id, ServerRole::Owner);
        let mock_channel_repo = MockChannelRepository::new();
        let mock_user_repo = MockUserRepository::new().with_user(owner);
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(ServerHandler::new(
            jwt_service.clone(),
            mock_server_repo,
            mock_channel_repo,
            mock_user_repo,
        ));
        let token = jwt_service.create_token(owner_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let result = ServerHandler::list_members(State(handler), Path(server.id), headers).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_member_role_success() {
        let owner_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), owner_id, "CODE123".to_string());

        let mock_server_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, user_id, ServerRole::Member);
        let mock_channel_repo = MockChannelRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(ServerHandler::new(
            jwt_service.clone(),
            mock_server_repo,
            mock_channel_repo,
        MockUserRepository::new(),));
        let token = jwt_service.create_token(owner_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let payload = serde_json::json!({"role": "ADMIN"});
        let result = ServerHandler::update_member_role(
            State(handler),
            Path((server.id, user_id)),
            headers,
            Json(payload),
        )
        .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_channels_success() {
        let owner_id = Uuid::new_v4();
        let server = Server::new("Test Server".to_string(), owner_id, "CODE123".to_string());

        let mock_server_repo = MockServerRepository::new()
            .with_server(server.clone())
            .with_member(server.id, owner_id, ServerRole::Owner);
        let mock_channel_repo = MockChannelRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(ServerHandler::new(
            jwt_service.clone(),
            mock_server_repo,
            mock_channel_repo,
        MockUserRepository::new(),));
        let token = jwt_service.create_token(owner_id).unwrap();
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let result = ServerHandler::get_channels(State(handler), Path(server.id), headers).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_invalid_token() {
        let mock_server_repo = MockServerRepository::new();
        let mock_channel_repo = MockChannelRepository::new();
        let jwt_service = JWTService::new("test_secret".to_string());

        let handler = Arc::new(ServerHandler::new(
            jwt_service,
            mock_server_repo,
            mock_channel_repo,
        MockUserRepository::new(),));
        let mut headers = HeaderMap::new();
        headers.insert("Authorization", "Bearer invalid_token".parse().unwrap());

        let result = ServerHandler::get_user_servers(State(handler), headers).await;
        assert!(result.is_err());
    }
}

