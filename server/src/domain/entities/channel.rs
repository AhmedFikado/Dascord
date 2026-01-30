use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub id: Uuid,
    pub server_id: Uuid,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Channel {
    pub fn new(server_id: Uuid, name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            server_id,
            name,
            created_at: chrono::Utc::now(),
        }
    }
}
