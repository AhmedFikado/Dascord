'use client';

import { Channel } from "@/types/models/channel";
import ChannelSetting from "./channel-setting";
import { Button } from "@/components/ui/button";
import { useState } from "react";
import { Dialog } from "../ui/dialog";
import { useRouter, useParams } from 'next/navigation';
import { useChannelStore } from "@/app/lib/stores/use-channel-store";
import { useServerStore } from "@/app/lib/stores/use-server-store";
import { useCurrentUser } from "@/app/lib/hooks/use-current-user";
import { Input } from "@/components/ui/input";
import { Save } from "lucide-react";
import { Role } from "@/types/models/role";

interface ChannelItemProps {
    channel: Channel;
}

export default function ChannelItem({ channel }: ChannelItemProps) {
    const [isDialogOpen, setIsDialogOpen] = useState(false);
    const [newChannelName, setNewChannelName] = useState(channel.name);
    const [isLoading, setIsLoading] = useState(false);
    const router = useRouter();
    const params = useParams();

    const { userId } = useCurrentUser();
    const servers = useServerStore((state) => state.servers);
    const members = useServerStore((state) => state.members);
    const serverId = params?.serverId as string;
    const currentServer = servers.find(s => s.id === serverId);
    const currentMember = members.find(m => m.user_id === userId);
    const canManageChannels = currentMember ? 
        (currentMember.role === Role.OWNER || currentMember.role === Role.ADMIN) : false;
    const currentChannelId = params?.channelId ? params.channelId as string : null;
    const isActive = currentChannelId === channel.id;
    const removeChannel = useChannelStore((state) => state.removeChannel);
    const updateChannel = useChannelStore((state) => state.updateChannel);


    const deleteChannel = async () => {
        await removeChannel(channel.id);
        setIsDialogOpen(false);
    };

    const handleSave = async () => {
        setIsLoading(true);
        await updateChannel(channel.id, newChannelName);
        setIsLoading(false);
        setIsDialogOpen(false)
    };

    const handleReset = () => {
        setNewChannelName(channel.name);
    };

    const handleChannelClick = () => {
        router.push(`/servers/${serverId}/channels/${channel.id}`);
    };

    const handleSettingsClick = (e: React.MouseEvent) => {
        e.stopPropagation();
        setIsDialogOpen(true);
    };

    const hasChanges = newChannelName !== channel.name;

    return (
        <>
            <li
                key={channel.id}
                onClick={handleChannelClick}
                className={`
                    group pl-3 pr-3 py-[6px] rounded-lg cursor-pointer mb-1 mx-1
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
                {canManageChannels && (
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
                )}
            </li>

            <Dialog
                isOpen={isDialogOpen}
                onClose={() => setIsDialogOpen(false)}
                title="Supprimer un channel"
            >
                <Input
                    label="Nom du channel"
                    type="text"
                    required
                    id="channel-name"
                    name="channel-name"
                    value={newChannelName}
                    onChange={(e) => setNewChannelName(e.target.value)}
                    placeholder="Nom du channel"
                />


                {hasChanges && (
                    <div className="flex justify-center">
                        <div className="flex gap-10 justify-center mt-4 bg-gray-400 py-3 rounded-xl w-4/5">
                            <Button
                                variant="secondary"
                                onClick={handleReset}
                            >
                                Réinitialiser
                            </Button>
                            <Button
                                variant="primary"
                                onClick={handleSave}
                                isLoading={isLoading}
                                className="flex items-center gap-2"
                            >
                                <Save size={16} />
                                Enregistrer
                            </Button>
                        </div>
                    </div>
                )}



                <div className="flex gap-4 justify-center mt-8">
                    <Button
                        onClick={() => deleteChannel()}
                        variant={'danger'}
                        width="200px"
                        style={{ alignSelf: 'center' }}
                        type="submit"
                    >
                        Supprimer le channel
                    </Button>
                </div>
            </Dialog>
        </>
    );
}