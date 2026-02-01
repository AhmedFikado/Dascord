'use client';

import { useState } from 'react';
import UserPanelBar from './user-panel-bar';
import UserPanelMenu from './user-panel-menu';
import { Status } from '@/types/models/status';
import { Dialog } from '../ui/dialog';
import UserSetting from './user-setting';
import { User } from '@/types/models/user';

const userAlice: User = {
    id: 1,
    username: 'Alice',
    email: 'alice@gmail.com',
    created_at: new Date(),
    status: Status.ONLINE,
};

export default function UserPanel() {
    const [isOpen, setIsOpen] = useState(false);
    const [status, setStatus] = useState(Status.ONLINE);
    const [isSettingsOpen, setIsSettingsOpen] = useState(false);
    const [isMenuOpen, setIsMenuOpen] = useState(false);

    const handleStatusChange = (newStatus: Status) => {
        setStatus(newStatus);
        setIsOpen(false);
        console.log('Nouveau statut:', newStatus);
    };

    const handleLogout = () => {
        setIsOpen(false);
        //à faire
    };

    const handleSettings = () => {
        setIsMenuOpen(false);
        setIsSettingsOpen(true);
    };

    return (
        <>
            <UserPanelMenu
                isOpen={isOpen}
                onClose={() => setIsOpen(false)}
                currentStatus={status}
                onStatusChange={handleStatusChange}
                onSettings={handleSettings}
                onLogout={handleLogout}
            />

            <UserPanelBar
                username="Alice"
                status={status}
                onClick={() => setIsOpen(!isOpen)}
            />

            <Dialog
                isOpen={isSettingsOpen}
                onClose={() => setIsSettingsOpen(false)}
                title="Paramètres utilisateur"
                size="xl"
            >
                <UserSetting user={userAlice} />
            </Dialog>
        </>
    );
}