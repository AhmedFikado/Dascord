use dashmap::DashMap;
use std::collections::HashSet;
use std::sync::Arc;
use uuid::Uuid;

use super::connection::Connection;
use super::message::{ClientMessage, ServerMessage};

/// Gestionnaire central de toutes les connexions WebSocket
pub struct ConnectionManager {
    /// Toutes les connexions actives (connection_id -> Connection)
    connections: DashMap<Uuid, Arc<Connection>>,
    
    /// Rooms (channel_id -> set of connection_ids)
    rooms: DashMap<String, HashSet<Uuid>>,
}

impl ConnectionManager {
    /// Créer un nouveau gestionnaire
    pub fn new() -> Self {
        Self {
            connections: DashMap::new(),
            rooms: DashMap::new(),
        }
    }
    
    /// Ajouter une connexion
    pub async fn add_connection(&self, connection: Connection) {
        let id = connection.id;
        self.connections.insert(id, Arc::new(connection));
        tracing::info!("Connexion ajoutée: {}", id);
    }
    
    /// Supprimer une connexion
    pub async fn remove_connection(&self, connection_id: Uuid) {
        // Retirer de toutes les rooms
        for mut room in self.rooms.iter_mut() {
            room.value_mut().remove(&connection_id);
        }
        
        // Supprimer la connexion
        self.connections.remove(&connection_id);
        tracing::info!("Connexion supprimée: {}", connection_id);
    }
    
    /// Envoyer un message à une connexion spécifique
    pub async fn send_to_connection(
        &self,
        connection_id: Uuid,
        message: ServerMessage,
    ) -> Result<(), String> {
        if let Some(conn) = self.connections.get(&connection_id) {
            conn.send(message)
        } else {
            Err("Connection not found".to_string())
        }
    }
    
    /// Broadcast un message à tous les membres d'un channel
    pub async fn broadcast_to_channel(&self, channel_id: &str, message: ServerMessage) {
        if let Some(room) = self.rooms.get(channel_id) {
            for conn_id in room.value().iter() {
                let _ = self.send_to_connection(*conn_id, message.clone()).await;
            }
        }
    }
    
    /// Mettre une connexion à un channel
    pub async fn join_channel(&self, connection_id: Uuid, channel_id: String) {
        self.rooms
            .entry(channel_id.clone())
            .or_insert_with(HashSet::new)
            .insert(connection_id);
        
        // Notifier les autres membres
        if let Some(conn) = self.connections.get(&connection_id) {
            let message = ServerMessage::UserJoined {
                channel_id: channel_id.clone(),
                user_id: conn.user_id.to_string(),
                username: conn.username.clone(),
            };
            self.broadcast_to_channel(&channel_id, message).await;
        }
        
        tracing::info!("📥 Connexion {} a rejoint le channel {}", connection_id, channel_id);
    }
    
    /// Retirer une connexion d'un channel
    pub async fn leave_channel(&self, connection_id: Uuid, channel_id: String) {
        if let Some(mut room) = self.rooms.get_mut(&channel_id) {
            room.remove(&connection_id);
        }
        
        // Notifier les autres membres
        if let Some(conn) = self.connections.get(&connection_id) {
            let message = ServerMessage::UserLeft {
                channel_id: channel_id.clone(),
                user_id: conn.user_id.to_string(),
            };
            self.broadcast_to_channel(&channel_id, message).await;
        }
        
        tracing::info!("Connexion {} a quitté le channel {}", connection_id, channel_id);
    }
    
    /// Gérer les messages reçus du client
    pub async fn handle_client_message(&self, connection_id: Uuid, message: ClientMessage) {
        match message {
            ClientMessage::JoinChannel { channel_id } => {
                self.join_channel(connection_id, channel_id).await;
            }
            
            ClientMessage::LeaveChannel { channel_id } => {
                self.leave_channel(connection_id, channel_id).await;
            }
            
            ClientMessage::SendMessage { channel_id, content } => {
                // Récupérer les infos de l'utilisateur
                if let Some(conn) = self.connections.get(&connection_id) {
                    let message = ServerMessage::NewMessage {
                        channel_id: channel_id.clone(),
                        user_id: conn.user_id.to_string(),
                        username: conn.username.clone(),
                        content,
                        created_at: chrono::Utc::now(),
                    };
                    
                    // Broadcast à tous les membres du channel
                    self.broadcast_to_channel(&channel_id, message).await;
                }
            }
            
            ClientMessage::Ping => {
                let _ = self.send_to_connection(connection_id, ServerMessage::Pong).await;
            }
        }
    }
    
    /// Nombre de connexions actives
    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }
}
