'use client';

import { Menu } from 'lucide-react';
import { useMobileNav } from '@/app/(dashboard)/layout';
import type { PrivateChannelWithUser } from '../../types/models/privateChannels';
import UserCard from '@/components/shared/user-card';
import { Status } from '@/types/models/status';
import { useTranslation } from 'react-i18next';

interface DMPageHeaderProps {
  conversation?: PrivateChannelWithUser | null;
}

export default function DMPageHeader({ conversation }: DMPageHeaderProps) {
  const { openNav } = useMobileNav();
  const { t } = useTranslation();
  const recipientUser = conversation?.recipient_user;

  return (
    <header className="h-[61px] bg-background flex items-center px-4 border-b border-gray-200 flex-shrink-0 w-full">
      <button
        className="md:hidden mr-3 p-1 flex-shrink-0"
        onClick={openNav}
        aria-label={t('DM.open_navigation')}
      >
        <Menu size={20} className="text-white" />
      </button>

      {recipientUser ? (
        <div className="flex items-center gap-3 flex-1 min-w-0">
          <UserCard
            username={recipientUser.username}
            size={32}
            status={recipientUser.status as Status | undefined}
            avatarId={recipientUser.avatar_id}
          />
          <span className="font-semibold text-white truncate">
            {recipientUser.username}
          </span>
        </div>
      ) : (
        <h1 className="text-white font-semibold flex-1">{t('DM.private_messages')}</h1>
      )}
    </header>
  );
}
