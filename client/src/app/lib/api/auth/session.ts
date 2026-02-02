import { getMe } from '../users';
import { hasToken, removeToken } from './token';
import { useAuthStore } from '../../stores/use-auth-store';

/**
 * Initialise la session utilisateur au chargement de l'app
 * Vérifie si un token existe et si l'utilisateur est toujours connecté
 */
export const initializeSession = async (): Promise<boolean> => {
    if (!hasToken()) {
        return false;
    }

    try {
        const response = await getMe();
        
        // Stocker l'utilisateur complet
        useAuthStore.getState().setUser({
            id: response.id,
            username: response.username,
            email: response.email,
            status: response.status as any,
            created_at: new Date(response.created_at),
        });
        useAuthStore.getState().setUserId(response.id);

        return true;
    } catch (error) {
        removeToken();
        useAuthStore.getState().setUser(null);
        useAuthStore.getState().setUserId(null);
        return false;
    }
};

/**
 * Vérifie si l'utilisateur a une session active
 */
export const checkSession = (): boolean => {
    return hasToken() && useAuthStore.getState().isAuthenticated;
};