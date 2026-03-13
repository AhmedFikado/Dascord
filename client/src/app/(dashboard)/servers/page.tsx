'use client';

import { useMobileNav } from '@/app/(dashboard)/layout';
import { Menu } from 'lucide-react';
import { useTranslation } from 'react-i18next';

export default function ServersPage() {

    const { openNav } = useMobileNav();
    const { t } = useTranslation();

    return (
        <div className="h-full flex items-center justify-center bg-background">

            <button
                className="md:hidden mr-3 p-1 flex-shrink-0 absolute top-4 left-4 z-10"
                onClick={openNav}
                aria-label={t('open_navigation')}
            >
                <Menu size={20} className="text-white" />
            </button>

            <div className="text-center">
                <h2 className="text-2xl font-semibold text-gray-50 mb-2">
                    {t('Nothing_server_selected')}
                </h2>
                <p className="text-gray-100">
                    {t('Select_a_server_from_the_sidebar_to_get_started')}
                </p>
            </div>
        </div>
    );
}