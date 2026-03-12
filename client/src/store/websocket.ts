import { useServerStore } from '@/app/lib/stores/use-server-store';
import { Member } from '@/types/models/member';
import { Role } from '@/types/models/role';
import { Status } from '@/types/models/user';
import { MessageData, ServerMessage, WebSocketStatus } from '@/types/websocket';
import { create } from 'zustand';

interface TypingUser {
  user_id: string;
  username: string;
  timestamp: number;
}

// État global du WebSocket
//Contient les messages, les utilisateurs en train de taper, et le statut de connexion
interface WebSocketState {
  // État de la connexion
  status: WebSocketStatus;
  error: string | null;

  // Message par channel
  messagesByChannel: Record<string, MessageData[]>;
  typingByChannel: Record<string, TypingUser[]>;
  // Utilisateurs présents dans chaque channel (pour afficher qui est en ligne)
  usersByChannel: Record<string, Set<string>>;

  // Actions
  setStatus: (status: WebSocketStatus) => void;
  setError: (error: string | null) => void;
  handleServerMessage: (message: ServerMessage) => void;
  addMessage: (channelId: string, message: MessageData) => void;
  removeMessage: (channelId: string, messageId: string) => void;
  updateMessage: (channelId: string, messageId: string, content: string) => void;
  setMessages: (channelId: string, messages: MessageData[]) => void;
  setTyping: (channelId: string, userId: string, username: string, isTyping: boolean) => void;
  // Gestion des utilisateurs dans les channels
  addUser: (channelId: string, userId: string) => void;
  removeUser: (channelId: string, userId: string) => void;
  clearChannel: (channelId: string) => void;
  reset: () => void;
}

// Durée avant de considérer qu'un utilisateur a arrêté de taper (3 secondes)
const TYPING_TIMEOUT = 3000;

