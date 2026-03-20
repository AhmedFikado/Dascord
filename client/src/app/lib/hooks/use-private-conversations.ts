import { useEffect } from 'react';
import { usePrivateChannelStore } from '../stores/use-private-channel-store';

export function usePrivateConversations() {
  const { privateChannels, isLoading, error, fetchPrivateChannels } = usePrivateChannelStore();

  useEffect(() => {
    fetchPrivateChannels();
  }, [fetchPrivateChannels]);

  return {
    conversations: privateChannels,
    isLoading,
    error,
  };
}
