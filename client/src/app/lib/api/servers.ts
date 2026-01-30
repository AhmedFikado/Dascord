import { Server } from "@/types/models/Server";

const servers: Server[] = [
    { id: 1, name: 'Serveur Ethan', owner_id: 1, invitation_code: 'ABC', created_at: new Date() },
    { id: 2, name: 'Zodiak', owner_id: 2, invitation_code: 'DEF', created_at: new Date() },
    { id: 3, name: 'Sprite Dev', owner_id: 3, invitation_code: 'GHI', created_at: new Date() },
    { id: 4, name: 'Bla Bla', owner_id: 4, invitation_code: 'EFG', created_at: new Date() },

];

export const serversApi = {

    getAll: async () => {
        console.log()
        return Promise.resolve(servers);
    },
}
