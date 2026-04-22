use crate::domain::entities::Server;
use crate::domain::value_objects::ServerRole;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateServerRequest {
    #[validate(length(
        min = 1,
        max = 100,
        message = "Server name must be between 1 and 100 characters"
    ))]
    pub name: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct JoinServerRequest {
    #[validate(length(min = 1, message = "Invitation code is required"))]
    pub invitation_code: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ServerResponse {
    pub id: String,
    pub name: String,
    pub owner_id: String,
    pub invitation_code: String,
    pub created_at: String,
}

impl From<Server> for ServerResponse {
    fn from(server: Server) -> Self {
        Self {
            id: server.id.to_string(),
            name: server.name,
            owner_id: server.owner_id.to_string(),
            invitation_code: server.invitation_code,
            created_at: server.created_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct MemberResponse {
    pub server_id: String,
    pub user_id: String,
    pub role: ServerRole,
    pub joined_at: String,
    pub user: UserInfo,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub email: String,
    pub status: String,
    pub avatar_id: Option<String>,
    pub created_at: String,
}

// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_valid_create_server_request() {
        let request = CreateServerRequest {
            name: "My Server".to_string(),
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_empty_server_name() {
        let request = CreateServerRequest {
            name: "".to_string(),
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn test_valid_join_server_request() {
        let request = JoinServerRequest {
            invitation_code: "ABC123".to_string(),
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_server_response_from_entity() {
        let server = Server {
            id: uuid::Uuid::new_v4(),
            name: "Test Server".to_string(),
            owner_id: uuid::Uuid::new_v4(),
            invitation_code: "INVITE".to_string(),
            created_at: chrono::Utc::now(),
        };
        let response = ServerResponse::from(server);
        assert_eq!(response.name, "Test Server");
    }
}
