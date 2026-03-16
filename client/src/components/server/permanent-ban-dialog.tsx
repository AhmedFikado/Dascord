import { Button } from '@/components/ui/button';
import { Dialog } from '@/components/ui/dialog';

interface PermanentBanDialogProps {
  isOpen: boolean;
  onClose: () => void;
  onConfirm: () => void;
}

export default function PermanentBanDialog({
  isOpen,
  onClose,
  onConfirm,
}: PermanentBanDialogProps) {
  return (
    <Dialog isOpen={isOpen} onClose={onClose} title="Bannir définitivement">
      <div className="space-y-4">
        <p className="text-white text-sm">
          Etes-vous sûr de bannir définitivement cette personne ? Cette action est irréversible.
        </p>
        <div className="flex gap-4 justify-end mt-6">
          <Button variant="danger" onClick={onConfirm}>
            Oui
          </Button>
          <Button variant="secondary" onClick={onClose}>
            Non
          </Button>
        </div>
      </div>
    </Dialog>
  );
}
