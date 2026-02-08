use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Server {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub invitation_code: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Server {
    pub fn new(name: String, owner_id: Uuid, invitation_code: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            owner_id,
            invitation_code,
            created_at: chrono::Utc::now(),
        }
    }
}


// --- UNIT TESTS ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_creation() {
        let name = "Test Server".to_string();
        let owner_id = Uuid::new_v4();
        let invitation_code = "INVITE123".to_string();

        let server = Server::new(name.clone(), owner_id, invitation_code.clone());

        assert_eq!(server.name, name);
        assert_eq!(server.owner_id, owner_id);
        assert_eq!(server.invitation_code, invitation_code);
    }

    #[test]
    fn test_server_has_unique_id() {
        let owner_id = Uuid::new_v4();
        let server1 = Server::new("Server1".to_string(), owner_id, "CODE1".to_string());
        let server2 = Server::new("Server2".to_string(), owner_id, "CODE2".to_string());

        assert_ne!(server1.id, server2.id);
    }
}
