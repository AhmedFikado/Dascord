import { useEffect } from 'react';
import { useMessageStore } from '../stores/use-messages-store';

export function useMessages(channelId: string) {
    const messages = useMessageStore((state) => state.messages);
    const isLoading = useMessageStore((state) => state.isLoading);
    const sendMessage = useMessageStore((state) => state.sendMessage);
    const deleteMessage = useMessageStore((state) => state.deleteMessage);
    const updateMessage = useMessageStore((state) => state.updateMessage);
    const fetchMessages = useMessageStore((state) => state.fetchMessages);

    useEffect(() => {
        if (channelId) {
            fetchMessages(channelId);
        }
    }, [channelId, fetchMessages]);

    return { messages, isLoading, sendMessage, deleteMessage, updateMessage };
}
