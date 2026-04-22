export interface BannedMember {
    user_id: string;
    username: string;
    ban_type: 'Permanent' | 'Temporary';
    banned_at: string;
    expires_at: string | null;
}
