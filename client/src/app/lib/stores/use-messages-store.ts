import { create } from 'zustand';
import { Message } from '@/types/models/message';
import { messagesApi } from '../api/messages';

interface MessageState {
    messages: Message[];
    isLoading: boolean;
    error: string | null;

    fetchMessages: (channelId: number) => Promise<void>;
    sendMessage: (channelId: number, content: string) => Promise<void>;
    deleteMessage: (channelId: number, messageId: number) => Promise<void>;
}

export const useMessageStore = create<MessageState>((set) => ({
    messages: [],
    isLoading: false,
    error: null,

    fetchMessages: async (channelId: number) => {
        set({ isLoading: true, error: null, messages: [] });

        try {
            const messages = await messagesApi.getMessageHistory(channelId);
            set({ messages, isLoading: false });
        } catch (error) {
            set({ error: 'Impossible de charger les messages', isLoading: false });
        }
    },

    sendMessage: async (channelId: number, content: string) => {
        try {
            const newMessage = await messagesApi.send(channelId, content);
            set((state) => ({
                messages: [...state.messages, newMessage]
            }));
        } catch (error) {
            console.error("Erreur d'envoi", error);
        }
    },

    deleteMessage: async (channelId: number, messageId: number) => {
        try {
            set((state) => ({
                messages: state.messages.filter((m) => m.id !== messageId),
            }));
            await messagesApi.delete(channelId, messageId);
        } catch (error) {
            console.error("Erreur de suppression", error);
        }
    },
}));