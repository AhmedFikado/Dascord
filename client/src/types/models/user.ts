export interface User {
    id: string,
    username: string,
    email: string,
    created_at: Date,
    status: Status,
    language: string,
}

export enum Status {
    ONLINE = "ONLINE",
    OFFLINE = "OFFLINE"
}