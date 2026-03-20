'use client';

import { usePrivateChannelStore } from '@/app/lib/stores/use-private-channel-store';
import { useEffect } from 'react';
import PrivateConversationList from './private-conversation-list';
import { useTranslation } from 'react-i18next';

export default function DMSidebar() {
  const { privateChannels, fetchPrivateChannels } = usePrivateChannelStore();
  const { t } = useTranslation();

  useEffect(() => {
    fetchPrivateChannels();
  }, [fetchPrivateChannels]);

  return (
    <aside className="w-60 h-full flex flex-col bg-backgroundSide overflow-y-auto scrollbar-hide flex-shrink-0">
      <div className="p-4 border-b border-gray-200 flex-shrink-0">
        <h2 className="font-semibold text-white">{t('DM.private_messages')}</h2>
      </div>
      <div className="flex-1 overflow-y-auto px-3 py-3">
        <PrivateConversationList conversations={privateChannels} />
      </div>
    </aside>
  );
}
