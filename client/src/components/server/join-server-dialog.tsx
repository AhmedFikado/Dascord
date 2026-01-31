import { Input } from "@/components/ui/input";
import { Dialog } from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { useState } from "react";

interface JoinServerDialogProps {
    isOpen?: boolean;
    onClose?: () => void;
    onBack?: () => void;
}

export default function JoinServerDialog({ isOpen, onClose, onBack }: JoinServerDialogProps) {
    const [internalIsOpen, setInternalIsOpen] = useState(false);
    const [isLoading, setIsLoading] = useState(false);
    const [invitationCode, setInvitationCode] = useState("");

    const dialogIsOpen = isOpen !== undefined ? isOpen : internalIsOpen;
    const handleClose = onClose || (() => setInternalIsOpen(false));
    const handleBack = onBack || handleClose;

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        if (!invitationCode.trim()) return;

        setIsLoading(true);
        try {
            handleClose();
            setInvitationCode("");
        } catch (error) {
            console.error("Erreur pour rejoindre le serveur", error);
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
                    Rejoindre un serveur
                </Button>
            )}

            <Dialog
                isOpen={dialogIsOpen}
                onClose={handleClose}
                title="Rejoindre un serveur"
            >
                <form onSubmit={handleSubmit} className="flex flex-col gap-8">
                    <Input
                        label="Code d'invitation"
                        type="text"
                        placeholder="Entrez le code ici !"
                        value={invitationCode}
                        onChange={(e) => setInvitationCode(e.target.value)}
                    />

                    <div className="text-white flex justify-between">

                        <Button
                            variant={"noBackground"}
                            width="100px"
                            onClick={handleBack}>
                            Retour
                        </Button>

                        <Button
                            variant={'primary'}
                            width="200px"
                            type="submit"
                        >
                            {isLoading ? "Vous passez la douane ..." : "Rejoindre le serveur"}
                        </Button>

                    </div>

                </form>
            </Dialog>
        </>
    );
}