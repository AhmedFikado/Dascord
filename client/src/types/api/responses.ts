
 // Réponses API pour l'auth
export interface AuthResponse {
    token: string;
    user_id: string;
}

export interface LogoutResponse {
    message: string;
}

export interface MeResponse {
    user_id: string;
    exp: number;
    iat: number;
}
