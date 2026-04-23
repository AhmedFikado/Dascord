import { apiClient } from './client';
import type { PrivateChannel, PrivateChannelWithUser } from '../../../types/models/privateChannels';


export const privateChannelsApi = {
  // POST /channels/private - Create a new private channel
  create: async (user1Id: string, user2Id: string): Promise<PrivateChannel> => {
    console.log('Creating private channel with user1:', user1Id, 'user2:', user2Id);
    try {
      const payload = { user1: user1Id, user2: user2Id };
      console.log('Payload:', JSON.stringify(payload));
      return await apiClient.post<PrivateChannel>('/channels/private', payload);
    } catch (error: any) {
      console.error('Error creating private channel:', error.response?.data || error.message);
      throw error;
    }
  },

  // GET /channels/private - Get list of all private channels for current user (from JWT)
  getList: async (): Promise<PrivateChannelWithUser[]> => {
    console.log('Fetching private channels');
    return await apiClient.get<PrivateChannelWithUser[]>('/channels/private');
  },

  // GET /channels/{id}/private - Get specific private channel
  getById: async (channelId: string): Promise<PrivateChannelWithUser> => {
    console.log('Fetching private channel:', channelId);
    return await apiClient.get<PrivateChannelWithUser>(`/channels/${channelId}/private`);
  },

  // PATCH /channels/private/{id}/hide - Hide a private channel for the current user
  hide: async (channelId: string): Promise<void> => {
    await apiClient.patch<void>(`/channels/private/${channelId}/hide`, {});
  },
};
