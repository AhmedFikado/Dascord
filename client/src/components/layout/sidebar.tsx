import ServerList from "../server/server-list";
import { Server } from "@/types/models/Server";
import CreateServerDialog from "../server/create-server-dialog";

export default function Sidebar() {

    const serversList: Server[] = [
        {
            id: 1,
            owner_id: 1,
            invitation_code: 'INVITE123',
            name: 'Serveur Ethan',
            created_at: new Date(),
        },
        {
            id: 2,
            owner_id: 2,
            invitation_code: 'INVITE456',
            name: 'Zodiak',
            created_at: new Date(),
        },
        {
            id: 3,
            owner_id: 3,
            invitation_code: 'INVITE789',
            name: 'Sprite Dev',
            created_at: new Date(),
        },
    ];

    return (
        <aside className="w-18 h-full bg-backgroundSide flex flex-col items-center py-3 gap-2 overflow-y-auto scrollbar-hide flex-shrink-0 border-r border-gray-200">
            <div className="w-12 h-12 bg-gray-400 rounded-2xl mb-1">
                <img
                    src="/icons/logo.png"
                    alt="Logo"
                    className="w-full h-full object-contain rounded-2xl"
                />
            </div>
            <ServerList servers={serversList} />
            <CreateServerDialog />
        </aside>
    );
}