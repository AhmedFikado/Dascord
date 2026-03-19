'use client';

import ServerList from "../server/server-list";
import { useServers } from '@/app/lib/hooks/use-servers';
import JoinCreateServer from '@/components/server/Join-create-server-dialog';
import { useRouter, usePathname } from 'next/navigation';
import { useTranslation } from 'react-i18next';

export default function Sidebar() {
    const { servers } = useServers();
    const router = useRouter();
    const pathname = usePathname();
    const { t } = useTranslation();
    const isDMActive = pathname?.startsWith('/dms');

    return (
        <aside className="w-18 h-full bg-backgroundSide flex flex-col items-center py-3 gap-2 overflow-y-auto scrollbar-hide flex-shrink-0 border-r border-gray-200">
            <button
                onClick={() => router.push('/dms')}
                title={t('DM.private_messages')}
                className={`
                    w-12 h-12 flex-shrink-0 mb-1 rounded-2xl overflow-hidden
                    transition-all duration-200 cursor-pointer
                    ${isDMActive
                    ? 'ring-2 ring-blurple ring-offset-2 ring-offset-backgroundSide'
                    : 'hover:ring-2 hover:ring-blurple hover:ring-offset-2 hover:ring-offset-backgroundSide'
                }
                `}
            >
                <img
                    src="/icons/logo.png"
                    alt={t('DM.private_messages')}
                    className="w-full h-full object-contain"
                />
            </button>
            <div className="w-10 h-0.5 bg-gray-300 rounded"></div>
            <ServerList servers={servers} />
            <JoinCreateServer />
        </aside>
    );
}
