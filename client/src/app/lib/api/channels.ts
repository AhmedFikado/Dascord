import { Channel } from "@/types/models/channel";

const mockChannels: Record<number, Channel[]> = {
    1: [
        { id: 1, server_id: 1, name: 'Général', created_at: new Date() },
        { id: 2, server_id: 1, name: 'information', created_at: new Date() },
        { id: 3, server_id: 1, name: 'Invites', created_at: new Date() },
    ],
    2: [
        { id: 4, server_id: 2, name: 'general', created_at: new Date() },
        { id: 5, server_id: 2, name: 'announcements', created_at: new Date() },
    ],
    3: [
        { id: 6, server_id: 3, name: 'dev-chat', created_at: new Date() },
        { id: 7, server_id: 3, name: 'code-review', created_at: new Date() },
    ],
};

export const channelsApi = {

    //  GET /servers/{serverId}/channels 
    getByServer: async (serverId: number) => {
        return mockChannels[serverId] || [];
    },

    // POST /servers/{serverId}/channels
    create: async (serverId: number, name: string): Promise<Channel> => {
        const newChannel: Channel = {
            id: Date.now(),
            server_id: serverId,
            name: name,
            created_at: new Date()
        };

        if (!mockChannels[serverId]) {
            mockChannels[serverId] = [];
        }
        mockChannels[serverId].push(newChannel);
        return newChannel;
    },

    // DELETE /channels/{id}
    delete: async (serverId: number, channelId: number) => {
        mockChannels[serverId] = mockChannels[serverId].filter(channel => channel.id !== channelId);
    },

};