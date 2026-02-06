use serde::{Deserialize, Serialize};

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

    /// Indiquer que l'utilisateur est en train de taper
    Typing { channel_id: String, is_typing: bool },
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
        message_id: String,
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
    UserLeft { channel_id: String, user_id: String },

    /// Un utilisateur est en train de taper
    UserTyping {
        channel_id: String,
        user_id: String,
        username: String,
        is_typing: bool,
    },

    /// Historique des messages d'un channel (envoyé après JoinChannel)
    MessageHistory {
        channel_id: String,
        messages: Vec<MessageData>,
    },

    /// Erreur avec code et détails
    Error {
        code: String,
        message: String,
        channel_id: Option<String>,
    },
}

/// Structure pour les données d'un message dans l'historique
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageData {
    pub message_id: String,
    pub user_id: String,
    pub username: String,
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl ServerMessage {
    /// Convertir en texte JSON pour l'envoi WebSocket
    pub fn to_text(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}
