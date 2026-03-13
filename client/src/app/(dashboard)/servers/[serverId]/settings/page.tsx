'use client';

import { use } from 'react';
import { useTranslation } from 'react-i18next';

export default function ServerSettingsPage({
    params
}: {
    params: Promise<{ serverId: string }>
}) {
    const { t } = useTranslation();

    return (
        <div className="flex-1 p-8">
            <h1 className="text-white text-2xl font-bold mb-4">
                {t('Settings_server_page.Server_settings')}
            </h1>
        </div>
    );
}