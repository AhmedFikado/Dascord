import { apiClient } from './client';
import type { User } from '@/types/models/user';

// Récupère les informations de l'utilisateur connecté
export const getMe = async (): Promise<User> => {
    return await apiClient.get<User>('/users/me');
};
