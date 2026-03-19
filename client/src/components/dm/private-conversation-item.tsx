'use client';

import { PrivateChannelWithUser } from '@/app/lib/api/private-channels';
import { useRouter, useParams } from 'next/navigation';
import { usePrivateChannelStore } from '@/app/lib/stores/use-private-channel-store';
import UserCard from '@/components/shared/user-card';
import { Status } from '@/types/models/user';
import { useTranslation } from 'react-i18next';

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
  const { setCurrentPrivateChannel, fetchMessages } = usePrivateChannelStore();

  const handleClick = async () => {
    setCurrentPrivateChannel(conversation);
    await fetchMessages(conversation.id);
    router.push(`/dms/${conversation.id}`);
  };

  const isActive = currentDmId === conversation.id;
  const recipientUser = conversation.recipient_user;

  return (
    <div
      onClick={handleClick}
      className={`
        w-full px-3 py-2 rounded-md flex items-center gap-3 cursor-pointer
        transition-all duration-150
        ${isActive ? 'bg-blurple text-white' : 'text-gray-light hover:bg-gray-700'}
      `}
      title={recipientUser?.username}
    >
      <UserCard
        username={recipientUser?.username}
        size={32}
        status={recipientUser?.status as Status | undefined}
      />

      <span className="truncate text-sm">
        {recipientUser?.username || t('DM.unknown_user')}
      </span>
    </div>
  );
}
