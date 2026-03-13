'use client';

import { use, useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';
import { channelsApi } from '@/app/lib/api/channels';
import { Loading } from '@/components/shared/loading-spinner';
import { useTranslation } from 'react-i18next';

export default function ServerPage({
    params
}: {
    params: Promise<{ serverId: string }>
}) {
    const { t } = useTranslation();
    const { serverId } = use(params);
    const router = useRouter();
    const [isLoading, setIsLoading] = useState(true);

    useEffect(() => {
        const redirectToFirstChannel = async () => {
            try {
                const channels = await channelsApi.getByServer(serverId);

                if (channels.length > 0) {
                    router.push(`/servers/${serverId}/channels/${channels[0].id}`);
                } else {
                    // Si pas de channels, rester sur la page du serveur
                    setIsLoading(false);
                }
            } catch (error) {
                console.error('Error fetching channels:', error);
                setIsLoading(false);
            }
        };

        redirectToFirstChannel();
    }, [serverId, router]);

    if (isLoading) {
        return (
            <div className="flex items-center justify-center h-screen">
                <Loading size="lg" />
            </div>
        );
    }

    return (
        <div className="flex items-center justify-center h-screen">
            <p className="text-gray-500">{t('Server_page.Nothing_server_selected')}</p>
        </div>
    );
}