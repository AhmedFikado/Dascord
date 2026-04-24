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
  reactions?: Record<string, string[]>;
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
      type: 'UserAvatarUpdated';
      payload: {
        user_id: string;
        avatar_id: string;
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
      type: 'ServerMemberUpdated';
      payload: {
        server_id: string;
        user_id: string;
        username: string;
      };
    }
  | {
      type: 'MemberRoleUpdated';
      payload: {
        server_id: string;
        user_id: string;
        new_role: string;
      };
    }
  | {
      type: 'MemberKicked';
      payload: {
        server_id: string;
        user_id: string;
      };
    }
  | {
      type: 'MemberBanned';
      payload: {
        server_id: string;
        user_id: string;
      };
    }
  | {
      type: 'ReactionAdded';
      payload: {
        channel_id: string;
        message_id: string;
        user_id: string;
        reaction: string;
      };
    }
  | {
      type: 'ReactionRemoved';
      payload: {
        channel_id: string;
        message_id: string;
        user_id: string;
        reaction: string;
      };
    }
  | {
      type: 'Error';
      payload: {
        code: string;
        message: string;
        channel_id?: string;
      };
    }
  | {
      type: 'PrivateChannelCreated';
      payload: {
        channel_id: string;
        user1_id: string;
        user2_id: string;
      };
    }
  | {
      type: 'PrivateChannelHidden';
      payload: {
        channel_id: string;
        user_id: string;
      };
    }
  | {
      type: 'UnreadUpdate';
      payload: {
        server_id: string;
        channel_id: string;
        first_unread_message_id: string;
      };
    }
  | {
      type: 'ChannelCreated';
      payload: {
        server_id: string;
        channel_id: string;
        channel_name: string;
        channel_type: string;
      };
    }
  | {
      type: 'ChannelDeleted';
      payload: {
        server_id: string;
        channel_id: string;
      };
    }
  | {
      type: 'ServerDeleted';
      payload: {
        server_id: string;
      };
    }
  | {
      type: 'ServerNameUpdated';
      payload: {
        server_id: string;
        name: string;
      };
    }
  | {
      type: 'ChannelNameUpdated';
      payload: {
        channel_id: string;
        server_id: string;
        name: string;
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
