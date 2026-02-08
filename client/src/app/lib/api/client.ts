import axios, { AxiosError, InternalAxiosRequestConfig } from 'axios';
import { getToken, removeToken } from './auth/token';

const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080';

const axiosInstance = axios.create({
    baseURL: API_BASE_URL,
    timeout: 10000,
    headers: { 'Content-Type': 'application/json' },
});

// Intercepteur requête : ajoute le token JWT
axiosInstance.interceptors.request.use((config: InternalAxiosRequestConfig) => {
    const token = getToken();
    if (token && config.headers) {
        config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
});

// Intercepteur réponse : gère les erreurs
axiosInstance.interceptors.response.use(
    (response) => response,
    (error: AxiosError) => {
        if (error.response?.status === 401 && typeof window !== 'undefined') {
            const errorMessage = (error.response.data as any)?.error || '';
            if (errorMessage.includes('token') || errorMessage.includes('Authorization header')) {
                removeToken();
                window.location.href = '/login';
            }
        }
        return Promise.reject(error);
    }
);

export const apiClient = {
    get: <T>(url: string, config?: any) =>
        axiosInstance.get<T>(url, config).then(res => res.data),

    post: <T>(url: string, data?: any, config?: any) =>
        axiosInstance.post<T>(url, data, config).then(res => res.data),

    put: <T>(url: string, data?: any, config?: any) =>
        axiosInstance.put<T>(url, data, config).then(res => res.data),

    patch: <T>(url: string, data?: any, config?: any) =>
        axiosInstance.patch<T>(url, data, config).then(res => res.data),

    delete: <T>(url: string, config?: any) =>
        axiosInstance.delete<T>(url, config).then(res => res.data),
};

export class ApiError extends Error {
    constructor(
        message: string,
        public status: number,
        public data?: unknown
    ) {
        super(message);
        this.name = 'ApiError';
    }
}

export { axiosInstance };
