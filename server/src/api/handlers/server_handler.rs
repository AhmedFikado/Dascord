use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::application::dto::server::CreateServerRequest;
use crate::application::dto::server::JoinServerRequest;
use crate::application::use_cases::server::*;
use crate::infrastructure::security::JWTService;
use crate::infrastructure::repositories::{ServerRepository, ChannelRepository};
use crate::utils::error::AppError;

#[derive(Clone)]
pub struct ServerHandler<SR: ServerRepository, CR: ChannelRepository> {
    jwt_service: Arc<JWTService>,
    create_server_uc: Arc<CreateServerUseCase<SR>>,
    get_user_servers_uc: Arc<GetUserServersUseCase<SR>>,
    get_server_info_uc: Arc<GetServerInfoUseCase<SR>>,
    update_server_uc: Arc<UpdateServerUseCase<SR>>,
    delete_server_uc: Arc<DeleteServerUseCase<SR>>,
    join_server_uc: Arc<JoinServerUseCase<SR>>,
    leave_server_uc: Arc<LeaveServerUseCase<SR>>,
    list_members_uc: Arc<ListMembersUseCase<SR>>,
    update_member_role_uc: Arc<UpdateMemberRoleUseCase<SR>>,
    get_channels_uc: Arc<GetChannelsUseCase<SR, CR>>,
    create_channel_uc: Arc<CreateChannelUseCase<SR, CR>>,
}

impl<SR: ServerRepository, CR: ChannelRepository> ServerHandler<SR, CR> {
    pub fn new(
        jwt_service: JWTService,
        server_repo: SR,
        channel_repo: CR,
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
            list_members_uc: Arc::new(ListMembersUseCase::new(server_repo.clone())),
            update_member_role_uc: Arc::new(UpdateMemberRoleUseCase::new(server_repo.clone())),
            get_channels_uc: Arc::new(GetChannelsUseCase::new(server_repo.clone(), channel_repo.clone())),
            create_channel_uc: Arc::new(CreateChannelUseCase::new(server_repo, channel_repo)),
        }
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
            .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".to_string()))?;

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
            .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".to_string()))?;

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
            .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".to_string()))?;

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
            .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".to_string()))?;

        let claims = handler.jwt_service.verify_token(token)?;
        let user_id = Uuid::parse_str(&claims.sub_id)
            .map_err(|_| AppError::Unauthorized("Invalid user ID".to_string()))?;
        
        let name = payload.get("name")
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
            .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".to_string()))?;

        let claims = handler.jwt_service.verify_token(token)?;
        let user_id = Uuid::parse_str(&claims.sub_id)
            .map_err(|_| AppError::Unauthorized("Invalid user ID".to_string()))?;
        
        handler.delete_server_uc.execute(id, user_id).await?;
        Ok((StatusCode::OK, Json(serde_json::json!({"message": "Server deleted"}))))
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
            .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".to_string()))?;

        let claims = handler.jwt_service.verify_token(token)?;
        let user_id = Uuid::parse_str(&claims.sub_id)
            .map_err(|_| AppError::Unauthorized("Invalid user ID".to_string()))?;
        
        handler.join_server_uc.execute(&request.invitation_code, user_id).await?;
        Ok((StatusCode::OK, Json(serde_json::json!({"message": "Joined server"}))))
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
            .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".to_string()))?;

        let claims = handler.jwt_service.verify_token(token)?;
        let user_id = Uuid::parse_str(&claims.sub_id)
            .map_err(|_| AppError::Unauthorized("Invalid user ID".to_string()))?;
        
        handler.leave_server_uc.execute(id, user_id).await?;
        Ok((StatusCode::OK, Json(serde_json::json!({"message": "Left server"}))))
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
            .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".to_string()))?;

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
            .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".to_string()))?;

        let claims = handler.jwt_service.verify_token(token)?;
        let requester_id = Uuid::parse_str(&claims.sub_id)
            .map_err(|_| AppError::Unauthorized("Invalid user ID".to_string()))?;
        
        let role = serde_json::from_value(payload.get("role").cloned().unwrap_or_default())
            .map_err(|_| AppError::ValidationError("Invalid role".to_string()))?;
        
        handler.update_member_role_uc.execute(id, target_user_id, requester_id, role).await?;
        Ok((StatusCode::OK, Json(serde_json::json!({"message": "Member role updated"}))))
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
            .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".to_string()))?;

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
            .ok_or_else(|| AppError::Unauthorized("Missing or invalid Authorization header".to_string()))?;

        let claims = handler.jwt_service.verify_token(token)?;
        let user_id = Uuid::parse_str(&claims.sub_id)
            .map_err(|_| AppError::Unauthorized("Invalid user ID".to_string()))?;
        
        let channel = handler.create_channel_uc.execute(id, user_id, request).await?;
        Ok((StatusCode::CREATED, Json(channel)))
    }
}
