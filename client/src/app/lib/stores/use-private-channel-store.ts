import { create } from 'zustand';
import { PrivateChannelWithUser } from '../../../types/models/privateChannels';
import { privateChannelsApi } from '../api/private-channels';
import { privateMessagesApi } from '../api/private-messages';
import { Message } from '@/types/models/message';
import i18n from 'i18next';
import { useWebSocketStore } from '@/store/websocket';

interface PrivateChannelState {
  privateChannels: PrivateChannelWithUser[];
  currentPrivateChannel: PrivateChannelWithUser | null;
  messagesByChannel: Record<string, Message[]>;
  isLoading: boolean;
  error: string | null;

  fetchPrivateChannels: () => Promise<void>;
  createOrGetPrivateChannel: (currentUserId: string, recipientId: string) => Promise<PrivateChannelWithUser>;
  setCurrentPrivateChannel: (channel: PrivateChannelWithUser) => void;
  fetchMessages: (channelId: string) => Promise<void>;
  sendMessage: (channelId: string, content: string) => Promise<void>;
  deleteMessage: (channelId: string, messageId: string) => Promise<void>;
  updateMessage: (channelId: string, messageId: string, content: string) => Promise<void>;
  addMessageLocally: (channelId: string, message: Message) => void;
  addReaction: (channelId: string, messageId: string, reaction: string, userId: string) => Promise<void>;
  removeReaction: (channelId: string, messageId: string, reaction: string, userId: string) => Promise<void>;
  hidePrivateChannel: (channelId: string) => Promise<void>;
  reset: () => void;
}

const t = i18n.t.bind(i18n);

