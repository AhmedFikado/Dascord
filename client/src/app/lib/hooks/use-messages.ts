import { useEffect } from 'react';
import { useMessageStore } from '../stores/use-messages-store';
import { Message } from '@/types/models/message';

const EMPTY_MESSAGES: Message[] = [];

export function useMessages(channelId: string) {
    const messages = useMessageStore((state) => state.messagesByChannel[channelId] ?? EMPTY_MESSAGES);
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
