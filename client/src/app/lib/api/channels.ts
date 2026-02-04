import { Channel } from "@/types/models/channel";
import { apiClient } from './client';

export const channelsApi = {

    //  GET /servers/{serverId}/channels 
    getByServer: async (serverId: string): Promise<Channel[]> => {
        return await apiClient.get<Channel[]>(`servers/${serverId}/channels`)
    },

    // POST /servers/{serverId}/channels
    create: async (serverId: string, name: string): Promise<Channel> => {
        const newChannel: Partial<Channel> = {
            server_id: serverId,
            name: name,
            created_at: new Date()
        };
        return await apiClient.post<Channel>(`/servers/${serverId}/channels`, newChannel);
    },

    // DELETE /channels/{id}
    delete: async (channelId: string): Promise<void> => {
        return await apiClient.delete(`/channels/${channelId}`);
    },

    update: async (channelId: string, name: string): Promise<Channel> => {
        const updatedChannel: Partial<Channel> = {
            name: name
        };
        return await apiClient.put<Channel>(`/channels/${channelId}`, updatedChannel);
    }

};