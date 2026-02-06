use axum::extract::ws::{Message, WebSocket};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::mpsc;
use uuid::Uuid;

use super::manager::ConnectionManager;
use super::message::{ClientMessage, ServerMessage};
use crate::infrastructure::database::MongoDBMessageRepository;

/// Représente une connexion WebSocket individuelle
pub struct Connection {
    /// ID unique de la connexion
    pub id: Uuid,

    /// ID + nom utilisateur connecté
    pub user_id: Uuid,
    pub username: String,

    /// Sender pour envoyer des messages à cette connexion
    tx: mpsc::UnboundedSender<ServerMessage>,
}

impl Connection {
    /// Créer une nouvelle connexion
    pub fn new(user_id: Uuid, username: String, tx: mpsc::UnboundedSender<ServerMessage>) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            username,
            tx,
        }
    }

    /// Envoyer un message à cette connexion
    pub fn send(&self, message: ServerMessage) -> Result<(), String> {
        self.tx
            .send(message)
            .map_err(|e| format!("Failed to send message: {}", e))
    }
}

/// Gérer une connexion WebSocket (lecture + écriture)
pub async fn handle_socket(
    socket: WebSocket,
    user_id: Uuid,
    username: String,
    manager: Arc<ConnectionManager>,
    message_repository: MongoDBMessageRepository,
) {
    // Séparer le socket en deux parties : écriture (sink) et lecture (stream)
    let (mut sink, mut stream) = socket.split();

    // Canal pour communiquer entre les tâches
    let (tx, mut rx) = mpsc::unbounded_channel::<ServerMessage>();

    // Créer la connexion
    let connection = Connection::new(user_id, username.clone(), tx);
    let connection_id = connection.id;

    // Enregistrer la connexion
    manager.add_connection(connection).await;

    // Envoyer message de confirmation
    let _ = manager
        .send_to_connection(
            connection_id,
            ServerMessage::Connected {
                user_id: user_id.to_string(),
            },
        )
        .await;

    // Écriture (envoyer messages au client)
    let write_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let Ok(text) = msg.to_text() {
                if sink.send(Message::Text(text)).await.is_err() {
                    break;
                }
            }
        }
    });

    // Lecture (recevoir messages du client)
    let manager_clone = manager.clone();
    let read_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = stream.next().await {
            if let Message::Text(text) = msg {
                if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                    manager_clone
                        .handle_client_message(connection_id, client_msg, &message_repository)
                        .await;
                }
            } else if let Message::Close(_) = msg {
                break;
            }
        }
    });

    // Attendre que l'une des tâches se termine
    tokio::select! {
        _ = write_task => {},
        _ = read_task => {},
    }

    // Nettoyer la connexion
    manager.remove_connection(connection_id).await;
}
