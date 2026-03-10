// Message envoyé client -> serveur
export type ClientMessage =
  | {
      type: 'JoinChannel';
      payload: { channel_id: string };
    }
  | {
      type: 'LeaveChannel';
      payload: { channel_id: string };
    }
  | {
      type: 'SendMessage';
      payload: { channel_id: string; content: string };
    }
  | {
      type: 'Typing';
      payload: { channel_id: string; is_typing: boolean };
    };

// Donnée message historique
export interface MessageData {
  message_id: string;
  user_id: string;
  username: string;
  content: string;
  created_at: string;
}

// Message reçu du serveur
export type ServerMessage =
  | {
      type: 'Connected';
      payload: { user_id: string };
    }
  | {
      type: 'NewMessage';
      payload: {
        channel_id: string;
        message_id: string;
        user_id: string;
        username: string;
        content: string;
        created_at: string;
      };
    }
  | {
      type: 'UserJoined';
      payload: {
        channel_id: string;
        user_id: string;
        username: string;
      };
    }
  | {
      type: 'UserLeft';
      payload: {
        channel_id: string;
        user_id: string;
      };
    }
  | {
      type: 'UserTyping';
      payload: {
        channel_id: string;
        user_id: string;
        username: string;
        is_typing: boolean;
      };
    }
  | {
      type: 'UserStatusChanged';
      payload: {
        user_id: string;
        status: string;
      };
    }
  | {
      type: 'MessageHistory';
      payload: {
        channel_id: string;
        messages: MessageData[];
      };
    }
  | {
      type: 'MessageUpdated';
      payload: {
        channel_id: string;
        message_id: string;
        user_id: string;
        content: string;
      };
    }
  | {
      type: 'MessageDeleted';
      payload: {
        channel_id: string;
        message_id: string;
      };
    }
  | {
      type: 'ServerMemberJoined';
      payload: {
        server_id: string;
        user_id: string;
        username: string;
      };
    }
  | {
      type: 'ServerMemberLeft';
      payload: {
        server_id: string;
        user_id: string;
      };
    }
  | {
      type: 'Error';
      payload: {
        code: string;
        message: string;
        channel_id?: string;
      };
    };

// État de la connexion WebSocket
export enum WebSocketStatus {
  DISCONNECTED = 'disconnected',
  CONNECTING = 'connecting',
  CONNECTED = 'connected',
  RECONNECTING = 'reconnecting',
  ERROR = 'error',
}

// Configuration WebSocket
export interface WebSocketConfig {
  url?: string;
  reconnectInterval?: number;
  maxReconnectAttempts?: number;
}
