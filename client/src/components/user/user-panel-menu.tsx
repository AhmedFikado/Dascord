'use client';

import StatusSelector from './status-selector';
import { Button } from '@/components/ui/button';
import { Settings, LogOut } from 'lucide-react';
import { Status } from '@/types/models/status';

interface UserPanelMenuProps {
    isOpen: boolean;
    onClose: () => void;
    currentStatus: Status;
    onStatusChange: (status: Status) => void;
    onSettings: () => void;
    onLogout: () => void;
}

export default function UserPanelMenu({
    isOpen,
    onClose,
    currentStatus,
    onStatusChange,
    onSettings,
    onLogout,
}: UserPanelMenuProps) {
    if (!isOpen) return null;

    return (
        <>
            <div
                className="fixed inset-0 z-40"
                onClick={onClose}
            />

            <div className="fixed bottom-20 left-3 z-50 w-72 bg-gray-300 rounded-xl shadow-lg overflow-hidden">
                <StatusSelector
                    currentStatus={currentStatus}
                    onStatusChange={onStatusChange}
                />

                <div className="flex py-1 px-3 space-between items-center justify-between bg-gray-300">
                    <Button
                        variant="noBackground"
                        onClick={onSettings}>
                        <Settings size={20} className="text-white" />
                    </Button>

                    <Button
                        variant="noBackground"
                        onClick={onLogout}
                        >
                        <LogOut size={20} className="text-red" />
                    </Button>
                </div>

            </div>
        </>
    );
}