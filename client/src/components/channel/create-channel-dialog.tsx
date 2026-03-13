import { Input } from "@/components/ui/input";
import { Dialog } from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Plus } from 'lucide-react';
import { useState } from "react";
import { useChannelStore } from "@/app/lib/stores/use-channel-store";
import { useTranslation } from 'react-i18next';

export default function CreateChannelDialog({ serverId }: { serverId: string }) {
    const [isDialogOpen, setIsDialogOpen] = useState(false);
    const [isLoading, setIsLoading] = useState(false);
    const [channelName, setChannelName] = useState("");
    const { t } = useTranslation();

    const addChannel = useChannelStore((state) => state.addChannel);

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        if (!channelName.trim()) return;

        setIsLoading(true);
        try {
            await addChannel(serverId, channelName);
            setChannelName("");
            setIsDialogOpen(false);
        } catch (error) {
            console.error("Erreur création channel", error);
        } finally {
            setIsLoading(false);
        }
    };

    return (
        <>
            <Button
                width="26px"
                height="26px"
                style={{ borderRadius: '10px', backgroundColor: '#5865F2', color: 'white', padding: 0 }}
                onClick={() => setIsDialogOpen(true)}>
                <Plus size={14} />
            </Button>

            <Dialog
                isOpen={isDialogOpen}
                onClose={() => setIsDialogOpen(false)}
                title={t('Create_channel_dialog.Create_channel')}
            >
                <form onSubmit={handleSubmit} className="flex flex-col gap-8">
                    <Input
                        label={t('Create_channel_dialog.Name_channel')}
                        type="text"
                        placeholder={t('Create_channel_dialog.Name_channel')}
                        value={channelName}
                        onChange={(e) => setChannelName(e.target.value)}
                        disabled={isLoading}
                    />


                    <Button
                        variant={'primary'}
                        width="100%"
                        type="submit"
                        disabled={isLoading || !channelName}
                    >
                        {isLoading ? t('Create_channel_dialog.Creation') : t('Create_channel_dialog.Create_channel')}
                    </Button>

                </form>
            </Dialog>
        </>
    )
}
