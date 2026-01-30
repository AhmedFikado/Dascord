'use client';

import { use } from 'react';
import { Channel } from "@/types/models/channel";
import ChannelList from "../channel/channel-list";
import ServerHeaderSide from "./server-header-side";
import CreateChannelDialog from "../channel/create-channel-dialog";
import { useServerStore } from '@/app/lib/stores/use-server-store';

interface ServerSidebarProps {
    serverId: number;
}

export default function ServerSidebar({ serverId }: ServerSidebarProps) {

    const { servers } = useServerStore();
    const server = servers.find(s => s.id === serverId);

    const channelsData = {
        1: [
            { id: 1, server_id: 1, name: 'Général', created_at: new Date() },
            { id: 2, server_id: 1, name: 'information', created_at: new Date() },
            { id: 3, server_id: 1, name: 'Invites', created_at: new Date() },
        ],
        2: [
            { id: 4, server_id: 2, name: 'general', created_at: new Date() },
            { id: 5, server_id: 2, name: 'announcements', created_at: new Date() },
        ],
        3: [
            { id: 6, server_id: 3, name: 'dev-chat', created_at: new Date() },
            { id: 7, server_id: 3, name: 'code-review', created_at: new Date() },
        ],
    };
    const channelsList = channelsData[serverId as keyof typeof channelsData] || [];

    return (
        <aside className="w-60 h-full flex flex-col bg-backgroundSide overflow-y-auto scrollbar-hide flex-shrink-0">
            <div className="p-4 border-b border-gray-200 flex-shrink-0">
                {server && <ServerHeaderSide server={server} />}
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