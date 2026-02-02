use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Types de messages que le client peut envoyer au serveur
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum ClientMessage {
    /// S'abonner à un channel
    JoinChannel { channel_id: String },
    
    /// Se désabonner d'un channel
    LeaveChannel { channel_id: String },
    
    /// Envoyer un message dans un channel
    SendMessage { channel_id: String, content: String },
    
    /// Ping pour garder la connexion active
    Ping,
}

/// Types de messages que le serveur envoie au client
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum ServerMessage {
    /// Confirmation de connexion réussie
    Connected { user_id: String },
    
    /// Nouveau message dans un channel
    NewMessage {
        channel_id: String,
        user_id: String,
        username: String,
        content: String,
        created_at: chrono::DateTime<chrono::Utc>,
    },
    
    /// Un user a rejoint le channel
    UserJoined {
        channel_id: String,
        user_id: String,
        username: String,
    },
    
    /// Un user a quitté le channel
    UserLeft {
        channel_id: String,
        user_id: String,
    },
    
    /// Réponse au ping
    Pong,
    
    /// Erreur
    Error { message: String },
}

impl ServerMessage {
    /// Convertir en texte JSON pour l'envoi WebSocket
    pub fn to_text(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}
