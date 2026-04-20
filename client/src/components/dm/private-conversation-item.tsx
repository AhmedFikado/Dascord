'use client';

import { PrivateChannelWithUser } from '@/app/lib/api/private-channels';
import { useRouter, useParams } from 'next/navigation';
import { usePrivateChannelStore } from '@/app/lib/stores/use-private-channel-store';
import UserCard from '@/components/shared/user-card';
import { Status } from '@/types/models/status';
import { useTranslation } from 'react-i18next';
import { useState } from 'react';

interface PrivateConversationItemProps {
  conversation: PrivateChannelWithUser;
}

export default function PrivateConversationItem({
  conversation,
}: PrivateConversationItemProps) {
  const router = useRouter();
  const params = useParams();
  const { t } = useTranslation();
  const currentDmId = params?.dmId as string | undefined;
  const { setCurrentPrivateChannel, fetchMessages, hidePrivateChannel } = usePrivateChannelStore();
  const [isHovered, setIsHovered] = useState(false);

  const handleClick = async () => {
    setCurrentPrivateChannel(conversation);
    await fetchMessages(conversation.id);
    router.push(`/dms/${conversation.id}`);
  };

  const handleHide = async (e: React.MouseEvent) => {
    e.stopPropagation();
    await hidePrivateChannel(conversation.id);
    if (currentDmId === conversation.id) {
      router.push('/dms');
    }
  };

  const isActive = currentDmId === conversation.id;
  const recipientUser = conversation.recipient_user;

  return (
    <div
      onClick={handleClick}
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
      className={`
        w-full px-3 py-2 rounded-md flex items-center gap-3 cursor-pointer
        transition-all duration-150 group
        ${isActive ? 'bg-blurple text-white' : 'text-gray-light hover:bg-gray-700'}
      `}
      title={recipientUser?.username}
    >
      <UserCard
        username={recipientUser?.username}
        size={32}
        status={recipientUser?.status as Status | undefined}
      />

      <span className="truncate text-sm flex-1">
        {recipientUser?.username || t('DM.unknown_user')}
      </span>

      {isHovered && (
        <button
          onClick={handleHide}
          className="ml-auto text-gray-400 hover:text-white transition-colors duration-100 flex-shrink-0"
          title={t('DM.hide_conversation')}
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
            <line x1="18" y1="6" x2="6" y2="18" />
            <line x1="6" y1="6" x2="18" y2="18" />
          </svg>
        </button>
      )}
    </div>
  );
}
