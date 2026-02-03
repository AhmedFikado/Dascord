import { Server } from "@/types/models/Server";

let mockServers: Server[] = [
    { id: '1', name: 'Serveur Ethan', owner_id: '1', invitation_code: 'ABC', created_at: new Date() },
    { id: '2', name: 'Zodiak', owner_id: '2', invitation_code: 'DEF', created_at: new Date() },
    { id: '3', name: 'Sprite Dev', owner_id: '3', invitation_code: 'GHI', created_at: new Date() },
    { id: '4', name: 'Bla Bla', owner_id: '4', invitation_code: 'EFG', created_at: new Date() },
];

export const serversApi = {

    // GET /servers
    getAll: async (): Promise<Server[]> => {
        return [...mockServers]; //obligé de rajouter ça pour pas créer de doublons donc à ne pas mettre quand l'API sera liée
    },

    // GET /server/{id}
    getOne: async (serverId: string): Promise<Server | undefined> => {
        return mockServers.find(s => s.id === serverId);
    },

    // POST /servers
    create: async (name: string): Promise<Server> => {
        const newServer: Server = {
            id: Date.now().toString(),
            name,
            owner_id: '1',
            invitation_code: Math.random().toString(36).substring(2, 5).toUpperCase(),
            created_at: new Date()
        };
        mockServers.push(newServer);
        return newServer;
    },

    // DELETE /servers/{id}
    delete: async (serverId: string): Promise<void> => {
        mockServers = mockServers.filter(s => s.id !== serverId);
    }
};
