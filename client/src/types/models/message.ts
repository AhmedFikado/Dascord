export interface Message {
    id: string | null;
    channel_id: string;
    user_id: string;
    username: string;
    content: string;
    created_at: string;
    updated_at?: string;
    deleted_at?: string;
    reactions?: Record<string, string[]>;
}