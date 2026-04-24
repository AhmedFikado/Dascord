import { create } from 'zustand';
import { markChannelAsRead, getUnreadChannelsForServer, getUnreadPrivateChannels } from '../api/read-status';
import { useMessageStore } from './use-messages-store';
import { usePrivateChannelStore } from './use-private-channel-store';

interface UnreadState {
  // channelId -> firstUnreadMessageId
  unreadChannels: Record<string, string>;
  // serverId -> hasUnread (only non-private servers)
  unreadServers: Set<string>;
  // channelId -> serverId (for reverse lookup when marking read)
  channelToServer: Record<string, string>;

  markChannelRead: (channelId: string) => void;
  markChannelReadRemote: (channelId: string) => Promise<void>;
  addUnread: (serverId: string, channelId: string, firstMessageId: string) => void;
  fetchUnreadForServer: (serverId: string) => Promise<void>;
  fetchUnreadPrivate: () => Promise<void>;
  isChannelUnread: (channelId: string) => boolean;
  isServerUnread: (serverId: string) => boolean;
  getFirstUnreadMessageId: (channelId: string) => string | undefined;
  reset: () => void;
}

export const useUnreadStore = create<UnreadState>((set, get) => ({
  unreadChannels: {},
  unreadServers: new Set<string>(),
  channelToServer: {},

  markChannelRead: (channelId: string) => {
    set((state) => {
      const updatedChannels = { ...state.unreadChannels };
      delete updatedChannels[channelId];

      const serverId = state.channelToServer[channelId];
      const updatedServers = new Set(state.unreadServers);
      if (serverId && serverId !== 'private') {
        const serverStillHasUnread = Object.entries(updatedChannels).some(
          ([cid]) => state.channelToServer[cid] === serverId
        );
        if (!serverStillHasUnread) {
          updatedServers.delete(serverId);
        }
      }

      const updatedMapping = { ...state.channelToServer };
      delete updatedMapping[channelId];

      return { unreadChannels: updatedChannels, unreadServers: updatedServers, channelToServer: updatedMapping };
    });
  },

  markChannelReadRemote: async (channelId: string) => {
    try {
      await markChannelAsRead(channelId);
    } catch {
      // silently ignore – the local state is already cleared
    }
    get().markChannelRead(channelId);
  },

  addUnread: (serverId: string, channelId: string, firstMessageId: string) => {
    set((state) => {
      const updatedChannels = { ...state.unreadChannels, [channelId]: firstMessageId };
      const updatedMapping = { ...state.channelToServer, [channelId]: serverId };
      const updatedServers = new Set(state.unreadServers);
      if (serverId !== 'private') {
        updatedServers.add(serverId);
      }
      return { unreadChannels: updatedChannels, unreadServers: updatedServers, channelToServer: updatedMapping };
    });
    // Invalidate the message cache so the next visit fetches fresh messages
    // including the new unread message
    if (serverId === 'private') {
      usePrivateChannelStore.getState().invalidateChannel(channelId);
    } else {
      useMessageStore.getState().invalidateChannel(channelId);
    }
  },

  fetchUnreadForServer: async (serverId: string) => {
    try {
      const data = await getUnreadChannelsForServer(serverId);
      set((state) => {
        const updatedChannels = { ...state.unreadChannels };
        const updatedMapping = { ...state.channelToServer };
        data.unread_channels.forEach(({ channel_id, first_unread_message_id }) => {
          updatedChannels[channel_id] = first_unread_message_id;
          updatedMapping[channel_id] = serverId;
        });
        const updatedServers = new Set(state.unreadServers);
        if (data.unread_channels.length > 0) {
          updatedServers.add(serverId);
        } else {
          updatedServers.delete(serverId);
        }
        return { unreadChannels: updatedChannels, unreadServers: updatedServers, channelToServer: updatedMapping };
      });
    } catch {
      // ignore fetch errors silently
    }
  },

  fetchUnreadPrivate: async () => {
    try {
      const data = await getUnreadPrivateChannels();
      set((state) => {
        const updatedChannels = { ...state.unreadChannels };
        const updatedMapping = { ...state.channelToServer };
        data.unread_channels.forEach(({ channel_id, first_unread_message_id }) => {
          updatedChannels[channel_id] = first_unread_message_id;
          updatedMapping[channel_id] = 'private';
        });
        return { unreadChannels: updatedChannels, channelToServer: updatedMapping };
      });
    } catch {
      // ignore fetch errors silently
    }
  },

  isChannelUnread: (channelId: string) => {
    return channelId in get().unreadChannels;
  },

  isServerUnread: (serverId: string) => {
    return get().unreadServers.has(serverId);
  },

  getFirstUnreadMessageId: (channelId: string) => {
    return get().unreadChannels[channelId];
  },

  reset: () => set({ unreadChannels: {}, unreadServers: new Set<string>(), channelToServer: {} }),
}));
