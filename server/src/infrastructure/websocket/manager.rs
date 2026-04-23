use dashmap::DashMap;
use sqlx::{PgPool, Row};
use std::collections::HashSet;
use std::sync::Arc;
use uuid::Uuid;

use super::connection::Connection;
use super::message::{ClientMessage, MessageData, ServerMessage};
use crate::domain::entities::message::Message;
use crate::infrastructure::database::MongoDBMessageRepository;

/// Gestionnaire central de toutes les connexions WebSocket
pub struct ConnectionManager {
    /// Toutes les connexions actives (connection_id -> Connection)
    connections: DashMap<Uuid, Arc<Connection>>,

    /// Rooms (channel_id -> set of connection_ids)
    rooms: DashMap<String, HashSet<Uuid>>,

    /// Mapping user_id -> connection_id (pour envoyer directement à un utilisateur)
    user_connections: DashMap<Uuid, Uuid>,

    /// Pool PostgreSQL optionnel (pour le tracking des messages non lus)
    pg_pool: Option<PgPool>,
}

impl ConnectionManager {
    /// Créer un nouveau gestionnaire
    pub fn new() -> Self {
        Self {
            connections: DashMap::new(),
            rooms: DashMap::new(),
            user_connections: DashMap::new(),
            pg_pool: None,
        }
    }

    /// Attacher un pool PostgreSQL (pour le tracking non lus)
    pub fn with_pg_pool(mut self, pool: PgPool) -> Self {
        self.pg_pool = Some(pool);
        self
    }

    /// Ajouter une connexion
    pub async fn add_connection(&self, connection: Connection) {
        let id = connection.id;
        let user_id = connection.user_id;
        self.connections.insert(id, Arc::new(connection));
        self.user_connections.insert(user_id, id);
        tracing::info!("Connexion ajoutée: {}", id);
    }

