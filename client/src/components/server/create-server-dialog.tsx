import { Input } from "@/components/ui/input";
import { Dialog } from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { useState } from "react";
import { serversApi } from "@/app/lib/api/servers";
import { useServerStore } from "@/app/lib/stores/use-server-store";
import { useChannelStore } from "@/app/lib/stores/use-channel-store";
import { useTranslation } from 'react-i18next';
import { useRouter } from 'next/navigation';

interface CreateServerDialogProps {
    isOpen?: boolean;
    onClose?: () => void;
    onBack?: () => void;
}

export default function CreateServerDialog({ isOpen, onClose, onBack }: CreateServerDialogProps) {
    const { t } = useTranslation();
    const [internalIsOpen, setInternalIsOpen] = useState(false);
    const [serverName, setServerName] = useState("");
    const [isLoading, setIsLoading] = useState(false);
    const router = useRouter();

    const addServer = useServerStore((state) => state.addServer);
    const addChannel = useChannelStore((state) => state.addChannel);

    const dialogIsOpen = isOpen !== undefined ? isOpen : internalIsOpen;
    const handleClose = onClose || (() => setInternalIsOpen(false));
    const handleBack = onBack || handleClose;

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        if (!serverName.trim()) return;

        setIsLoading(true);
        try {
            const newServer = await serversApi.create(serverName);
            router.push(`/servers/${newServer.id}`);
            addServer(newServer);
            const welcomeChannel = await addChannel(newServer.id, "Invites");
            setServerName("");
            handleClose();
        } catch (error) {
            console.error("Erreur création serveur", error);
        } finally {
            setIsLoading(false);
        }
    };

    return (
        <>
            {isOpen === undefined && (
                <Button
                    style={{ borderRadius: '10px', backgroundColor: '#5865F2', color: 'white', padding: 0 }}
                    onClick={() => setInternalIsOpen(true)}
                >
                    {t('Create_server_dialog.create_server')}
                </Button>
            )}

            <Dialog
                isOpen={dialogIsOpen}
                onClose={handleClose}
                title={t('Create_server_dialog.create_server')}
            >
                <form onSubmit={handleSubmit} className="flex flex-col gap-8">
                    <Input
                        label={t('Create_server_dialog.server_name')}
                        type="text"
                        placeholder={t('Create_server_dialog.server_name_placeholder')}
                        value={serverName}
                        onChange={(e) => setServerName(e.target.value)}
                        disabled={isLoading}
                    />

                    <div className="text-white flex justify-between">

                        <Button
                            variant={"noBackground"}
                            width="80px"
                            onClick={handleBack}>
                            {t('Create_server_dialog.back')}
                        </Button>

                        <Button
                            variant={'primary'}
                            width="200px"
                            type="submit"
                            disabled={isLoading || !serverName || serverName.trim() === ""}
                        >
                            {isLoading ? t('Create_server_dialog.creating') : t('Create_server_dialog.create_the_server')}
                        </Button>
                    </div>

                </form>
            </Dialog>
        </>
    );
}