export const usePrivateChannelStore = create<PrivateChannelState>((set, get) => ({
  privateChannels: [],
  currentPrivateChannel: null,
  messagesByChannel: {},
  isLoading: false,
  error: null,

  fetchPrivateChannels: async () => {
    set({ isLoading: true, error: null });
    try {
      const channels = await privateChannelsApi.getList();
      // Server returns channels ordered by last_message_at DESC — use that order directly
      set({ privateChannels: channels, isLoading: false });
      // Sync les IDs dans le WS store pour la détection dans NewMessage
      useWebSocketStore.setState(state => ({
        knownPrivateChannelIds: new Set([
          ...state.knownPrivateChannelIds,
          ...channels.map(ch => ch.id),
        ]),
      }));
    } catch (error) {
      set({ error: t('Use_private_channel_store.Error_loading_channels'), isLoading: false });
    }
  },

  createOrGetPrivateChannel: async (currentUserId: string, recipientId: string) => {
    try {
      // D'abord, vérifier si un channel existe déjà avec cet utilisateur
      const existingChannels = await privateChannelsApi.getList();
      let channelWithUser = existingChannels.find(
        (ch) =>
          (ch.user1 === currentUserId && ch.user2 === recipientId) ||
          (ch.user1 === recipientId && ch.user2 === currentUserId)
      ) ?? null;

      // Si aucun channel existant, en créer un nouveau
      if (!channelWithUser) {
        const newChannel = await privateChannelsApi.create(currentUserId, recipientId);
        channelWithUser = await privateChannelsApi.getById(newChannel.id);
      }

      if (!channelWithUser) {
        throw new Error('Channel not found');
      }

      // Mettre à jour la liste locale (ajouter ou remonter en tête)
      set((state) => {
        const existingIndex = state.privateChannels.findIndex(
          (ch) => ch.id === channelWithUser!.id
        );
        let updated: PrivateChannelWithUser[];
        if (existingIndex !== -1) {
          updated = [
            channelWithUser!,
            ...state.privateChannels.slice(0, existingIndex),
            ...state.privateChannels.slice(existingIndex + 1),
          ];
        } else {
          updated = [channelWithUser!, ...state.privateChannels];
        }
        return { privateChannels: updated, currentPrivateChannel: channelWithUser };
      });

      return channelWithUser;
    } catch (error) {
      console.error('Error in createOrGetPrivateChannel:', error);
      set({ error: t('Use_private_channel_store.Error_opening_conversation') });
      throw error;
    }
  },

  setCurrentPrivateChannel: (channel) => {
    set({ currentPrivateChannel: channel });
  },

  fetchMessages: async (channelId: string) => {
    set({ isLoading: true, error: null });
    try {
      const state = get();
      const cached = state.messagesByChannel[channelId];
      if (cached) {
        set({ isLoading: false });
        return;
      }

      const messages = await privateMessagesApi.getHistory(channelId);
      const sortedMessages = messages.sort(
        (a, b) =>
          new Date(a.created_at).getTime() - new Date(b.created_at).getTime()
      );

      set((state) => ({
        messagesByChannel: {
          ...state.messagesByChannel,
          [channelId]: sortedMessages,
        },
        isLoading: false,
      }));
    } catch (error) {
      set({ error: t('Use_private_channel_store.Error_loading_channels'), isLoading: false });
    }
  },

  sendMessage: async (channelId: string, content: string) => {
    try {
      const newMessage = await privateMessagesApi.send(channelId, content);
      
      set((state) => {
        // Add message to the channel
        const updated = {
          ...state.messagesByChannel,
          [channelId]: [...(state.messagesByChannel[channelId] || []), newMessage],
        };

        // Move channel to top
        const channel = state.privateChannels.find((ch) => ch.id === channelId);
        const sortedChannels = channel
          ? [channel, ...state.privateChannels.filter((ch) => ch.id !== channelId)]
          : state.privateChannels;

        return {
          messagesByChannel: updated,
          privateChannels: sortedChannels,
        };
      });
      // Re-sync channel order from server (which uses last_message_at DESC)
      get().fetchPrivateChannels();
    } catch (error) {
      set({ error: t('Use_private_channel_store.Error_sending_message') });
    }
  },

  deleteMessage: async (channelId: string, messageId: string) => {
    try {
      await privateMessagesApi.delete(messageId);
      set((state) => ({
        messagesByChannel: {
          ...state.messagesByChannel,
          [channelId]: (state.messagesByChannel[channelId] || []).filter(
            (m) => m.id !== messageId
          ),
        },
      }));
    } catch (error) {
      set({ error: t('Use_private_channel_store.Error_deleting_message') });
      throw error;
    }
  },

  updateMessage: async (channelId: string, messageId: string, content: string) => {
    try {
      const updatedMessage = await privateMessagesApi.update(messageId, content);
      set((state) => ({
        messagesByChannel: {
          ...state.messagesByChannel,
          [channelId]: (state.messagesByChannel[channelId] || []).map((m) =>
            m.id === messageId ? updatedMessage : m
          ),
        },
      }));
    } catch (error) {
      set({ error: t('Use_private_channel_store.Error_updating_message') });
      throw error;
    }
  },

  addMessageLocally: (channelId: string, message: Message) => {
    set((state) => ({
      messagesByChannel: {
        ...state.messagesByChannel,
        [channelId]: [...(state.messagesByChannel[channelId] || []), message],
      },
    }));
  },

  addReaction: async (channelId: string, messageId: string, reaction: string, userId: string) => {
    try {
      await privateMessagesApi.addReaction(messageId, reaction);
      set((state) => ({
        messagesByChannel: {
          ...state.messagesByChannel,
          [channelId]: (state.messagesByChannel[channelId] || []).map((m) => {
            if (m.id !== messageId) return m;
            const users = m.reactions?.[reaction] || [];
            if (users.includes(userId)) return m;
            return { ...m, reactions: { ...m.reactions, [reaction]: [...users, userId] } };
          }),
        },
      }));
    } catch (error) {
      console.error('Error adding reaction:', error);
    }
  },

  removeReaction: async (channelId: string, messageId: string, reaction: string, userId: string) => {
    try {
      await privateMessagesApi.removeReaction(messageId, reaction);
      set((state) => ({
        messagesByChannel: {
          ...state.messagesByChannel,
          [channelId]: (state.messagesByChannel[channelId] || []).map((m) => {
            if (m.id !== messageId) return m;
            const users = (m.reactions?.[reaction] || []).filter((id) => id !== userId);
            const updatedReactions = { ...m.reactions };
            if (users.length === 0) {
              delete updatedReactions[reaction];
            } else {
              updatedReactions[reaction] = users;
            }
            return { ...m, reactions: updatedReactions };
          }),
        },
      }));
    } catch (error) {
      console.error('Error removing reaction:', error);
    }
  },

  hidePrivateChannel: async (channelId: string) => {
    await privateChannelsApi.hide(channelId);
    set((state) => ({
      privateChannels: state.privateChannels.filter((ch) => ch.id !== channelId),
    }));
  },

  reset: () => {
    set({
      privateChannels: [],
      currentPrivateChannel: null,
      messagesByChannel: {},
      isLoading: false,
      error: null,
    });
  },
}));
