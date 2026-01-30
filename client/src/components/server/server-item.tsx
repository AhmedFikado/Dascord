'use client';

import { Server } from "@/types/models/Server";
import { useRouter, useParams } from 'next/navigation';

interface ServerItemProps {
    server: Server;
}

export default function ServerItem({ server }: ServerItemProps) {
    const router = useRouter();
    const params = useParams();
    const currentServerId = params?.serverId ? parseInt(params.serverId as string) : null;

    const getInitials = (name: string): string => {
        const words = name.trim().split(' ');
        if (words.length === 1) {
            return words[0][0].toUpperCase();
        }
        return words.slice(0, 2).map(word => word[0].toUpperCase()).join('');
    };

    const handleClick = () => {
        router.push(`/servers/${server.id}/channels/1`);
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