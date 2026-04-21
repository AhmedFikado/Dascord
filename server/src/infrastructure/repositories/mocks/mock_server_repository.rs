use crate::domain::entities::{Server, BanType};
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
    bans: Arc<Mutex<HashMap<(Uuid, Uuid), Option<chrono::DateTime<chrono::Utc>>>>>, // (server_id, user_id) -> expires_at
    ban_entries: Arc<Mutex<Vec<(Uuid, Uuid, String, BanType, chrono::DateTime<chrono::Utc>, Option<chrono::DateTime<chrono::Utc>>)>>>, // (server_id, user_id, username, ban_type, banned_at, expires_at)
}

impl MockServerRepository {
    pub fn new() -> Self {
        Self {
            servers: Arc::new(Mutex::new(HashMap::new())),
            members: Arc::new(Mutex::new(HashMap::new())),
            invitation_codes: Arc::new(Mutex::new(HashMap::new())),
            bans: Arc::new(Mutex::new(HashMap::new())),
            ban_entries: Arc::new(Mutex::new(Vec::new())),
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

    pub fn with_ban(self, server_id: Uuid, user_id: Uuid, username: String, ban_type: BanType, expires_at: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        {
            let mut bans = self.bans.lock().unwrap();
            bans.insert((server_id, user_id), expires_at);
            let mut entries = self.ban_entries.lock().unwrap();
            entries.push((server_id, user_id, username, ban_type, chrono::Utc::now(), expires_at));
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

    async fn is_banned(&self, server_id: Uuid, user_id: Uuid) -> AppResult<bool> {
        let bans = self.bans.lock().unwrap();
        if let Some(expires_at) = bans.get(&(server_id, user_id)) {
            match expires_at {
                None => Ok(true), // permanent
                Some(exp) => Ok(*exp > chrono::Utc::now()), // temporaire actif
            }
        } else {
            Ok(false)
        }
    }

    async fn ban_member(&self, server_id: Uuid, user_id: Uuid, _banned_by: Uuid, ban_type: BanType, expires_at: Option<chrono::DateTime<chrono::Utc>>) -> AppResult<()> {
        {
            let mut bans = self.bans.lock().unwrap();
            bans.insert((server_id, user_id), expires_at);
            let mut entries = self.ban_entries.lock().unwrap();
            entries.push((server_id, user_id, String::new(), ban_type, chrono::Utc::now(), expires_at));
        }
        self.remove_member(server_id, user_id).await
    }

    async fn list_bans(&self, server_id: Uuid) -> AppResult<Vec<(Uuid, String, BanType, chrono::DateTime<chrono::Utc>, Option<chrono::DateTime<chrono::Utc>>)>> {
        let bans = self.bans.lock().unwrap();
        let entries = self.ban_entries.lock().unwrap();
        let now = chrono::Utc::now();

        let result = entries.iter()
            .filter(|(sid, uid, _, _, _, expires_at)| {
                *sid == server_id && bans.contains_key(&(*sid, *uid)) && {
                    match expires_at {
                        None => true,
                        Some(exp) => *exp > now,
                    }
                }
            })
            .map(|(_, uid, username, ban_type, banned_at, expires_at)| {
                (*uid, username.clone(), *ban_type, *banned_at, *expires_at)
            })
            .collect();

        Ok(result)
    }

    async fn unban_member(&self, server_id: Uuid, user_id: Uuid) -> AppResult<()> {
        let mut bans = self.bans.lock().unwrap();
        bans.remove(&(server_id, user_id));
        let mut entries = self.ban_entries.lock().unwrap();
        entries.retain(|(sid, uid, _, _, _, _)| !(*sid == server_id && *uid == user_id));
        Ok(())
    }
}
