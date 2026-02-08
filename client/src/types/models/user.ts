export interface User {
    id: string,
    username: string,
    email: string,
    created_at: Date,
    status: Status
}

export enum Status {
    ONLINE = "ONLINE",
    OFFLINE = "OFFLINE"
}