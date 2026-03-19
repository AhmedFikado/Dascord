'use client';

import { usePrivateChannelStore } from '@/app/lib/stores/use-private-channel-store';
import { useParams } from 'next/navigation';
import { useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';
import MessageList from '@/components/chat/message-list';
import PrivateMessageInput from '@/components/chat/private-message-input';
import DMPageHeader from '@/components/dm/dm-page-header';

export default function DMPage() {
  const params = useParams();
  const dmId = params?.dmId as string;
  const {
    privateChannels,
    currentPrivateChannel,
    messagesByChannel,
    fetchMessages,
    sendMessage,
    deleteMessage,
    updateMessage,
    setCurrentPrivateChannel,
  } = usePrivateChannelStore();

  const { t } = useTranslation();
  const [isLoading, setIsLoading] = useState(true);

  // Restore currentPrivateChannel from the list when navigating directly to a DM URL
  useEffect(() => {
    if (dmId && !currentPrivateChannel && privateChannels.length > 0) {
      const channel = privateChannels.find(ch => ch.id === dmId);
      if (channel) {
        setCurrentPrivateChannel(channel);
      }
    }
  }, [dmId, currentPrivateChannel, privateChannels, setCurrentPrivateChannel]);

  useEffect(() => {
    const loadMessages = async () => {
      if (dmId) {
        setIsLoading(true);
        await fetchMessages(dmId);
        setIsLoading(false);
      }
    };
    loadMessages();
  }, [dmId, fetchMessages]);

  const messages = messagesByChannel[dmId] || [];
  // Toujours lire depuis privateChannels pour avoir le statut à jour (mis à jour par WS)
  const channel = privateChannels.find(ch => ch.id === dmId) ?? currentPrivateChannel;

  return (
    <div className="flex-1 flex flex-col bg-background h-full min-w-0">
      <DMPageHeader conversation={channel} />
      <div className="flex-1 overflow-hidden flex flex-col">
        {isLoading ? (
          <div className="flex items-center justify-center h-full">
            <p className="text-gray-light">{t('DM.loading_messages')}</p>
          </div>
        ) : (
          <MessageList
            channelId={dmId}
            messages={messages}
            onDeleteMessage={(messageId) => deleteMessage(dmId, messageId)}
            onUpdateMessage={(messageId, content) => updateMessage(dmId, messageId, content)}
          />
        )}
      </div>
      <PrivateMessageInput channelId={dmId} onSend={(content) => sendMessage(dmId, content)} />
    </div>
  );
}
