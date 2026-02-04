'use client';

import { useWebSocketContext } from '@/components/shared/websocket-provider';
import { useWebSocketStore } from '@/store/websocket';
import { Message } from '@/types/models/message';
import { useEffect, useRef } from 'react';
import MessageItem from './message-item';

interface MessageListProps {
  channelId: string;
}

export default function MessageList({ channelId }: MessageListProps) {
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const { joinChannel, leaveChannel } = useWebSocketContext();
  const messages = useWebSocketStore(state => state.messagesByChannel[channelId] || []);
  // Joindre le channel au montage
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

  // Auto-scroll vers le bas
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  // Convertir MessageData en Message pour compatibilité
  const formattedMessages: Message[] = messages.map(msg => ({
    id: msg.message_id,
    channel_id: channelId,
    user_id: msg.user_id,
    content: msg.content,
    created_at: new Date(msg.created_at),
    user: {
      id: msg.user_id,
      username: msg.username,
      email: '',
      created_at: new Date(),
      status: 0,
    },
  }));

  return (
    <div className="h-full overflow-y-auto flex flex-col-reverse">
      <div className="flex flex-col">
        {formattedMessages.map(message => (
          <MessageItem key={message.id} message={message} />
        ))}
        <div ref={messagesEndRef} />
      </div>
    </div>
  );
}
