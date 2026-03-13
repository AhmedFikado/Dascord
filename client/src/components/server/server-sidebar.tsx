'use client';

import { use } from 'react';
import { Channel } from "@/types/models/channel";
import ChannelList from "../channel/channel-list";
import ServerHeaderSide from "./server-header-side";
import CreateChannelDialog from "../channel/create-channel-dialog";
import { useServerStore } from '@/app/lib/stores/use-server-store';
import { useChannels } from '@/app/lib/hooks/use-channels';
import { useCurrentUser } from '@/app/lib/hooks/use-current-user';
import { Role } from '@/types/models/role';
import { useTranslation } from 'react-i18next';

interface ServerSidebarProps {
    serverId: string;
}

export default function ServerSidebar({ serverId }: ServerSidebarProps) {

    const { t } = useTranslation();
    const { userId } = useCurrentUser();
    const { servers, members } = useServerStore();
    const { channels, isLoading } = useChannels(serverId);

    const server = servers.find(s => s.id === serverId);
    const currentMember = members.find(m => m.user_id === userId);
    const canManageChannels = currentMember ?
        (currentMember.role === Role.OWNER || currentMember.role === Role.ADMIN) : false;

    return (
        <aside className="w-60 h-full flex flex-col bg-backgroundSide overflow-y-auto scrollbar-hide flex-shrink-0">
            <div className="p-4 border-b border-gray-200 flex-shrink-0">
                {server && <ServerHeaderSide server={server} />}
            </div>
            <div className="flex-1 overflow-y-auto">
                {canManageChannels && (
                    <div className="px-3 mb-3 py-3 flex items-center justify-between border-b border-gray-200 text-gray-light">
                        {t('Server_sidebar.create_channel')}
                        <CreateChannelDialog serverId={serverId} />
                    </div>
                )}
                <ChannelList channels={channels} />
            </div>
        </aside>
    );
}