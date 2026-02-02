export interface User {
    id: number,
    username: string,
    email: string,
    created_at: Date,
    status: Status
}

export enum Status {
    ONLINE,
    OFFLINE
}