export const useWebSocketStore = create<WebSocketState>((set, get) => ({
  status: WebSocketStatus.DISCONNECTED,
  error: null,
  messagesByChannel: {},
  typingByChannel: {},
  usersByChannel: {},

  // Mettre à jour le statut de connexion
  setStatus: (status: WebSocketStatus) => set({ status }),

  setError: (error: string | null) => set({ error }),

  // Gérer les messages reçus du serveur
  handleServerMessage: (message: ServerMessage) => {
    switch (message.type) {
      case 'Connected':
        // Confirmation de connexion réussie
        set({ status: WebSocketStatus.CONNECTED, error: null });
        break;

      case 'NewMessage':
        // Nouveau message reçu, l'ajouter au channel
        get().addMessage(message.payload.channel_id, {
          message_id: message.payload.message_id,
          user_id: message.payload.user_id,
          username: message.payload.username,
          content: message.payload.content,
          created_at: message.payload.created_at,
        });
        break;

      case 'MessageHistory':
        // Historique des messages d'un channel (reçu après JoinChannel)
        get().setMessages(message.payload.channel_id, message.payload.messages);
        break;

      case 'UserJoined':
        // Un utilisateur a rejoint le channel, l'ajouter à la liste
        get().addUser(message.payload.channel_id, message.payload.user_id);
        break;

      case 'UserLeft':
        // Un utilisateur a quitté le channel, le retirer de la liste
        get().removeUser(message.payload.channel_id, message.payload.user_id);
        break;

      case 'UserTyping':
        get().setTyping(
          message.payload.channel_id,
          message.payload.user_id,
          message.payload.username,
          message.payload.is_typing
        );
        break;

      case 'UserStatusChanged':
        // Mettre à jour le statut dans le store des membres
        {
          const members = useServerStore.getState().members;
          const updatedMembers = members.map((member: Member) =>
            member.user_id === message.payload.user_id
              ? { ...member, user: { ...member.user, status: message.payload.status as Status } }
              : member
          );
          useServerStore.setState({ members: updatedMembers });
        }
        break;

      case 'MessageUpdated':
        // Un message a été modifié
        get().updateMessage(
          message.payload.channel_id,
          message.payload.message_id,
          message.payload.content
        );
        break;

      case 'MessageDeleted':
        // Un message a été supprimé
        get().removeMessage(message.payload.channel_id, message.payload.message_id);
        break;

      case 'ServerMemberJoined':
        // Un nouveau membre a rejoint le serveur, rafraîchir la liste
        {
          const serverStore = useServerStore.getState();
          const currentServer = serverStore.currentServer;
          const members = serverStore.members;

          // Si on est sur le serveur concerné et qu'on a déjà des membres chargés
          if (currentServer?.id === message.payload.server_id && members.length > 0) {
            serverStore.getMembers(message.payload.server_id);
          }
        }
        break;

      case 'ServerMemberLeft':
        // Un membre a quitté le serveur, rafraîchir la liste
        {
          const serverStore = useServerStore.getState();
          const currentServer = serverStore.currentServer;
          const members = serverStore.members;

          // Si on est sur le serveur concerné et qu'on a déjà des membres chargés
          if (currentServer?.id === message.payload.server_id && members.length > 0) {
            serverStore.getMembers(message.payload.server_id);
          }
        }
        break;

      case 'MemberRoleUpdated':
        // Le rôle d'un membre a changé, mettre à jour le store
        {
          const serverStore = useServerStore.getState();
          const currentServer = serverStore.currentServer;

          // Si on est sur le serveur concerné
          if (currentServer?.id === message.payload.server_id) {
            // Mettre à jour le rôle du membre localement en récupérant toujours le state le plus récent
            const latestMembers = useServerStore.getState().members;
            const updatedMembers = latestMembers.map((member: Member) => {
              if (member.user_id === message.payload.user_id) {
                return { ...member, role: message.payload.new_role as Role };
              }
              return member;
            });

            useServerStore.setState({ members: updatedMembers });
          }
        }
        break;

      case 'Error':
        console.error('WebSocket error:', message.payload);
        set({ error: message.payload.message });
        break;
    }
  },

  // Ajouter un nouveau message à un channel
  addMessage: (channelId: string, message: MessageData) =>
    set((state: WebSocketState) => ({
      messagesByChannel: {
        ...state.messagesByChannel,
        [channelId]: [...(state.messagesByChannel[channelId] || []), message],
      },
    })),

  // Supprimer un message d'un channel
  removeMessage: (channelId: string, messageId: string) =>
    set((state: WebSocketState) => {
      const currentMessages = state.messagesByChannel[channelId] || [];
      const filtered = currentMessages.filter(m => m.message_id !== messageId);

      return {
        messagesByChannel: {
          ...state.messagesByChannel,
          [channelId]: filtered,
        },
      };
    }),

  // Mettre à jour un message dans un channel
  updateMessage: (channelId: string, messageId: string, content: string) =>
    set((state: WebSocketState) => {
      const currentMessages = state.messagesByChannel[channelId] || [];
      const updated = currentMessages.map(m =>
        m.message_id === messageId ? { ...m, content } : m
      );

      return {
        messagesByChannel: {
          ...state.messagesByChannel,
          [channelId]: updated,
        },
      };
    }),

  // Définir l'historique complet des messages d'un channel
  setMessages: (channelId: string, messages: MessageData[]) =>
    set((state: WebSocketState) => ({
      messagesByChannel: {
        ...state.messagesByChannel,
        [channelId]: messages,
      },
    })),

  // Gérer l'indicateur "en train de taper" pour un utilisateur
  setTyping: (channelId: string, userId: string, username: string, isTyping: boolean) =>
    set((state: WebSocketState) => {
      const currentTyping = state.typingByChannel[channelId] || [];
      const now = Date.now();

      let newTyping: TypingUser[];
      if (isTyping) {
        // Ajouter ou mettre à jour l'utilisateur dans la liste
        const filtered = currentTyping.filter((t: TypingUser) => t.user_id !== userId);
        newTyping = [...filtered, { user_id: userId, username, timestamp: now }];
      } else {
        // Retirer l'utilisateur de la liste
        newTyping = currentTyping.filter((t: TypingUser) => t.user_id !== userId);
      }

      // Nettoyer les utilisateurs dont le timeout a expiré
      newTyping = newTyping.filter((t: TypingUser) => now - t.timestamp < TYPING_TIMEOUT);

      return {
        typingByChannel: {
          ...state.typingByChannel,
          [channelId]: newTyping,
        },
      };
    }),

  // Ajouter un utilisateur à la liste des membres présents dans un channel
  addUser: (channelId: string, userId: string) =>
    set((state: WebSocketState) => {
      const users = new Set(state.usersByChannel[channelId] || []);
      users.add(userId);
      return {
        usersByChannel: {
          ...state.usersByChannel,
          [channelId]: users,
        },
      };
    }),

  // Retirer un utilisateur de la liste des membres présents dans un channel
  removeUser: (channelId: string, userId: string) =>
    set((state: WebSocketState) => {
      const users = new Set(state.usersByChannel[channelId] || []);
      users.delete(userId);
      return {
        usersByChannel: {
          ...state.usersByChannel,
          [channelId]: users,
        },
      };
    }),

  // Nettoyer toutes les données d'un channel spécifique
  clearChannel: (channelId: string) =>
    set((state: WebSocketState) => {
      const { [channelId]: _, ...restMessages } = state.messagesByChannel;
      const { [channelId]: __, ...restTyping } = state.typingByChannel;
      const { [channelId]: ___, ...restUsers } = state.usersByChannel;

      return {
        messagesByChannel: restMessages,
        typingByChannel: restTyping,
        usersByChannel: restUsers,
      };
    }),

  // Réinitialiser complètement le store (lors de la déconnexion)
  reset: () =>
    set({
      status: WebSocketStatus.DISCONNECTED,
      error: null,
      messagesByChannel: {},
      typingByChannel: {},
      usersByChannel: {},
    }),
}));
