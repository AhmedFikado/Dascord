const TOKEN_KEY = 'auth_token';

// Fonction utilitaire pour récupérer un cookie par son nom
const getCookie = (name: string): string | null => {
    if (typeof window === 'undefined') return null;

    const value = `; ${document.cookie}`;
    const parts = value.split(`; ${name}=`);

    if (parts.length === 2) {
        return parts.pop()?.split(';').shift() || null;
    }

    return null;
};

// Sauvegarde le token JWT dans un cookie
export const saveToken = (token: string): void => {
    if (typeof window === 'undefined') return;
    document.cookie = `${TOKEN_KEY}=${token}; path=/; max-age=${7 * 24 * 60 * 60}; SameSite=Lax`;
};

// Récupère le token JWT depuis le cookie
export const getToken = (): string | null => {
    return getCookie(TOKEN_KEY);
};

// Supprime le token JWT du cookie
export const removeToken = (): void => {
    if (typeof window === 'undefined') return;
    document.cookie = `${TOKEN_KEY}=; path=/; max-age=0`;
};

// Vérifie si un token existe
export const hasToken = (): boolean => {
    return getToken() !== null;
};