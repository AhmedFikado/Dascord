'use client';

import { useState } from 'react';
import UserPanelBar from './user-panel-bar';
import UserPanelMenu from './user-panel-menu';
import { Status } from '@/types/models/status';
import { Dialog } from '../ui/dialog';
import UserSetting from './user-setting';
import { User } from '@/types/models/user';
import { useAuthStore } from '@/app/lib/stores/use-auth-store';
import { useRouter } from 'next/navigation';
import { useCurrentUser } from '@/app/lib/hooks/use-current-user';

export default function UserPanel() {
    const [isOpen, setIsOpen] = useState(false);
    const [status, setStatus] = useState(Status.ONLINE);
    const [isSettingsOpen, setIsSettingsOpen] = useState(false);
    const [isMenuOpen, setIsMenuOpen] = useState(false);
    const router = useRouter();
    const authStore = useAuthStore();
    const { user } = useCurrentUser();

    const handleStatusChange = (newStatus: Status) => {
        setStatus(newStatus);
        setIsOpen(false);
    };

    const handleLogout = () => {
        setIsOpen(false);
        authStore.logout();
        router.push('/login');
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

            {user && (
                <>
                    <UserPanelBar
                        username={user.username}
                        status={status}
                        onClick={() => setIsOpen(!isOpen)}
                    />

                    <Dialog
                        isOpen={isSettingsOpen}
                        onClose={() => setIsSettingsOpen(false)}
                        title="Paramètres utilisateur"
                        size="xl"
                    >
                        <UserSetting user={user} />
                    </Dialog>
                </>
            )}
        </>
    );
}