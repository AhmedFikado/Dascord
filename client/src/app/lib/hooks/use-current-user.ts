import { useAuthStore } from '../stores/use-auth-store';
import type { User } from '@/types/models/user';

export const useCurrentUser = () => {
    const user = useAuthStore((state) => state.user);
    const userId = useAuthStore((state) => state.userId);
    const isAuthenticated = useAuthStore((state) => state.isAuthenticated);
    const isLoading = useAuthStore((state) => state.isLoading);

    return {
        user,
        userId,
        isAuthenticated,
        isLoading,
    };
};
