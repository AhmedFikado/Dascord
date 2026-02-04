import { create } from 'zustand';
import { Message } from '@/types/models/message';
import { messagesApi } from '../api/messages';

interface MessageState {
    messages: Message[];
    messagesByChannel: Record<string, Message[]>;
    isLoading: boolean;
    error: string | null;

    fetchMessages: (channelId: string) => Promise<void>;
    sendMessage: (channelId: string, content: string) => Promise<void>;
    deleteMessage: (channelId: string, messageId: string) => Promise<void>;
}

export const useMessageStore = create<MessageState>((set) => ({
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
            
            const sortedMessages = messages.sort((a, b) => 
                new Date(a.created_at).getTime() - new Date(b.created_at).getTime()
            );
            
            set((state) => ({
                messages: sortedMessages,
                messagesByChannel: {
                    ...state.messagesByChannel,
                    [channelId]: sortedMessages
                },
                isLoading: false
            }));
        } catch (error) {
            set({ error: 'Impossible de charger les messages', isLoading: false });
        }
    },

    sendMessage: async (channelId: string, content: string) => {
        try {
            const newMessage = await messagesApi.send(channelId, content);
            set((state) => ({
                messages: [...state.messages, newMessage],
                messagesByChannel: {
                    ...state.messagesByChannel,
                    [channelId]: [...(state.messagesByChannel[channelId] || []), newMessage]
                }
            }));
        } catch (error) {
            console.error("Erreur d'envoi", error);
        }
    },

    deleteMessage: async (channelId: string, messageId: string) => {
        try {
            await messagesApi.delete(channelId, messageId);
            set((state) => ({
                messages: state.messages.filter((m) => m.id !== messageId),
                messagesByChannel: {
                    ...state.messagesByChannel,
                    [channelId]: (state.messagesByChannel[channelId] || []).filter((m) => m.id !== messageId)
                }
            }));
        } catch (error) {
            console.error("Erreur de suppression", error);
        }
    },
}));