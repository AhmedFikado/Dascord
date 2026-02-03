import { Role } from './role';
import { User } from './user';

export interface Member {
    server_id: string;
    user_id: string;
    role: Role;
    joined_at: Date;
    user: User;
}

/**
 * Type pour ajouter un membre à un serveur
 */
export interface CreateMember {
    server_id: string;
    user_id: string;
    role?: Role;
}

/**
 * Type pour mettre à jour le rôle d'un membre
 */
export interface UpdateMemberRole {
    role: Role;
}

