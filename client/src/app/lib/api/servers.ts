import { Server } from "@/types/models/Server";
import { Member } from "@/types/models/member";
import { BannedMember } from "@/types/models/BannedMember";
import { apiClient } from './client';
import { Role } from "@/types/models/role";

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
    },

    // GET /servers/{id}/members
    getMembers: async (serverId: string): Promise<Member[]> => {
        return await apiClient.get<Member[]>(`/servers/${serverId}/members`);
    },

    // PUT /servers/{id}/members/:userId
    updateRoleMember: async (serverId: string, userId: string, role: Role): Promise<void> => {
        await apiClient.put(`/servers/${serverId}/members/${userId}`, { role });
    },

    // DELETE /servers/{id}/members/:userId/kick
    kickMember: async (serverId: string, userId: string): Promise<void> => {
        await apiClient.delete(`/servers/${serverId}/members/${userId}/kick`);
    },

    // POST /servers/{id}/members/:userId/ban
    banMember: async (serverId: string, userId: string, banType: 'Permanent' | 'Temporary', expiresAt?: string): Promise<void> => {
        await apiClient.post(`/servers/${serverId}/members/${userId}/ban`, {
            ban_type: banType,
            expires_at: expiresAt ?? null,
        });
    },

    // GET /servers/{id}/bans
    getBannedMembers: async (serverId: string): Promise<BannedMember[]> => {
        return await apiClient.get<BannedMember[]>(`/servers/${serverId}/bans`);
    },

    // DELETE /servers/{id}/members/:userId/ban
    unbanMember: async (serverId: string, userId: string): Promise<void> => {
        await apiClient.delete(`/servers/${serverId}/members/${userId}/ban`);
    },
};