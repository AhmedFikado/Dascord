use crate::domain::entities::Server;
use crate::domain::value_objects::ServerRole;
use crate::infrastructure::repositories::ServerRepository;
use crate::utils::error::{AppResult};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Clone)]
pub struct MockServerRepository {
    servers: Arc<Mutex<HashMap<Uuid, Server>>>,
    members: Arc<Mutex<HashMap<(Uuid, Uuid), ServerRole>>>, // (server_id, user_id) -> role
    invitation_codes: Arc<Mutex<HashMap<String, Uuid>>>,    // code -> server_id
}

impl MockServerRepository {
    pub fn new() -> Self {
        Self {
            servers: Arc::new(Mutex::new(HashMap::new())),
            members: Arc::new(Mutex::new(HashMap::new())),
            invitation_codes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn with_server(self, server: Server) -> Self {
        {
            let mut servers = self.servers.lock().unwrap();
            let mut codes = self.invitation_codes.lock().unwrap();
            codes.insert(server.invitation_code.clone(), server.id);
            servers.insert(server.id, server);
        }
        self
    }

    pub fn with_member(self, server_id: Uuid, user_id: Uuid, role: ServerRole) -> Self {
        {
            let mut members = self.members.lock().unwrap();
            members.insert((server_id, user_id), role);
        }
        self
    }
}

#[async_trait]
impl ServerRepository for MockServerRepository {
    async fn create(&self, server: Server) -> AppResult<Server> {
        let mut servers = self.servers.lock().unwrap();
        let mut codes = self.invitation_codes.lock().unwrap();
        let mut members = self.members.lock().unwrap();

        servers.insert(server.id, server.clone());
        codes.insert(server.invitation_code.clone(), server.id);
        members.insert((server.id, server.owner_id), ServerRole::Owner);

        Ok(server)
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Server>> {
        let servers = self.servers.lock().unwrap();
        Ok(servers.get(&id).cloned())
    }

    async fn find_by_invitation_code(&self, invitation_code: &str) -> AppResult<Option<Server>> {
        let codes = self.invitation_codes.lock().unwrap();
        let servers = self.servers.lock().unwrap();

        if let Some(server_id) = codes.get(invitation_code) {
            Ok(servers.get(server_id).cloned())
        } else {
            Ok(None)
        }
    }

    async fn find_by_user(&self, user_id: Uuid) -> AppResult<Vec<Server>> {
        let members = self.members.lock().unwrap();
        let servers = self.servers.lock().unwrap();

        let mut user_servers = Vec::new();
        for ((server_id, member_id), _) in members.iter() {
            if *member_id == user_id {
                if let Some(server) = servers.get(server_id) {
                    user_servers.push(server.clone());
                }
            }
        }

        Ok(user_servers)
    }

    async fn update(&self, server: Server) -> AppResult<Server> {
        let mut servers = self.servers.lock().unwrap();
        servers.insert(server.id, server.clone());
        Ok(server)
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        let mut servers = self.servers.lock().unwrap();
        let mut members = self.members.lock().unwrap();
        let mut codes = self.invitation_codes.lock().unwrap();

        if let Some(server) = servers.remove(&id) {
            codes.remove(&server.invitation_code);
            members.retain(|(server_id, _), _| *server_id != id);
        }

        Ok(())
    }

    async fn add_member(&self, server_id: Uuid, user_id: Uuid) -> AppResult<()> {
        let mut members = self.members.lock().unwrap();
        members.insert((server_id, user_id), ServerRole::Member);
        Ok(())
    }

    async fn remove_member(&self, server_id: Uuid, user_id: Uuid) -> AppResult<()> {
        let mut members = self.members.lock().unwrap();
        members.remove(&(server_id, user_id));
        Ok(())
    }

    async fn is_member(&self, server_id: Uuid, user_id: Uuid) -> AppResult<bool> {
        let members = self.members.lock().unwrap();
        Ok(members.contains_key(&(server_id, user_id)))
    }

    async fn get_members(&self, server_id: Uuid) -> AppResult<Vec<(Uuid, Uuid, ServerRole, chrono::DateTime<chrono::Utc>)>> {
        let members = self.members.lock().unwrap();
        let server_members: Vec<(Uuid, Uuid, ServerRole, chrono::DateTime<chrono::Utc>)> = members
            .iter()
            .filter_map(|((sid, uid), role)| {
                if *sid == server_id {
                    Some((*sid, *uid, role.clone(), chrono::Utc::now()))
                } else {
                    None
                }
            })
            .collect();
        Ok(server_members)
    }

    async fn get_member_role(
        &self,
        server_id: Uuid,
        user_id: Uuid,
    ) -> AppResult<Option<ServerRole>> {
        let members = self.members.lock().unwrap();
        Ok(members.get(&(server_id, user_id)).cloned())
    }

    async fn update_member_role(
        &self,
        server_id: Uuid,
        user_id: Uuid,
        role: ServerRole,
    ) -> AppResult<()> {
        let mut members = self.members.lock().unwrap();
        members.insert((server_id, user_id), role);
        Ok(())
    }
}
