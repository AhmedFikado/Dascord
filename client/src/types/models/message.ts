import { User } from "./user";

export interface Message {
    id: string;
    channel_id: string;
    user_id: string;
    content: string;
    created_at: Date;
    updated_at?: Date;
    deleted_at?: Date;
    user: User;
}