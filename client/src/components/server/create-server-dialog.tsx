import { Input } from "@/components/ui/input";
import { Dialog } from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Plus } from 'lucide-react';
import { useState } from "react";
import { serversApi } from "@/app/lib/api/servers";
import { useServerStore } from "@/app/lib/stores/use-server-store";

export default function CreateServerDialog() {
    const [isDialogOpen, setIsDialogOpen] = useState(false);
    const [serverName, setServerName] = useState("");
    const [isLoading, setIsLoading] = useState(false);

    const addServer = useServerStore((state) => state.addServer);

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        if (!serverName.trim()) return;

        setIsLoading(true);
        try {
            const newServer = await serversApi.create(serverName);
            addServer(newServer);
            setServerName("");
            setIsDialogOpen(false);
        } catch (error) {
            console.error("Erreur création serveur", error);
        } finally {
            setIsLoading(false);
        }
    };

    return (
        <>
            <Button
                width="45px"
                height="45px"
                style={{ borderRadius: '1rem', backgroundColor: '#5865F2', color: 'white', padding: 0 }}
                onClick={() => setIsDialogOpen(true)}>
                <Plus />
            </Button>

            <Dialog
                isOpen={isDialogOpen}
                onClose={() => setIsDialogOpen(false)}
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

                    <Button
                        variant={'primary'}
                        width="100%"
                        type="submit"
                        disabled={isLoading || !serverName}
                    >
                        {isLoading ? "Création..." : "Créer le serveur"}
                    </Button>
                </form>
            </Dialog>
        </>
    )
}