import { Input } from "@/components/ui/input";
import { Dialog } from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { useState } from "react";
import { useServerStore } from "@/app/lib/stores/use-server-store";
import { useSnackbar } from "@/components/shared/error-message";
import { useTranslation } from 'react-i18next';

interface JoinServerDialogProps {
    isOpen?: boolean;
    onClose?: () => void;
    onBack?: () => void;
}

export default function JoinServerDialog({ isOpen, onClose, onBack }: JoinServerDialogProps) {
    const { t } = useTranslation();
    const [internalIsOpen, setInternalIsOpen] = useState(false);
    const [isLoading, setIsLoading] = useState(false);
    const [invitationCode, setInvitationCode] = useState("");
    const joinServer = useServerStore((state) => state.joinServer);
    const { showSnackbar } = useSnackbar();

    const dialogIsOpen = isOpen !== undefined ? isOpen : internalIsOpen;
    const handleClose = onClose || (() => setInternalIsOpen(false));
    const handleBack = onBack || handleClose;

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        if (!invitationCode.trim()) return;
        setIsLoading(true);
        try {
            await joinServer(invitationCode);
            handleClose();
            showSnackbar({ message: t('Join_server_dialog.join_success'), severity: "success" });
        } catch (error) {
            showSnackbar({ message: t('Join_server_dialog.join_failed'), severity: "error" });
        } finally {
            setIsLoading(false);
            setInvitationCode("");
        }
    };

    return (
        <>
            {isOpen === undefined && (
                <Button
                    style={{ borderRadius: '10px', backgroundColor: '#5865F2', color: 'white', padding: 0 }}
                    onClick={() => setInternalIsOpen(true)}
                >
                    {t('Join_server_dialog.join_server')}
                </Button>
            )}

            <Dialog
                isOpen={dialogIsOpen}
                onClose={handleClose}
                title={t('Join_server_dialog.join_server')}
            >
                <form onSubmit={handleSubmit} className="flex flex-col gap-8">
                    <Input
                        label={t('Join_server_dialog.invitation_code')}
                        type="text"
                        placeholder={t('Join_server_dialog.invitation_code_placeholder')}
                        value={invitationCode}
                        onChange={(e) => setInvitationCode(e.target.value)}
                    />

                    <div className="text-white flex justify-between">

                        <Button
                            variant={"noBackground"}
                            width="80px"
                            onClick={handleBack}>
                            {t('Join_server_dialog.back')}
                        </Button>

                        <Button
                            variant={'primary'}
                            width="200px"
                            type="submit"
                        >
                            {isLoading ? t('Join_server_dialog.joining') : t('Join_server_dialog.join_the_server')}
                        </Button>

                    </div>

                </form>
            </Dialog>
        </>
    );
}