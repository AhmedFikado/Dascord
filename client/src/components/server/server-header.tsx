'use client';

import { Menu, Users } from 'lucide-react';
import { useMobileNav } from '@/app/(dashboard)/layout';
import { useTranslation } from 'react-i18next';

export default function ServerHeader({ channelName }: { channelName: string }) {
    const { t } = useTranslation();
    const { openNav, openMembers } = useMobileNav();

    return (
        <header className="h-[61px] bg-background flex items-center px-4 border-b border-gray-200 flex-shrink-0 w-full">
            <button
                className="md:hidden mr-3 p-1 flex-shrink-0"
                onClick={openNav}
                aria-label={t('Server_header.open_navigation')}
            >
                <Menu size={20} className="text-white" />
            </button>

            <h1 className="text-white text-lg font-bold flex-1">#{channelName}</h1>

            <button
                className="lg:hidden ml-3 p-1 flex-shrink-0"
                onClick={openMembers}
                aria-label={t('Server_header.show_members')}
            >
                <Users size={20} className="text-white" />
            </button>
        </header>
    );
}
