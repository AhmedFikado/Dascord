'use client';

import ServerList from "../server/server-list";
import { useServers } from '@/app/lib/hooks/use-servers';
import { useUnreadStore } from '@/app/lib/stores/use-unread-store';
import JoinCreateServer from '@/components/server/Join-create-server-dialog';
import { useRouter, usePathname } from 'next/navigation';
import { useTranslation } from 'react-i18next';

export default function Sidebar() {
    const { servers } = useServers();
    const router = useRouter();
    const pathname = usePathname();
    const { t } = useTranslation();
    const isDMActive = pathname?.startsWith('/dms');

    const unreadChannels = useUnreadStore((state) => state.unreadChannels);
    const channelToServer = useUnreadStore((state) => state.channelToServer);
    const hasUnreadDMs = Object.keys(unreadChannels).some(
        (cid) => channelToServer[cid] === 'private'
    );

    return (
        <aside className="w-18 h-full bg-backgroundSide flex flex-col items-center py-3 gap-2 overflow-y-auto scrollbar-hide flex-shrink-0 border-r border-gray-200">
            <div className="relative mb-1 flex-shrink-0">
                <button
                    onClick={() => router.push('/dms')}
                    title={t('DM.private_messages')}
                    className={`
                        w-12 h-12 rounded-2xl overflow-hidden
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
                {hasUnreadDMs && !isDMActive && (
                    <span className="absolute bottom-0 right-0 w-3.5 h-3.5 bg-red-500 rounded-full border-2 border-backgroundSide" />
                )}
            </div>
            <div className="w-10 h-0.5 bg-gray-300 rounded"></div>
            <ServerList servers={servers} />
            <JoinCreateServer />
        </aside>
    );
}
