import { Input } from "@/components/ui/input";
import { Dialog } from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { useState } from "react";
import { serversApi } from "@/app/lib/api/servers";
import { useServerStore } from "@/app/lib/stores/use-server-store";
import { useChannelStore } from "@/app/lib/stores/use-channel-store";

interface CreateServerDialogProps {
    isOpen?: boolean;
    onClose?: () => void;
    onBack?: () => void;
}

export default function CreateServerDialog({ isOpen, onClose, onBack }: CreateServerDialogProps) {
    const [internalIsOpen, setInternalIsOpen] = useState(false);
    const [serverName, setServerName] = useState("");
    const [isLoading, setIsLoading] = useState(false);

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
                    Créer un serveur
                </Button>
            )}

            <Dialog
                isOpen={dialogIsOpen}
                onClose={handleClose}
                title="Créer un serveur"
            >
                <form onSubmit={handleSubmit} className="flex flex-col gap-8">
                    <Input
                        label="Nom du serveur"
                        type="text"
                        placeholder="Ex: Le repaire des codeurs"
                        value={serverName}
                        onChange={(e) => setServerName(e.target.value)}
                        disabled={isLoading}
                    />

                    <div className="text-white flex justify-between">

                        <Button
                            variant={"noBackground"}
                            width="80px"
                            onClick={handleBack}>
                            Retour
                        </Button>

                        <Button
                            variant={'primary'}
                            width="200px"
                            type="submit"
                            disabled={isLoading || !serverName}
                        >
                            {isLoading ? "Création..." : "Créer le serveur"}
                        </Button>
                    </div>

                </form>
            </Dialog>
        </>
    );
}
