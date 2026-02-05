import { create } from 'zustand';
import type { User } from '@/types/models/user';
import * as authApi from '@/app/lib/api/auth';
import { getMe } from '@/app/lib/api/users';
import type { LoginFormData, SignupFormData } from '@/app/lib/api/validations/auth.schema';

interface AuthState {
    user: User | null;
    userId: string | null;
    isAuthenticated: boolean;
    isLoading: boolean;
    error: string | null;
}

interface AuthActions {
    login: (data: LoginFormData) => Promise<void>;
    signup: (data: SignupFormData) => Promise<void>;
    logout: () => Promise<void>;
    setUser: (user: User | null) => void;
    setUserId: (userId: string | null) => void;
    setError: (error: string | null) => void;
    clearError: () => void;
}

type AuthStore = AuthState & AuthActions;

export const useAuthStore = create<AuthStore>((set) => ({
    // État initial
    user: null,
    userId: null,
    isAuthenticated: false,
    isLoading: false,
    error: null,

    // Action: Connexion
    login: async (data: LoginFormData) => {
        set({ isLoading: true, error: null });
        try {
            const response = await authApi.login(data);

            // Récupérer les infos complètes de l'utilisateur
            const userInfo = await getMe();

            set({
                user: {
                    id: userInfo.id,
                    username: userInfo.username,
                    email: userInfo.email,
                    status: userInfo.status as any,
                    created_at: new Date(userInfo.created_at),
                },
                userId: userInfo.id,
                isAuthenticated: true,
                isLoading: false,
            });
        } catch (error: any) {
            const errorMessage = error.response?.data?.message
                || error.response?.data?.error
                || error.message
                || 'Erreur lors de la connexion';
            set({
                error: errorMessage,
                isLoading: false,
            });
            throw error;
        }
    },

    // Action: Inscription
    signup: async (data: SignupFormData) => {
        set({ isLoading: true, error: null });
        try {
            const response = await authApi.signup(data);
            const userInfo = await getMe();

            set({
                user: {
                    id: userInfo.id,
                    username: userInfo.username,
                    email: userInfo.email,
                    status: userInfo.status as any,
                    created_at: new Date(userInfo.created_at),
                },
                userId: userInfo.id,
                isAuthenticated: true,
                isLoading: false,
            });
        } catch (error: any) {
            const errorMessage = error.response?.data?.message
                || error.response?.data?.error
                || error.message
                || 'Erreur lors de l\'inscription';
            set({
                error: errorMessage,
                isLoading: false,
            });
            throw error;
        }
    },

    // Action: Déconnexion
    logout: async () => {
        set({ isLoading: true, error: null });
        try {
            await authApi.logout();
            set({
                user: null,
                userId: null,
                isAuthenticated: false,
                isLoading: false,
            });
            if (typeof window !== 'undefined') {
                window.location.href = '/login';
            }
        } catch (error: any) {
            set({
                error: error.message || 'Erreur lors de la déconnexion',
                isLoading: false,
            });
            throw error;
        }
    },

    // Action: Définir l'utilisateur
    setUser: (user: User | null) => {
        set({
            user,
            isAuthenticated: user !== null,
        });
    },

    // Action: Définir l'ID utilisateur
    setUserId: (userId: string | null) => {
        set({
            userId,
            isAuthenticated: userId !== null,
        });
    },

    // Action: Définir une erreur
    setError: (error: string | null) => {
        set({ error });
    },

    // Action: Effacer l'erreur
    clearError: () => {
        set({ error: null });
    },
}));
