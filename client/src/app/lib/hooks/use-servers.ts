import { useEffect } from 'react';
import { useServerStore } from '../stores/use-server-store';

export function useServers() {
    const servers = useServerStore((state) => state.servers);
    const isLoading = useServerStore((state) => state.isLoading);
    const error = useServerStore((state) => state.error);
    const fetchServers = useServerStore((state) => state.fetchServers);

    useEffect(() => {
        fetchServers();
    }, [fetchServers]);

    return { servers, isLoading, error };
}