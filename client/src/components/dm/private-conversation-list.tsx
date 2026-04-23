'use client';

import { PrivateChannelWithUser } from '../../types/models/privateChannels';

import PrivateConversationItem from './private-conversation-item';
import { useTranslation } from 'react-i18next';

interface PrivateConversationListProps {
  conversations: PrivateChannelWithUser[];
}

export default function PrivateConversationList({
  conversations,
}: PrivateConversationListProps) {
  const { t } = useTranslation();
  return (
    <nav className="">
      <ul className="space-y-1">
        {conversations.length === 0 ? (
          <li className="px-3 py-2 text-center text-gray-light text-sm">
            {t('DM.no_conversation')}
          </li>
        ) : (
          conversations.map((conversation) => (
            <li key={conversation.id}>
              <PrivateConversationItem conversation={conversation} />
            </li>
          ))
        )}
      </ul>
    </nav>
  );
}
