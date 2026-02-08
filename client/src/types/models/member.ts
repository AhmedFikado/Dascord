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
 * Type pour mettre à jour le rôle d'un membre
 */
export interface UpdateMemberRole {
    role: Role;
}