    /// Supprimer une connexion
    pub async fn remove_connection(&self, connection_id: Uuid) {
        // Retirer de toutes les rooms
        for mut room in self.rooms.iter_mut() {
            room.value_mut().remove(&connection_id);
        }

        // Retirer du mapping user_connections
        if let Some(conn) = self.connections.get(&connection_id) {
            self.user_connections.remove(&conn.user_id);
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

    /// Broadcast un message à toutes les connexions actives
    pub async fn broadcast_to_all(&self, message: ServerMessage) {
        for entry in self.connections.iter() {
            let conn_id = *entry.key();
            let _ = self.send_to_connection(conn_id, message.clone()).await;
        }
    }

    /// Envoyer un message directement à un utilisateur (par user_id)
    pub async fn send_to_user(&self, user_id: Uuid, message: ServerMessage) {
        if let Some(conn_id) = self.user_connections.get(&user_id) {
            let _ = self.send_to_connection(*conn_id, message).await;
        }
    }

    /// Mettre une connexion à un channel
    pub async fn join_channel(
        &self,
        connection_id: Uuid,
        channel_id: String,
        message_repository: &MongoDBMessageRepository,
    ) {
        self.rooms
            .entry(channel_id.clone())
            .or_insert_with(HashSet::new)
            .insert(connection_id);

        // Récupérer et envoyer l'historique des messages
        match message_repository
            .get_messages_by_channel(&channel_id)
            .await
        {
            Ok(messages) => {
                let message_data: Vec<MessageData> = messages
                    .iter()
                    .map(|msg| MessageData {
                        message_id: msg
                            .id
                            .as_ref()
                            .map(|id| id.to_hex())
                            .unwrap_or_else(|| "unknown".to_string()),
                        user_id: msg.user_id.clone(),
                        username: msg.username.clone(),
                        content: msg.content.clone(),
                        created_at: msg.created_at,
                        reactions: msg.reactions.clone(),
                    })
                    .collect();

                let history_msg = ServerMessage::MessageHistory {
                    channel_id: channel_id.clone(),
                    messages: message_data,
                };

                let _ = self.send_to_connection(connection_id, history_msg).await;
                tracing::info!(
                    "Envoyé {} messages d'historique pour le channel {}",
                    messages.len(),
                    channel_id
                );
            }
            Err(e) => {
                tracing::error!("Erreur lors de la récupération de l'historique: {:?}", e);
                let error_msg = ServerMessage::Error {
                    code: "HISTORY_FETCH_ERROR".to_string(),
                    message: "Impossible de récupérer l'historique des messages".to_string(),
                    channel_id: Some(channel_id.clone()),
                };
                let _ = self.send_to_connection(connection_id, error_msg).await;
            }
        }

        // Notifier les autres membres
        if let Some(conn) = self.connections.get(&connection_id) {
            let message = ServerMessage::UserJoined {
                channel_id: channel_id.clone(),
                user_id: conn.user_id.to_string(),
                username: conn.username.clone(),
            };
            self.broadcast_to_channel(&channel_id, message).await;
        }

        tracing::info!(
            "Connexion {} a rejoint le channel {}",
            connection_id,
            channel_id
        );
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

        tracing::info!(
            "Connexion {} a quitté le channel {}",
            connection_id,
            channel_id
        );
    }

    /// Notifier les membres du serveur non présents dans le channel qu'il y a un message non lu
    async fn notify_unread(
        &self,
        channel_id: &str,
        message_id: &str,
        sender_connection_id: Uuid,
        pg_pool: &PgPool,
    ) {
        let channel_uuid = match Uuid::parse_str(channel_id) {
            Ok(u) => u,
            Err(_) => return,
        };

        // Récupérer le server_id et tous les membres du serveur pour ce channel
        let rows = match sqlx::query(
            "SELECT sm.user_id, c.server_id
             FROM server_members sm
             INNER JOIN channels c ON c.server_id = sm.server_id
             WHERE c.id = $1",
        )
        .bind(channel_uuid)
        .fetch_all(pg_pool)
        .await
        {
            Ok(r) => r,
            Err(e) => {
                tracing::error!("Erreur récupération membres pour unread: {:?}", e);
                return;
            }
        };

        if rows.is_empty() {
            return;
        }

        let server_id: Uuid = rows[0].get("server_id");

        // Récupérer les connection_ids abonnés au channel (sauf l'émetteur)
        let subscribed_conn_ids: HashSet<Uuid> = self
            .rooms
            .get(channel_id)
            .map(|r| r.value().clone())
            .unwrap_or_default();

        for row in &rows {
            let member_user_id: Uuid = row.get("user_id");

            // Récupérer la connexion de ce membre
            let member_conn_id = match self.user_connections.get(&member_user_id) {
                Some(c) => *c,
                None => continue, // Pas connecté
            };

            // Si le membre est l'émetteur du message, on ignore
            if member_conn_id == sender_connection_id {
                continue;
            }

            // Si le membre est abonné au channel (en train de le lire), on ignore
            if subscribed_conn_ids.contains(&member_conn_id) {
                continue;
            }

            // Insérer en base le statut non lu (ON CONFLICT DO NOTHING = on garde le premier)
            let _ = sqlx::query(
                "INSERT INTO unread_channels (user_id, channel_id, first_unread_message_id, is_private)
                 VALUES ($1, $2, $3, FALSE)
                 ON CONFLICT (user_id, channel_id) DO NOTHING",
            )
            .bind(member_user_id)
            .bind(channel_uuid)
            .bind(message_id)
            .execute(pg_pool)
            .await;

            // Envoyer l'événement UnreadUpdate
            self.send_to_user(
                member_user_id,
                ServerMessage::UnreadUpdate {
                    server_id: server_id.to_string(),
                    channel_id: channel_id.to_string(),
                    first_unread_message_id: message_id.to_string(),
                },
            )
            .await;
        }
    }

    /// Notifier le destinataire d'un message privé non lu
    pub async fn notify_private_channel_unread(
        &self,
        channel_id: &str,
        message_id: &str,
        sender_user_id: Uuid,
        recipient_user_id: Uuid,
    ) {
        // Vérifier si le destinataire est abonné au channel privé
        let subscribed = self
            .rooms
            .get(channel_id)
            .map(|r| {
                r.value()
                    .iter()
                    .any(|conn_id| {
                        self.connections
                            .get(conn_id)
                            .map(|c| c.user_id == recipient_user_id)
                            .unwrap_or(false)
                    })
            })
            .unwrap_or(false);

        if subscribed {
            return;
        }

        // Insérer en base si pg_pool disponible
        if let Some(pool) = &self.pg_pool {
            if let Ok(channel_uuid) = uuid::Uuid::parse_str(channel_id) {
                let _ = sqlx::query(
                    "INSERT INTO unread_channels (user_id, channel_id, first_unread_message_id, is_private)
                     VALUES ($1, $2, $3, TRUE)
                     ON CONFLICT (user_id, channel_id) DO NOTHING",
                )
                .bind(recipient_user_id)
                .bind(channel_uuid)
                .bind(message_id)
                .execute(pool)
                .await;
            }
        }

        // Envoyer l'événement UnreadUpdate au destinataire
        self.send_to_user(
            recipient_user_id,
            ServerMessage::UnreadUpdate {
                server_id: "private".to_string(),
                channel_id: channel_id.to_string(),
                first_unread_message_id: message_id.to_string(),
            },
        )
        .await;

        tracing::debug!(
            "UnreadUpdate (private) envoyé à {} pour channel {}",
            recipient_user_id,
            channel_id
        );
    }

    /// Gérer les messages reçus du client
    pub async fn handle_client_message(
        &self,
        connection_id: Uuid,
        message: ClientMessage,
        message_repository: &MongoDBMessageRepository,
    ) {
        match message {
            ClientMessage::JoinChannel { channel_id } => {
                self.join_channel(connection_id, channel_id, message_repository)
                    .await;
            }

            ClientMessage::LeaveChannel { channel_id } => {
                self.leave_channel(connection_id, channel_id).await;
            }

            ClientMessage::SendMessage {
                channel_id,
                content,
            } => {
                // Vérifier que le contenu n'est pas vide
                if content.trim().is_empty() {
                    let error_msg = ServerMessage::Error {
                        code: "EMPTY_MESSAGE".to_string(),
                        message: "Le message ne peut pas être vide".to_string(),
                        channel_id: Some(channel_id),
                    };
                    let _ = self.send_to_connection(connection_id, error_msg).await;
                    return;
                }

                // Récupérer les infos de l'utilisateur
                if let Some(conn) = self.connections.get(&connection_id) {
                    // Créer le message à sauvegarder
                    let new_message = Message::new(
                        channel_id.clone(),
                        conn.user_id.to_string(),
                        conn.username.clone(),
                        content.clone(),
                    );

                    // Sauvegarder en base de données
                    match message_repository.save_message(&new_message).await {
                        Ok(message_id) => {
                            // Broadcast à tous les membres du channel
                            let ws_message = ServerMessage::NewMessage {
                                channel_id: channel_id.clone(),
                                message_id: message_id.clone(),
                                user_id: conn.user_id.to_string(),
                                username: conn.username.clone(),
                                content,
                                created_at: new_message.created_at,
                            };

                            self.broadcast_to_channel(&channel_id, ws_message).await;
                            tracing::info!(
                                "Message sauvegardé et diffusé sur le channel {}",
                                channel_id
                            );

                            // Notifier les membres non abonnés (messages non lus)
                            if let Some(pool) = &self.pg_pool {
                                self.notify_unread(
                                    &channel_id,
                                    &message_id,
                                    connection_id,
                                    pool,
                                )
                                .await;
                            }
                        }
                        Err(e) => {
                            tracing::error!("Erreur lors de la sauvegarde du message: {:?}", e);
                            let error_msg = ServerMessage::Error {
                                code: "MESSAGE_SAVE_ERROR".to_string(),
                                message: "Impossible de sauvegarder le message".to_string(),
                                channel_id: Some(channel_id),
                            };
                            let _ = self.send_to_connection(connection_id, error_msg).await;
                        }
                    }
                } else {
                    let error_msg = ServerMessage::Error {
                        code: "CONNECTION_NOT_FOUND".to_string(),
                        message: "Connexion introuvable".to_string(),
                        channel_id: Some(channel_id),
                    };
                    let _ = self.send_to_connection(connection_id, error_msg).await;
                }
            }

            ClientMessage::Typing {
                channel_id,
                is_typing,
            } => {
                // Récupérer les infos de l'utilisateur
                if let Some(conn) = self.connections.get(&connection_id) {
                    let message = ServerMessage::UserTyping {
                        channel_id: channel_id.clone(),
                        user_id: conn.user_id.to_string(),
                        username: conn.username.clone(),
                        is_typing,
                    };

                    // Broadcast à tous les autres membres du channel (sauf l'émetteur)
                    if let Some(room) = self.rooms.get(&channel_id) {
                        for conn_id in room.value().iter() {
                            if *conn_id != connection_id {
                                let _ = self.send_to_connection(*conn_id, message.clone()).await;
                            }
                        }
                    }

                    tracing::debug!(
                        "User {} typing status: {} dans channel {}",
                        conn.username,
                        is_typing,
                        channel_id
                    );
                } else {
                    let error_msg = ServerMessage::Error {
                        code: "CONNECTION_NOT_FOUND".to_string(),
                        message: "Connexion introuvable".to_string(),
                        channel_id: Some(channel_id),
                    };
                    let _ = self.send_to_connection(connection_id, error_msg).await;
                }
            }
        }
    }

    /// Nombre de connexions actives
    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }

    /// Broadcaster un changement de statut à toutes les connexions
    pub async fn broadcast_status_change(&self, user_id: Uuid, status: String) {
        let message = ServerMessage::UserStatusChanged {
            user_id: user_id.to_string(),
            status,
        };

        for conn in self.connections.iter() {
            let _ = conn.value().send(message.clone());
        }

        tracing::info!("Status change broadcasted for user {}", user_id);
    }
}
