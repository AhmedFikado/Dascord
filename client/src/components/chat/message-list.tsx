'use client';

import { useWebSocketContext } from '@/components/shared/websocket-provider';
import { useWebSocketStore } from '@/store/websocket';
import { Message } from '@/types/models/message';
import { useEffect, useRef } from 'react';
import MessageItem from './message-item';

interface MessageListProps {
  channelId: string;
  messages: Message[];
  onDeleteMessage: (id: string) => void;
}

interface WsMessage {
  message_id: string;
  channel_id: string;
  user_id: string;
  username: string;
  content: string;
  created_at: string;
}

const EMPTY_ARRAY: WsMessage[] = [];

export default function MessageList({ channelId, messages, onDeleteMessage }: MessageListProps) {
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const { joinChannel, leaveChannel } = useWebSocketContext();

  // Messages WebSocket en temps réeel
  const wsMessages = useWebSocketStore(state => state.messagesByChannel[channelId]) || EMPTY_ARRAY;

  // Joindre le channel au montage pour recevoir les messages en temps réel
  useEffect(() => {
    if (channelId) {
      joinChannel(channelId);
    }

    return () => {
      if (channelId) {
        leaveChannel(channelId);
      }
    };
  }, [channelId, joinChannel, leaveChannel]);

  // Fusionner les messages HTTP et WebSocket (éviter les doublons par ID)
  const allMessages = [...messages];
  wsMessages.forEach(wsMsg => {
    if (!allMessages.find(m => m.id === wsMsg.message_id)) {
      allMessages.push({
        id: wsMsg.message_id,
        channel_id: channelId,
        user_id: wsMsg.user_id,
        username: wsMsg.username,
        content: wsMsg.content,
        created_at: wsMsg.created_at,
      });
    }
  });

  // Trier par date
  const sortedMessages = allMessages.sort(
    (a, b) => new Date(a.created_at).getTime() - new Date(b.created_at).getTime()
  );

  // Auto-scroll vers le bas quand de nouveaux messages arrivent
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [sortedMessages.length]);

  return (
    <div className="h-full overflow-y-auto flex flex-col-reverse">
      <div className="flex flex-col">
        {sortedMessages.map(message => (
          <MessageItem
            key={message.id}
            message={message}
            onDelete={() => onDeleteMessage(message.id || '')}
          />
        ))}
        <div ref={messagesEndRef} />
      </div>
    </div>
  );
}
