import { apiClient } from './client';
import { LoginFormData, SignupFormData } from './validations/auth.schema';
import { saveToken, removeToken } from './auth/token';
import type { AuthResponse, LogoutResponse, MeResponse } from '@/types/api/responses';

// Inscription d'un nouvel utilisateur
export const signup = async (data: SignupFormData): Promise<AuthResponse> => {
    const response = await apiClient.post<AuthResponse>('/auth/signup', data);
    saveToken(response.token);
    return response;
};

// Connexion d'un utilisateur
export const login = async (data: LoginFormData): Promise<AuthResponse> => {
    const response = await apiClient.post<AuthResponse>('/auth/login', data);
    saveToken(response.token);
    return response;
};

// Déconnexion de l'utilisateur
export const logout = async (): Promise<LogoutResponse> => {
    const response = await apiClient.post<LogoutResponse>('/auth/logout');
    removeToken();
    return response;
};

// Récupère les informations de l'utilisateur connecté
export const getMe = async (): Promise<MeResponse> => {
    return await apiClient.get<MeResponse>('/auth/me');
};
