import { Status } from "./status";

export interface User {
    id: string,
    username: string,
    email: string,
    created_at: Date,
    status: Status,
    language: string,
    avatar_id?: string,
}