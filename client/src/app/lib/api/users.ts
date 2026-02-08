import { apiClient } from './client';
import type { User, Status } from '@/types/models/user';

// Récupère les informations de l'utilisateur connecté
export const getMe = async (): Promise<User> => {
    return await apiClient.get<User>('/users/me');
};

// Met à jour le statut de l'utilisateur connecté
export const updateStatus = async (status: Status): Promise<User> => {
    return await apiClient.put<User>('/users/me/status', { status });
};
