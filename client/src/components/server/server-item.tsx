'use client';

import { Server } from "@/types/models/Server";
import { useRouter, useParams } from 'next/navigation';
import { useChannelStore } from '@/app/lib/stores/use-channel-store';
import { useUnreadStore } from '@/app/lib/stores/use-unread-store';
import { useEffect } from 'react';

interface ServerItemProps {
    server: Server;
}

export default function ServerItem({ server }: ServerItemProps) {
    const router = useRouter();
    const params = useParams();
    const currentServerId = params?.serverId as string | undefined;
    const { channelsByServer, fetchChannels } = useChannelStore();
    const { isServerUnread, fetchUnreadForServer } = useUnreadStore();

    useEffect(() => {
        fetchUnreadForServer(server.id);
    }, [server.id, fetchUnreadForServer]);

    const getInitials = (name: string): string => {
        const words = name.trim().split(' ');
        if (words.length === 1) {
            return words[0][0].toUpperCase();
        }
        return words.slice(0, 2).map(word => word[0].toUpperCase()).join('');
    };

    const handleClick = async () => {
        const channels = channelsByServer[server.id];
        if (!channels || channels.length === 0) {
            await fetchChannels(server.id);
            const updatedChannels = useChannelStore.getState().channelsByServer[server.id];
            if (updatedChannels && updatedChannels.length > 0) {
                router.push(`/servers/${server.id}/channels/${updatedChannels[0].id}`);
            }
        } else {
            router.push(`/servers/${server.id}/channels/${channels[0].id}`);
        }
    };

    const isActive = currentServerId === server.id;
    const hasUnread = isServerUnread(server.id);

    return (
        <div className="relative mb-2">
            <div
                onClick={handleClick}
                className={`
                    w-12 h-12 min-w-12 min-h-12 flex-shrink-0
                    flex items-center justify-center
                    text-white font-bold text-lg cursor-pointer
                    transition-all duration-200 rounded-[16px]
                    ${isActive
                        ? 'bg-blurple rounded-[16px]'
                        : 'bg-gray-300 rounded-[16px] hover:bg-blurple hover:rounded-[16px]'
                    }
                `}
                title={server.name}
            >
                {getInitials(server.name)}
            </div>
            {hasUnread && !isActive && (
                <span className="absolute bottom-0 right-0 w-3.5 h-3.5 bg-red-500 rounded-full border-2 border-backgroundPrimary" />
            )}
        </div>
    );
}
