'use client';

import { useState } from 'react';
import UserPanelBar from './user-panel-bar';
import UserPanelMenu from './user-panel-menu';
import { Status } from '@/types/models/status';

export default function UserPanel() {
    const [isOpen, setIsOpen] = useState(false);
    const [status, setStatus] = useState(Status.ONLINE);

    const handleStatusChange = (newStatus: Status) => {
        setStatus(newStatus);
        setIsOpen(false);
        console.log('Nouveau statut:', newStatus);
    };

    const handleLogout = () => {
        setIsOpen(false);
    };

    const handleSettings = () => {
        setIsOpen(false);
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
        </>
    );
}