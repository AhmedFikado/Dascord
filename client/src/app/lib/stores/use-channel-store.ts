import { create } from 'zustand';
import { Channel } from '@/types/models/channel';
import { channelsApi } from '../api/channels';

interface ChannelState {
    channels: Channel[];
    currentChannel: Channel | null;
    isLoading: boolean;
    error: string | null;
    channelsByServer: Record<string, Channel[]>;

    fetchChannels: (serverId: string) => Promise<void>;
    setCurrentChannel: (channel: Channel) => void;
    addChannel: (serverId: string, channelName: string) => Promise<Channel>;
    removeChannel: (channelId: string) => Promise<void>;
    updateChannel: (channelId: string, name: string) => Promise<Channel>;
}

export const useChannelStore = create<ChannelState>((set) => ({
    channels: [],
    currentChannel: null,
    isLoading: false,
    error: null,
    channelsByServer: {},

    fetchChannels: async (serverId: string) => {
        set({ isLoading: true, error: null });
        try {
            const state = useChannelStore.getState();
            const cached = state.channelsByServer[serverId];
            if (cached) {
                set({ channels: cached, isLoading: false });
                return;
            }
            const channels = await channelsApi.getByServer(serverId);
            set((state) => ({
                channels,
                channelsByServer: {
                    ...state.channelsByServer,
                    [serverId]: channels
                },
                isLoading: false
            }));
        } catch (error) {
            set({ error: 'Erreur lors du chargement des channels', isLoading: false });
        }
    },

    setCurrentChannel: (channel) => set({ currentChannel: channel }),

    addChannel: async (serverId, channelName) => {
        set({ isLoading: true, error: null });
        try {
            const addChannel = await channelsApi.create(serverId, channelName);
            set((state) => {
                const updatedChannelsByServer = { ...state.channelsByServer };
                if (updatedChannelsByServer[serverId]) {
                    updatedChannelsByServer[serverId] = [...updatedChannelsByServer[serverId], addChannel];
                }

                return {
                    channels: [...state.channels, addChannel],
                    channelsByServer: updatedChannelsByServer,
                    isLoading: false
                };
            });
            return addChannel;
        } catch (error) {
            set({ error: 'Erreur lors de l\'ajout du channel', isLoading: false });
            throw error;
        }
    },

    removeChannel: async (channelId) => {
        set({ isLoading: true, error: null });
        try {
            await channelsApi.delete(channelId);
            set((state) => {
                const updatedChannelsByServer = { ...state.channelsByServer };
                Object.keys(updatedChannelsByServer).forEach((serverId) => {
                    updatedChannelsByServer[serverId] = updatedChannelsByServer[serverId].filter((c) => c.id !== channelId);
                });

                return {
                    channels: state.channels.filter((c) => c.id !== channelId),
                    channelsByServer: updatedChannelsByServer,
                    isLoading: false
                };
            });
        } catch (error) {
            set({ error: 'Erreur lors de la suppression du channel', isLoading: false });
        }
    },

    updateChannel: async (channelId, name) => {
        set({ isLoading: true, error: null });
        try {
            const updatedChannel = await channelsApi.update(channelId, name);
            set((state) => {

                const updatedChannelsByServer = { ...state.channelsByServer };
                Object.keys(updatedChannelsByServer).forEach((serverId) => {
                    updatedChannelsByServer[serverId] = updatedChannelsByServer[serverId].map((c) =>
                        c.id === channelId ? updatedChannel : c
                    );
                });

                return {
                    channels: state.channels.map((c) => c.id === channelId ? updatedChannel : c),
                    channelsByServer: updatedChannelsByServer,
                    isLoading: false
                };
            });
            return updatedChannel;
        } catch (error) {
            set({ error: 'Erreur lors de la mise à jour du channel', isLoading: false });
            throw error;
        }
    },

}));