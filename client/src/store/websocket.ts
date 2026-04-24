import { useAuthStore } from '@/app/lib/stores/use-auth-store';
import { useServerStore } from '@/app/lib/stores/use-server-store';
import { usePrivateChannelStore } from '@/app/lib/stores/use-private-channel-store';
import { useUnreadStore } from '@/app/lib/stores/use-unread-store';
import { useChannelStore } from '@/app/lib/stores/use-channel-store';
import { Member } from '@/types/models/member';
import { Role } from '@/types/models/role';
import { Status } from '@/types/models/status';
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
  // IDs des canaux privés reçus via PrivateChannelCreated (pour la détection dans NewMessage)
  knownPrivateChannelIds: Set<string>;

  onIncomingMessage: ((senderName: string, preview: string) => void) | null;
  setNotificationCallback: (cb: ((senderName: string, preview: string) => void) | null) => void;

  // Actions
  setStatus: (status: WebSocketStatus) => void;
  setError: (error: string | null) => void;
  handleServerMessage: (message: ServerMessage) => void;
  addMessage: (channelId: string, message: MessageData) => void;
  removeMessage: (channelId: string, messageId: string) => void;
  updateMessage: (channelId: string, messageId: string, content: string) => void;
  updateReaction: (channelId: string, messageId: string, emoji: string, userId: string, add: boolean) => void;
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
  knownPrivateChannelIds: new Set<string>(),
  onIncomingMessage: null,

  setNotificationCallback: (cb) => set({ onIncomingMessage: cb }),

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
        // Notification Electron pour les messages d'autres utilisateurs
        {
          const cb = get().onIncomingMessage;
          const currentUserId = useAuthStore.getState().userId;
          if (cb && message.payload.user_id !== currentUserId) {
            cb(message.payload.username, message.payload.content.slice(0, 100));
          }
        }
        // Si c'est un canal privé connu, remonter le canal en tête de liste
        {
          const channelId = message.payload.channel_id;
          const privateStore = usePrivateChannelStore.getState();
          const isKnownPrivate = privateStore.privateChannels.some(ch => ch.id === channelId);
          if (isKnownPrivate) {
            usePrivateChannelStore.setState(state => {
              const channel = state.privateChannels.find(ch => ch.id === channelId);
              if (!channel) return state;
              return {
                privateChannels: [channel, ...state.privateChannels.filter(ch => ch.id !== channelId)],
              };
            });
          } else if (get().knownPrivateChannelIds.has(channelId)) {
            // Canal privé connu (via PrivateChannelCreated) mais pas encore chargé → re-fetch
            privateStore.fetchPrivateChannels();
          }
        }
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
        // Mettre à jour le statut dans le store des membres du serveur
        {
          const members = useServerStore.getState().members;
          const updatedMembers = members.map((member: Member) =>
            member.user_id === message.payload.user_id
              ? { ...member, user: { ...member.user, status: message.payload.status as Status } }
              : member
          );
          useServerStore.setState({ members: updatedMembers });
        }
        // Mettre à jour le statut du recipient dans les conversations privées
        {
          const privateStore = usePrivateChannelStore.getState();
          const updatedChannels = privateStore.privateChannels.map(ch => {
            if (ch.recipient_user?.id === message.payload.user_id) {
              return { ...ch, recipient_user: { ...ch.recipient_user!, status: message.payload.status } };
            }
            return ch;
          });
          usePrivateChannelStore.setState({ privateChannels: updatedChannels });
          // Mettre à jour currentPrivateChannel aussi si concerné
          const current = privateStore.currentPrivateChannel;
          if (current?.recipient_user?.id === message.payload.user_id) {
            usePrivateChannelStore.setState({
              currentPrivateChannel: {
                ...current,
                recipient_user: { ...current.recipient_user!, status: message.payload.status },
              },
            });
          }
        }
        break;

      case 'UserAvatarUpdated':
        {
          const { user_id, avatar_id } = message.payload;

          const authStore = useAuthStore.getState();
          if (authStore.user?.id === user_id) {
            useAuthStore.setState({
              user: { ...authStore.user, avatar_id },
            });
          }

          const serverStore = useServerStore.getState();
          useServerStore.setState({
            members: serverStore.members.map((member: Member) =>
              member.user_id === user_id
                ? { ...member, user: { ...member.user, avatar_id } }
                : member
            ),
          });

          const privateStore = usePrivateChannelStore.getState();
          const updatedChannels = privateStore.privateChannels.map(channel => {
            if (channel.recipient_user?.id === user_id) {
              return {
                ...channel,
                recipient_user: {
                  ...channel.recipient_user,
                  avatar_id,
                },
              };
            }
            return channel;
          });

          const updatedCurrentChannel =
            privateStore.currentPrivateChannel?.recipient_user?.id === user_id
              ? {
                ...privateStore.currentPrivateChannel,
                recipient_user: {
                  ...privateStore.currentPrivateChannel.recipient_user,
                  avatar_id,
                },
              }
              : privateStore.currentPrivateChannel;

          usePrivateChannelStore.setState({
            privateChannels: updatedChannels,
            currentPrivateChannel: updatedCurrentChannel,
          });
        }
        break;

      case 'MessageUpdated':
        // Un message a été modifié dans le WS store
        get().updateMessage(
          message.payload.channel_id,
          message.payload.message_id,
          message.payload.content
        );
        // Sync avec le store des messages privés (pour les messages chargés via REST)
        {
          const privateStore = usePrivateChannelStore.getState();
          const channelMsgs = privateStore.messagesByChannel[message.payload.channel_id];
          if (channelMsgs) {
            usePrivateChannelStore.setState(state => ({
              messagesByChannel: {
                ...state.messagesByChannel,
                [message.payload.channel_id]: channelMsgs.map(m =>
                  m.id === message.payload.message_id
                    ? { ...m, content: message.payload.content }
                    : m
                ),
              },
            }));
          }
        }
        break;

      case 'MessageDeleted':
        // Un message a été supprimé dans le WS store
        get().removeMessage(message.payload.channel_id, message.payload.message_id);
        // Sync avec le store des messages privés (pour les messages chargés via REST)
        {
          const privateStore = usePrivateChannelStore.getState();
          const channelMsgs = privateStore.messagesByChannel[message.payload.channel_id];
          if (channelMsgs) {
            usePrivateChannelStore.setState(state => ({
              messagesByChannel: {
                ...state.messagesByChannel,
                [message.payload.channel_id]: channelMsgs.filter(
                  m => m.id !== message.payload.message_id
                ),
              },
            }));
          }
        }
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

      case 'ServerMemberUpdated':
        // Un membre a été mis à jour, rafraîchir la liste
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

      case 'MemberKicked':
      case 'MemberBanned':
        {
          const currentUserId = useAuthStore.getState().userId;

          if (currentUserId === message.payload.user_id) {
            useServerStore.getState().reset();
            window.location.href = '/servers';
          } else {
            const serverStore = useServerStore.getState();
            if (serverStore.currentServer?.id === message.payload.server_id) {
              useServerStore.setState({
                members: serverStore.members.filter(m => m.user_id !== message.payload.user_id),
              });
            }
          }
        }
        break;

      case 'PrivateChannelCreated':
        // Un nouveau canal privé a été créé — rafraîchir la liste si on est concerné
        {
          const currentUserId = useAuthStore.getState().userId;
          // Mémoriser cet ID comme canal privé (pour la détection dans NewMessage)
          set(state => ({
            knownPrivateChannelIds: new Set([...state.knownPrivateChannelIds, message.payload.channel_id]),
          }));
          if (
            currentUserId === message.payload.user1_id ||
            currentUserId === message.payload.user2_id
          ) {
            usePrivateChannelStore.getState().fetchPrivateChannels();
          }
        }
        break;

      case 'ReactionAdded':
        get().updateReaction(
          message.payload.channel_id,
          message.payload.message_id,
          message.payload.reaction,
          message.payload.user_id,
          true
        );
        // Sync avec les messages privés chargés via REST
        {
          const privateStore = usePrivateChannelStore.getState();
          const channelMsgs = privateStore.messagesByChannel[message.payload.channel_id];
          if (channelMsgs) {
            usePrivateChannelStore.setState(state => ({
              messagesByChannel: {
                ...state.messagesByChannel,
                [message.payload.channel_id]: channelMsgs.map(m => {
                  if (m.id !== message.payload.message_id) return m;
                  const users = m.reactions?.[message.payload.reaction] || [];
                  if (users.includes(message.payload.user_id)) return m;
                  return { ...m, reactions: { ...m.reactions, [message.payload.reaction]: [...users, message.payload.user_id] } };
                }),
              },
            }));
          }
        }
        break;

      case 'ReactionRemoved':
        get().updateReaction(
          message.payload.channel_id,
          message.payload.message_id,
          message.payload.reaction,
          message.payload.user_id,
          false
        );
        // Sync avec les messages privés chargés via REST
        {
          const privateStore = usePrivateChannelStore.getState();
          const channelMsgs = privateStore.messagesByChannel[message.payload.channel_id];
          if (channelMsgs) {
            usePrivateChannelStore.setState(state => ({
              messagesByChannel: {
                ...state.messagesByChannel,
                [message.payload.channel_id]: channelMsgs.map(m => {
                  if (m.id !== message.payload.message_id) return m;
                  const users = (m.reactions?.[message.payload.reaction] || []).filter(id => id !== message.payload.user_id);
                  const updatedReactions = { ...m.reactions };
                  if (users.length === 0) delete updatedReactions[message.payload.reaction];
                  else updatedReactions[message.payload.reaction] = users;
                  return { ...m, reactions: updatedReactions };
                }),
              },
            }));
          }
        }
        break;

      case 'PrivateChannelHidden':
        // Retirer le canal de la liste des conversations privées
        usePrivateChannelStore.setState(state => ({
          privateChannels: state.privateChannels.filter(ch => ch.id !== message.payload.channel_id),
        }));
        break;

      case 'UnreadUpdate':
        // Nouveau message non lu dans un channel où l'utilisateur n'est pas
        useUnreadStore.getState().addUnread(
          message.payload.server_id,
          message.payload.channel_id,
          message.payload.first_unread_message_id
        );
        {
          const cb = get().onIncomingMessage;
          if (cb) {
            const { channel_id, server_id } = message.payload;
            if (server_id === 'private') {
              const dm = usePrivateChannelStore.getState().privateChannels.find(ch => ch.id === channel_id);
              const senderName = dm?.recipient_user?.username ?? 'Message privé';
              cb(senderName, '');
            } else {
              const channelsByServer = useChannelStore.getState().channelsByServer;
              const channel = Object.values(channelsByServer).flat().find(ch => ch.id === channel_id);
              cb(`#${channel?.name ?? 'channel'}`, '');
            }
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

  // Mettre à jour les réactions d'un message
  updateReaction: (channelId: string, messageId: string, emoji: string, userId: string, add: boolean) =>
    set((state: WebSocketState) => {
      const currentMessages = state.messagesByChannel[channelId] || [];
      const updated = currentMessages.map(m => {
        if (m.message_id !== messageId) return m;
        const reactions = { ...(m.reactions || {}) };
        const users = reactions[emoji] ? [...reactions[emoji]] : [];
        if (add) {
          if (!users.includes(userId)) reactions[emoji] = [...users, userId];
        } else {
          reactions[emoji] = users.filter(id => id !== userId);
          if (reactions[emoji].length === 0) delete reactions[emoji];
        }
        return { ...m, reactions };
      });
      return { messagesByChannel: { ...state.messagesByChannel, [channelId]: updated } };
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
      knownPrivateChannelIds: new Set<string>(),
      onIncomingMessage: null,
    }),
}));
