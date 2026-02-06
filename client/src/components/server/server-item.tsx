'use client';

import { Server } from "@/types/models/Server";
import { useRouter, useParams } from 'next/navigation';
import { useChannelStore } from '@/app/lib/stores/use-channel-store';
import { useEffect, useState } from 'react';

interface ServerItemProps {
    server: Server;
}

export default function ServerItem({ server }: ServerItemProps) {
    const router = useRouter();
    const params = useParams();
    const currentServerId = params?.serverId as string | undefined;
    const { channelsByServer, fetchChannels } = useChannelStore();
    const [firstChannelId, setFirstChannelId] = useState<string | null>(null);

    useEffect(() => {
        const channels = channelsByServer[server.id];
        if (channels && channels.length > 0) {
            setFirstChannelId(channels[0].id);
        }
    }, [channelsByServer, server.id]);

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

    return (
        <div
            onClick={handleClick}
            className={`
                w-12 h-12 min-w-12 min-h-12 flex-shrink-0
                flex items-center justify-center 
                text-white font-bold text-lg cursor-pointer 
                transition-all duration-200 mb-2 rounded-[16px]
                ${isActive
                    ? 'bg-blurple rounded-[16px]'
                    : 'bg-gray-300 rounded-[16px] hover:bg-blurple hover:rounded-[16px]'
                }
            `}
            title={server.name}
        >
            {getInitials(server.name)}
        </div>
    );
}