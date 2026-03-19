import { Member } from '@/types/models/member';
import { Role } from '@/types/models/role';
import { Server } from '@/types/models/Server';
import { create } from 'zustand';
import { channelsApi } from '../api/channels';
import { messagesApi } from '../api/messages';
import { serversApi } from '../api/servers';
import { useMessageStore } from './use-messages-store';
import { useTranslation } from 'react-i18next';
import i18n from 'i18next';

interface ServerState {
  servers: Server[];
  currentServer: Server | null;
  members: Member[];
  isLoading: boolean;
  error: string | null;

  fetchServers: () => Promise<void>;
  addServer: (server: Server) => void;
  setCurrentServer: (server: Server | null) => void;
  createServer: (name: string) => Promise<Server>;
  deleteServer: (serverId: string) => Promise<void>;
  joinServer: (invitationCode: string) => Promise<void>;
  leaveServer: (serverId: string) => Promise<void>;
  updateServer: (serverId: string, name: string) => Promise<Server>;
  getMembers: (serverId: string) => Promise<void>;
  updateRoleMember: (serverId: string, userId: string, role: Role) => Promise<void>;
  kickMember: (serverId: string, userId: string) => Promise<void>;
  banMember: (serverId: string, userId: string, banType: 'Permanent' | 'Temporary', expiresAt?: string) => Promise<void>;
  reset: () => void;
}

const t = i18n.t.bind(i18n);

export const useServerStore = create<ServerState>(set => ({
  servers: [],
  currentServer: null,
  members: [],
  isLoading: false,
  error: null,

  fetchServers: async () => {
    set({ isLoading: true, error: null });
    try {
      const servers = await serversApi.getAll();
      set({ servers, isLoading: false });
    } catch (error) {
      set({ error: t('Use_server_store.Error_loading_servers'), isLoading: false });
    }
  },

  addServer: server => set(state => ({ servers: [...state.servers, server] })),

  setCurrentServer: server => set({ currentServer: server }),

  createServer: async name => {
    set({ isLoading: true, error: null });
    try {
      const newServer = await serversApi.create(name);
      set(state => ({
        servers: [...state.servers, newServer],
        isLoading: false,
      }));
      return newServer;
    } catch (error) {
      set({ error: t('Use_server_store.Error_create_server'), isLoading: false });
      throw error;
    }
  },

  deleteServer: async serverId => {
    set({ isLoading: true, error: null });
    try {
      await serversApi.delete(serverId);
      set(state => ({
        servers: state.servers.filter(s => s.id !== serverId),
        isLoading: false,
      }));
    } catch (error) {
      set({ error: t('Use_server_store.Error_deleting_server'), isLoading: false });
      throw error;
    }
  },

  joinServer: async invitationCode => {
    set({ isLoading: true, error: null });
    try {
      await serversApi.joinServer(invitationCode);
      const servers = await serversApi.getAll();

      // Trouver le serveur qu'on vient de rejoindre
      const joinedServer = servers[servers.length - 1];
      if (joinedServer) {
        const channels = await channelsApi.getByServer(joinedServer.id);
        if (channels.length > 0) {
          // Envoyer le message de bienvenue
          await messagesApi.sendWelcome(channels[0].id);
          // Invalider le cache des messages pour forcer le rechargement
          useMessageStore.getState().reset();
        }
      }

      set({ servers, isLoading: false });
    } catch (error) {
      set({ error: t('Use_server_store.Error_joining_server'), isLoading: false });
      throw error;
    }
  },

  leaveServer: async serverId => {
    set({ isLoading: true, error: null });
    try {
      await serversApi.leave(serverId);
      const servers = await serversApi.getAll();
      set({ servers, isLoading: false });
    } catch (error) {
      set({ error: t('Use_server_store.Error_leaving_server'), isLoading: false });
      throw error;
    }
  },

  updateServer: async (serverId, name) => {
    set({ isLoading: true, error: null });
    try {
      const updatedServer = await serversApi.updateServer(serverId, name);
      set(state => ({
        servers: state.servers.map(s => (s.id === serverId ? updatedServer : s)),
        isLoading: false,
      }));
      return updatedServer;
    } catch (error) {
      set({ error: t('Use_server_store.Error_updating_server'), isLoading: false });
      throw error;
    }
  },

  getMembers: async serverId => {
    set({ isLoading: true, error: null });
    try {
      const members = await serversApi.getMembers(serverId);
      set({ members, isLoading: false });
    } catch (error) {
      set({ error: t('Use_server_store.Error_loading_members'), isLoading: false });
    }
  },

  updateRoleMember: async (serverId, userId, role) => {
    set({ isLoading: true, error: null });
    try {
      await serversApi.updateRoleMember(serverId, userId, role);
      set({ isLoading: false });
    } catch (error) {
      set({ error: t('Use_server_store.Error_updating_member_role'), isLoading: false });
      throw error;
    }
  },

  kickMember: async (serverId, userId) => {
    set({ isLoading: true, error: null });
    try {
      await serversApi.kickMember(serverId, userId);
      set(state => ({
        members: state.members.filter(m => m.user_id !== userId),
        isLoading: false,
      }));
    } catch (error) {
      set({ error: t('Use_server_store.Error_kicking_member'), isLoading: false });
      throw error;
    }
  },

  banMember: async (serverId, userId, banType, expiresAt) => {
    set({ isLoading: true, error: null });
    try {
      await serversApi.banMember(serverId, userId, banType, expiresAt);
      set(state => ({
        members: state.members.filter(m => m.user_id !== userId),
        isLoading: false,
      }));
    } catch (error) {
      set({ error: t('Use_server_store.Error_banning_member'), isLoading: false });
      throw error;
    }
  },

  reset: () =>
    set({ servers: [], currentServer: null, members: [], isLoading: false, error: null }),
}));
