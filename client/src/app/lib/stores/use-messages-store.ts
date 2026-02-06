import { useWebSocketStore } from '@/store/websocket';
import { Message } from '@/types/models/message';
import { create } from 'zustand';
import { messagesApi } from '../api/messages';

interface MessageState {
  messages: Message[];
  messagesByChannel: Record<string, Message[]>;
  isLoading: boolean;
  error: string | null;

  fetchMessages: (channelId: string) => Promise<void>;
  sendMessage: (channelId: string, content: string) => Promise<void>;
  deleteMessage: (channelId: string, messageId: string) => Promise<void>;
  reset: () => void;
}

export const useMessageStore = create<MessageState>(set => ({
  messages: [],
  messagesByChannel: {},
  isLoading: false,
  error: null,

  fetchMessages: async (channelId: string) => {
    set({ isLoading: true, error: null });

    try {
      const state = useMessageStore.getState();
      const cached = state.messagesByChannel[channelId];
      if (cached) {
        set({ messages: cached, isLoading: false });
        return;
      }
      const messages = await messagesApi.getMessageHistory(channelId);

      const sortedMessages = messages.sort(
        (a, b) => new Date(a.created_at).getTime() - new Date(b.created_at).getTime()
      );

      set(state => ({
        messages: sortedMessages,
        messagesByChannel: {
          ...state.messagesByChannel,
          [channelId]: sortedMessages,
        },
        isLoading: false,
      }));
    } catch (error) {
      set({ error: 'Impossible de charger les messages', isLoading: false });
    }
  },

  sendMessage: async (channelId: string, content: string) => {
    try {
      const newMessage = await messagesApi.send(channelId, content);
      set(state => ({
        messages: [...state.messages, newMessage],
        messagesByChannel: {
          ...state.messagesByChannel,
          [channelId]: [...(state.messagesByChannel[channelId] || []), newMessage],
        },
      }));
    } catch (error) {
      console.error("Erreur d'envoi", error);
    }
  },

  deleteMessage: async (channelId: string, messageId: string) => {
    try {
      await messagesApi.delete(messageId);
      set(state => ({
        messages: state.messages.filter(m => m.id !== messageId),
        messagesByChannel: {
          ...state.messagesByChannel,
          [channelId]: (state.messagesByChannel[channelId] || []).filter(m => m.id !== messageId),
        },
        error: null,
      }));
      // Supprimer aussi du store WebSocket
      useWebSocketStore.getState().removeMessage(channelId, messageId);
    } catch (error: any) {
      console.error('Erreur de suppression', error);
      const errorMessage =
        error?.response?.data?.error || "Vous n'avez pas la permission de supprimer ce message";
      set({ error: errorMessage });
      throw error;
    }
  },

  reset: () => set({ messages: [], messagesByChannel: {}, isLoading: false, error: null }),
}));
