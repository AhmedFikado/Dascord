import { apiClient } from './client';
import type { User } from '@/types/models/user';
import { Status } from '@/types/models/status';

// Récupère les informations de l'utilisateur connecté
export const getMe = async (): Promise<User> => {
    return await apiClient.get<User>('/users/me');
};

// Met à jour le statut de l'utilisateur connecté
export const updateStatus = async (status: Status): Promise<User> => {
    return await apiClient.put<User>('/users/me/status', { status });
};

// Met à jour les informations de l'utilisateur connecté
export const updateUserInfo = async (userInfo: Partial<User>): Promise<User> => {
    return await apiClient.put<User>('/users/update_user', userInfo);
};