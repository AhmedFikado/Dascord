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
    addServer: (server: Server) => void;
    removeServer: (serverId: number) => void;
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

    addServer: (server) => set((state) => ({ servers: [...state.servers, server] })),

    removeServer: (serverId) =>
        set((state) => ({
            servers: state.servers.filter((s) => s.id !== serverId),
        })),
}));