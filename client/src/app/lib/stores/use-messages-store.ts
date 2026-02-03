import { create } from 'zustand';
import { Message } from '@/types/models/message';
import { messagesApi } from '../api/messages';
import { useAuthStore } from './use-auth-store';
import { User } from '@/types/models/user';

interface MessageState {
    messages: Message[];
    isLoading: boolean;
    error: string | null;

    fetchMessages: (channelId: string) => Promise<void>;
    sendMessage: (channelId: string, content: string, user?: User) => Promise<void>;
    deleteMessage: (channelId: string, messageId: string) => Promise<void>;
}

export const useMessageStore = create<MessageState>((set) => ({
    messages: [],
    isLoading: false,
    error: null,

    fetchMessages: async (channelId: string) => {
        set({ isLoading: true, error: null, messages: [] });

        try {
            const messages = await messagesApi.getMessageHistory(channelId);
            set({ messages, isLoading: false });
        } catch (error) {
            set({ error: 'Impossible de charger les messages', isLoading: false });
        }
    },

    sendMessage: async (channelId: string, content: string) => {
        try {
            const newMessage = await messagesApi.send(channelId, content, useAuthStore.getState().user ?? undefined);
            set((state) => ({
                messages: [...state.messages, newMessage]
            }));
        } catch (error) {
            console.error("Erreur d'envoi", error);
        }
    },

    deleteMessage: async (channelId: string, messageId: string) => {
        try {
            set((state) => ({
                messages: state.messages.filter((m) => m.id !== String(messageId)),
            }));
            await messagesApi.delete(channelId, messageId);
        } catch (error) {
            console.error("Erreur de suppression", error);
        }
    },
}));