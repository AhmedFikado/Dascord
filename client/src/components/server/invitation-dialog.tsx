import { Share2 } from 'lucide-react';
import { ClipboardCheck } from 'lucide-react';
import { Dialog } from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { useState } from "react";
import { Input } from '../ui/input';
import { useSnackbar } from "@/components/shared/error-message";
import { Server } from "@/types/models/Server";
import { useTranslation } from 'react-i18next';

interface InvitationDialogProps {
    server: Server;
}

export default function InvitationDialog({ server }: InvitationDialogProps) {

    const { t } = useTranslation();
    const [isDialogOpen, setIsDialogOpen] = useState(false);
    const { showSnackbar } = useSnackbar();

    const copyToClipboard = async (text: string) => {
        try {
            await navigator.clipboard.writeText(text);
            showSnackbar({ message: t('Invitation_dialog.code_copied'), severity: "success" });
        } catch (err) {
            showSnackbar({ message: t('Invitation_dialog.copy_failed'), severity: "error" });
        }
    };

    const invitationCode = server.invitation_code;

    return (

        <>
            <Button width="28px"
                height="28px"
                style={{ borderRadius: '10px', backgroundColor: '#5865F2', color: 'white', padding: 0 }}
                onClick={() => setIsDialogOpen(true)}>
                <Share2 size={12} />
            </Button>

            <Dialog
                isOpen={isDialogOpen}
                onClose={() => setIsDialogOpen(false)}
                title={t('Invitation_dialog.invitation_code')}

            >
                <Input
                    type="text"
                    value={invitationCode}
                    readOnly
                    rightIcon={
                        <Button onClick={() => copyToClipboard(invitationCode)}
                            variant="nothing">
                            <ClipboardCheck size={16} className="cursor-pointer text-white" />
                        </Button>
                    }
                />

            </Dialog>
        </>

    );
}