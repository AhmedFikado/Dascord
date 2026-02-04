use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum ServerRole {
    Owner,
    Admin,
    Member,
}

impl ServerRole {
    pub fn can_have_permissions(&self) -> bool {
        matches!(self, ServerRole::Owner | ServerRole::Admin)
    }

    pub fn can_delete_server(&self) -> bool {
        matches!(self, ServerRole::Owner)
    }
}
