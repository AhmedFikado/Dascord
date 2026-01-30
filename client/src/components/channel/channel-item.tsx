'use client';

import { Channel } from "@/types/models/channel";
import ChannelSetting from "./channel-setting";
import { Button } from "@/components/ui/button";
import { useState } from "react";
import { Dialog } from "../ui/dialog";
import { useRouter, useParams } from 'next/navigation';

interface ChannelItemProps {
    channel: Channel;
}

export default function ChannelItem({ channel }: ChannelItemProps) {
    const [isDialogOpen, setIsDialogOpen] = useState(false);
    const router = useRouter();
    const params = useParams();

    const currentChannelId = params?.channelId ? parseInt(params.channelId as string) : null;
    const serverId = params?.serverId || '1';
    const isActive = currentChannelId === channel.id;

    const deleteChannel = async () => {
        console.log("Deleting channel");
        setIsDialogOpen(false);
    };

    const handleChannelClick = () => {
        router.push(`/servers/${serverId}/channels/${channel.id}`);
    };

    const handleSettingsClick = (e: React.MouseEvent) => {
        e.stopPropagation();
        setIsDialogOpen(true);
    };

    return (
        <>
            <li
                key={channel.id}
                onClick={handleChannelClick}
                className={`
                    group pl-3 pr-3 py-[6px] rounded-lg cursor-pointer 
                    flex justify-between items-center transition-colors
                    ${isActive
                        ? 'bg-hoverSide text-white'
                        : 'text-gray-light hover:bg-hoverSide hover:text-white'
                    }
                `}
            >
                <div className="flex items-center gap-2">
                    <span className={isActive ? 'text-white' : 'text-gray-50'}>#</span>
                    <span className={`text-sm font-medium ${isActive ? 'text-white' : ''}`}>
                        {channel.name}
                    </span>
                </div>
                <div>
                    <Button
                        onClick={handleSettingsClick}
                        variant="noBackground"
                        width="26px"
                        height="26px"
                        style={{ borderRadius: '10px', padding: 0 }}
                    >
                        <ChannelSetting />
                    </Button>
                </div>
            </li>

            <Dialog
                isOpen={isDialogOpen}
                onClose={() => setIsDialogOpen(false)}
                title="Supprimer un channel"
            >
                <h2 className="text-white mb-8 text-center">
                    Êtes-vous sûr de vouloir supprimer le channel #{channel.name} ?
                </h2>

                <div className="flex gap-4 justify-center">
                    <Button
                        onClick={() => deleteChannel()}
                        variant={'danger'}
                        width="200px"
                        style={{ alignSelf: 'center' }}
                        type="submit"
                    >
                        Supprimer le channel
                    </Button>
                    <Button
                        onClick={() => setIsDialogOpen(false)}
                        variant={'secondary'}
                        width="100px"
                        style={{ alignSelf: 'center' }}
                        type="submit"
                    >
                        Annuler
                    </Button>
                </div>
            </Dialog>
        </>
    );
}