import { useEffect } from 'react';
import { useServerStore } from '../stores/use-server-store';

export const useMembers = (serverId: string | null) => {
    const members = useServerStore((state) => state.members);
    const isLoading = useServerStore((state) => state.isLoading);
    const error = useServerStore((state) => state.error);
    const getMembers = useServerStore((state) => state.getMembers);

    useEffect(() => {
        if (serverId) {
            getMembers(serverId);
        }
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [serverId]);

    return {
        members,
        isLoading,
        error,
    };
};
