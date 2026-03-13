import { create } from 'zustand';
import { Channel } from '@/types/models/channel';
import { channelsApi } from '../api/channels';
import { useTranslation } from 'react-i18next';
import i18n from 'i18next';

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
    reset: () => void;
}

const t = i18n.t.bind(i18n);

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
            set({ error: t('Use_channel_store.Error_loading_channels'), isLoading: false });
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
            set({ error: t('Use_channel_store.Error_adding_channel'), isLoading: false });
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
            set({ error: t('Use_channel_store.Error_deleting_channel'), isLoading: false });
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
            set({ error: t('Use_channel_store.Error_updating_channel'), isLoading: false });
            throw error;
        }
    },

    reset: () => set({ channels: [], currentChannel: null, channelsByServer: {}, isLoading: false, error: null }),
}));
