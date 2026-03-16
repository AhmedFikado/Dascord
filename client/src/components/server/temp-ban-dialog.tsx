import { Button } from '@/components/ui/button';
import { Dialog } from '@/components/ui/dialog';
import { Dropdown } from '@/components/ui/dropdown';
import { useState } from 'react';

interface TempBanDialogProps {
  isOpen: boolean;
  onClose: () => void;
  onConfirm: (duration: string) => void;
}

export default function TempBanDialog({
  isOpen,
  onClose,
  onConfirm,
}: TempBanDialogProps) {
  const [duration, setDuration] = useState('1h');

  const options = [
    { label: '1 heure', value: '1h' },
    { label: '10 heures', value: '10h' },
    { label: '24 heures', value: '24h' },
    { label: '48 heures', value: '48h' },
    { label: '1 semaine', value: '1w' },
  ];

  return (
    <Dialog isOpen={isOpen} onClose={onClose} title="Bannir temporairement">
      <div className="space-y-4">
        <p className="text-white text-sm">Veuillez choisir une période de ban</p>
        <Dropdown options={options} value={duration} onChange={val => setDuration(val)} />
        <div className="flex gap-4 justify-end mt-6">
          <Button variant="secondary" onClick={onClose}>
            Non
          </Button>
          <Button variant="danger" onClick={() => onConfirm(duration)}>
            Oui
          </Button>
        </div>
      </div>
    </Dialog>
  );
}
