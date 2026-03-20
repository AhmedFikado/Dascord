import { Message } from '@/types/models/message';
import { apiClient } from './client';

export const privateMessagesApi = {
  // GET /channels/:channel_id/messages/private - Get private message history
  getHistory: async (channelId: string): Promise<Message[]> => {
    return await apiClient.get<Message[]>(`/channels/${channelId}/messages/private`);
  },

  // POST /channels/:channel_id/messages/private - Send private message
  send: async (channelId: string, content: string): Promise<Message> => {
    return await apiClient.post<Message>(`/channels/${channelId}/messages/private`, {
      content,
    });
  },

  // DELETE /messages/private/:id - Delete private message
  delete: async (messageId: string): Promise<void> => {
    await apiClient.delete(`/messages/private/${messageId}`);
  },

  // PUT /messages/private/:id - Update private message
  update: async (messageId: string, content: string): Promise<Message> => {
    return await apiClient.put<Message>(`/messages/private/${messageId}`, {
      content,
    });
  },

  // POST /messages/private/:id/reactions
  addReaction: async (messageId: string, reaction: string): Promise<void> => {
    await apiClient.post(`/messages/private/${messageId}/reactions`, { reaction });
  },

  // DELETE /messages/private/:id/reactions/:reaction
  removeReaction: async (messageId: string, emoji: string): Promise<void> => {
    await apiClient.delete(`/messages/private/${messageId}/reactions/${encodeURIComponent(emoji)}`);
  },
};
