import { Message } from '@/types/models/message';
import { apiClient } from './client';

export const messagesApi = {
  // GET /channels/{id}/messages
  getMessageHistory: async (channelId: string): Promise<Message[]> => {
    return await apiClient.get<Message[]>(`/channels/${channelId}/messages`);
  },

  // POST /channels/{id}/messages
  send: async (channelId: string, content: string): Promise<Message> => {
    return await apiClient.post<Message>(`/channels/${channelId}/messages`, { content });
  },

  // POST /channels/{id}/messages/welcome
  sendWelcome: async (channelId: string): Promise<Message> => {
    return await apiClient.post<Message>(`/channels/${channelId}/messages/welcome`);
  },

  // DELETE /messages/{id}
  delete: async (messageId: string): Promise<void> => {
    await apiClient.delete(`/messages/${messageId}`);
  },

  // PUT /messages/{id}
  update: async (messageId: string, content: string): Promise<Message> => {
    return await apiClient.put<Message>(`/messages/${messageId}`, { content });
  },

  // POST /messages/{id}/reactions
  addReaction: async (messageId: string, reaction: string): Promise<void> => {
    await apiClient.post(`/messages/${messageId}/reactions`, { reaction });
  },

  // DELETE /messages/{id}/reactions
  removeReaction: async (messageId: string, emoji: string): Promise<void> => {
    await apiClient.delete(`/messages/${messageId}/reactions/${encodeURIComponent(emoji)}`);
  },
};
