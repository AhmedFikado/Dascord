import { useEffect } from 'react';
import { useChannelStore } from '../stores/use-channel-store';

export function useChannels(serverId: string) {

    const channels = useChannelStore((state) => state.channelsByServer[serverId]);
    const isLoading = useChannelStore((state) => state.isLoading);
    const error = useChannelStore((state) => state.error);
    const fetchChannels = useChannelStore((state) => state.fetchChannels);

    useEffect(() => {
        if (serverId) {
            fetchChannels(serverId);
        }
    }, [serverId, fetchChannels]);

    return { channels, isLoading, error };
}