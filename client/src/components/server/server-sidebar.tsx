'use client';

import { use } from 'react';
import { Channel } from "@/types/models/channel";
import ChannelList from "../channel/channel-list";
import ServerHeaderSide from "./server-header-side";
import CreateChannelDialog from "../channel/create-channel-dialog";
import { useServerStore } from '@/app/lib/stores/use-server-store';
import { useChannels } from '@/app/lib/hooks/use-channels';

interface ServerSidebarProps {
    serverId: number;
}

export default function ServerSidebar({ serverId }: ServerSidebarProps) {

    const { servers } = useServerStore();
    const { channels, isLoading } = useChannels(serverId);

    const server = servers.find(s => s.id === serverId);

    return (
        <aside className="w-60 h-full flex flex-col bg-backgroundSide overflow-y-auto scrollbar-hide flex-shrink-0">
            <div className="p-4 border-b border-gray-200 flex-shrink-0">
                {server && <ServerHeaderSide server={server} />}
            </div>
            <div className="flex-1 overflow-y-auto">
                <div className="px-3 mb-3 py-3 flex items-center justify-between border-b border-gray-200 text-gray-light">
                    Créer un channel
                    <CreateChannelDialog serverId={serverId} />
                </div>
                <ChannelList channels={channels} />
            </div>
        </aside>
    );
}