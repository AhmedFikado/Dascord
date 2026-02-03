import { create } from 'zustand';
import { Channel } from '@/types/models/channel';
import { channelsApi } from '../api/channels';

interface ChannelState {
    channels: Channel[];
    currentChannel: Channel | null;
    isLoading: boolean;
    error: string | null;

    fetchChannels: (serverId: string) => Promise<void>;
    setCurrentChannel: (channel: Channel) => void;
    addChannel: (channel: Channel) => void;
    removeChannel: (channelId: string) => void;
}

export const useChannelStore = create<ChannelState>((set) => ({
    channels: [],
    currentChannel: null,
    isLoading: false,
    error: null,

    fetchChannels: async (serverId: string) => {
        set({ isLoading: true, error: null });
        try {
            const channels = await channelsApi.getByServer(serverId);
            set({ channels, isLoading: false });
        } catch (error) {
            set({ error: 'Erreur lors du chargement des channels', isLoading: false });
        }
    },

    setCurrentChannel: (channel) => set({ currentChannel: channel }),

    addChannel: (channel) => set((state) => ({ channels: [...state.channels, channel] })),

    removeChannel: (channelId) =>
        set((state) => ({
            channels: state.channels.filter((c) => c.id !== channelId),
        })),
}));