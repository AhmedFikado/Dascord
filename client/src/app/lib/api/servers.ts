import { Server } from "@/types/models/Server";
import { apiClient } from './client';

export const serversApi = {

    // GET /servers
    getAll: async (): Promise<Server[]> => {
        return await apiClient.get<Server[]>('/servers');
    },

    // GET /server/{id}
    getOne: async (serverId: string): Promise<Server | undefined> => {
        return await apiClient.get<Server>(`/servers/${serverId}`);
    },

    // POST /servers
    create: async (name: string): Promise<Server> => {
        const newServer: Partial<Server> = {
            name: name,
            created_at: new Date()
        };
        return await apiClient.post<Server>('/servers', newServer);
    },

    // POST /servers/join
    joinServer: async (invitationCode: string): Promise<void> => {
        await apiClient.post('/servers/join', { invitation_code: invitationCode });
    },

    // PUT /servers/{id}
    updateServer: async (ServerId: string, name: string): Promise<Server> => {
        const updatedServer: Partial<Server> = {
            name: name
        };
        return await apiClient.put<Server>(`/servers/${ServerId}`, updatedServer);
    },

    // DELETE /servers/{id}
    delete: async (serverId: string): Promise<void> => {
        await apiClient.delete(`/servers/${serverId}`);
    },

    // DELETE /servers/{id}/leave
    leave: async (serverId: string): Promise<void> => {
        await apiClient.delete(`/servers/${serverId}/leave`);
    }
};
