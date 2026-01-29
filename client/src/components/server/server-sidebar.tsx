import { Channel } from "@/types/models/channel";
import ChannelList from "../channel/channel-list";
import ServerHeaderSide from "./server-header-side";
import { Server } from "@/types/models/Server";
import CreateChannelDialog from "../channel/create-channel-dialog";


export default function ServerSidebar({ serverId }) {

    const server: Server = {
        id: 1,
        owner_id: 1,
        invitation_code: 'INVITE123',
        name: 'Serveur Ethan',
        created_at: new Date(),
    };

    const channelsList: Channel[] = [
        {
            id: 1,
            server_id: 1,
            name: 'Général',
            created_at: new Date(),
        },
        {
            id: 2,
            server_id: 2,
            name: 'information',
            created_at: new Date(),
        },
        {
            id: 3,
            server_id: 3,
            name: 'Invites',
            created_at: new Date(),
        },
    ];

    return (
        <aside className="w-60 h-full flex flex-col bg-backgroundSide overflow-y-auto scrollbar-hide flex-shrink-0">
            <div className="p-4 border-b border-gray-200 flex-shrink-0">
                <ServerHeaderSide server={server} />
            </div>
            <div className="flex-1 overflow-y-auto">
                <div className="px-3 mb-3 py-3 flex items-center justify-between border-b border-gray-200 text-gray-light">
                    Créer un channel
                    <CreateChannelDialog />
                </div>
                <ChannelList channels={channelsList} />
            </div>
        </aside>
    );
}