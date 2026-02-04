import { create } from 'zustand';
import { Server } from '@/types/models/Server';
import { serversApi } from '../api/servers';

interface ServerState {
    servers: Server[];
    currentServer: Server | null;
    isLoading: boolean;
    error: string | null;

    fetchServers: () => Promise<void>;
    setCurrentServer: (server: Server) => void;
    createServer: (name: string) => Promise<Server>;
    deleteServer: (serverId: string) => Promise<void>;
    joinServer: (invitationCode: string) => Promise<void>;
    leaveServer: (serverId: string) => Promise<void>;
    updateServer: (serverId: string, name: string) => Promise<Server>;
}

export const useServerStore = create<ServerState>((set) => ({
    servers: [],
    currentServer: null,
    isLoading: false,
    error: null,

    fetchServers: async () => {
        set({ isLoading: true, error: null });
        try {
            const servers = await serversApi.getAll();
            console.log(servers)
            set({ servers, isLoading: false });
        } catch (error) {
            set({ error: 'Erreur lors du chargement des serveurs', isLoading: false });
        }
    },

    setCurrentServer: (server) => set({ currentServer: server }),

    createServer: async (name) => {
        set({ isLoading: true, error: null });
        try {
            const newServer = await serversApi.create(name);
            set((state) => ({
                servers: [...state.servers, newServer],
                isLoading: false,
            }));
            return newServer;
        } catch (error) {
            set({ error: 'Erreur lors de la création du serveur', isLoading: false });
            throw error;
        }
    },

    deleteServer: async (serverId) => {
        set({ isLoading: true, error: null });
        try {
            await serversApi.delete(serverId);
            set((state) => ({
                servers: state.servers.filter((s) => s.id !== serverId),
                isLoading: false,
            }));
        } catch (error) {
            set({ error: 'Erreur lors de la suppression du serveur', isLoading: false });
            throw error;
        }
    },

    joinServer: async (invitationCode) => {
        set({ isLoading: true, error: null });
        try {
            await serversApi.joinServer(invitationCode);
            const servers = await serversApi.getAll();
            set({ servers, isLoading: false });
        } catch (error) {
            set({ error: 'Erreur lors de la connexion au serveur', isLoading: false });
            throw error;
        }
    },

    leaveServer: async (serverId) => {
        set({ isLoading: true, error: null });
        try {
            await serversApi.leave(serverId);
            const servers = await serversApi.getAll();
            set({ servers, isLoading: false });
        } catch (error) {
            set({ error: 'Erreur lors du départ du serveur', isLoading: false });
            throw error;
        }
    },

    updateServer: async (serverId, name) => {
        set({ isLoading: true, error: null });
        try {
            const updatedServer = await serversApi.updateServer(serverId, name);
            set((state) => ({
                servers: state.servers.map((s) => s.id === serverId ? updatedServer : s),
                isLoading: false,
            }));
            return updatedServer;
        } catch (error) {
            set({ error: 'Erreur lors de la mise à jour du serveur', isLoading: false });
            throw error;
        }
    }

}));