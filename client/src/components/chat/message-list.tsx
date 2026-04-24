'use client';

import { useWebSocketContext } from '@/components/shared/websocket-provider';
import { useWebSocketStore } from '@/store/websocket';
import { useUnreadStore } from '@/app/lib/stores/use-unread-store';
import { MessageData } from '@/types/websocket';
import { Message } from '@/types/models/message';
import { useEffect, useRef, useCallback } from 'react';
import { useTranslation } from 'react-i18next';
import MessageItem from './message-item';

interface MessageListProps {
  channelId: string;
  messages: Message[];
  onDeleteMessage: (id: string) => void;
  onUpdateMessage: (id: string, content: string) => void;
  onAddReaction?: (messageId: string, reaction: string) => void;
  onRemoveReaction?: (messageId: string, reaction: string) => void;
}

const EMPTY_ARRAY: MessageData[] = [];

export default function MessageList({ channelId, messages, onDeleteMessage, onUpdateMessage, onAddReaction, onRemoveReaction }: MessageListProps) {
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const unreadDividerRef = useRef<HTMLDivElement>(null);

  // Tracks which channelId received its initial scroll — replaces hasInitiallyScrolled + reset effect
  const channelScrolledRef = useRef<string | null>(null);
  // Message count snapshot taken at initial scroll time, to detect truly new messages afterwards
  const prevMessagesLengthRef = useRef(0);
  // Suppresses handleScroll during programmatic scrollIntoView to avoid premature markAsRead
  const isProgrammaticScrollRef = useRef(false);

  const { joinChannel, leaveChannel } = useWebSocketContext();
  const { t } = useTranslation();

  const wsMessages = useWebSocketStore(state => state.messagesByChannel[channelId]) || EMPTY_ARRAY;
  const { getFirstUnreadMessageId, markChannelReadRemote, isChannelUnread } = useUnreadStore();
  const firstUnreadMessageId = getFirstUnreadMessageId(channelId);

  useEffect(() => {
    if (channelId) joinChannel(channelId);
    return () => {
      if (channelId) {
        leaveChannel(channelId);
        // Reset so the next visit to this channel triggers a fresh initial scroll
        channelScrolledRef.current = null;
      }
    };
  }, [channelId, joinChannel, leaveChannel]);

  // Fusionner les messages HTTP et WebSocket (les données WS ont priorité)
  const wsMessagesMap = new Map(wsMessages.map(m => [m.message_id, m]));
  const allMessages = messages.map(m => {
    const wsMsg = wsMessagesMap.get(m.id || '');
    if (!wsMsg) return m;
    return {
      ...m,
      content: wsMsg.content ?? m.content,
      reactions: wsMsg.reactions !== undefined ? wsMsg.reactions : m.reactions,
    };
  });
  wsMessages.forEach(wsMsg => {
    if (!allMessages.find(m => m.id === wsMsg.message_id)) {
      allMessages.push({
        id: wsMsg.message_id,
        channel_id: channelId,
        user_id: wsMsg.user_id,
        username: wsMsg.username,
        content: wsMsg.content,
        created_at: wsMsg.created_at,
        reactions: wsMsg.reactions,
      });
    }
  });

  const sortedMessages = allMessages.sort(
    (a, b) => new Date(a.created_at).getTime() - new Date(b.created_at).getTime()
  );

  const firstUnreadIndex = firstUnreadMessageId
    ? sortedMessages.findIndex(m => m.id === firstUnreadMessageId)
    : -1;

  // flex-col-reverse: scrollTop=0 = bottom (newest messages)
  const isAtBottom = useCallback(() => {
    const el = containerRef.current;
    if (!el) return false;
    return el.scrollTop < 50;
  }, []);

  const markAsRead = useCallback(() => {
    if (isChannelUnread(channelId)) {
      markChannelReadRemote(channelId);
    }
  }, [channelId, isChannelUnread, markChannelReadRemote]);

  const markAsReadRef = useRef(markAsRead);
  markAsReadRef.current = markAsRead;

  // Initial scroll: once per channel visit, wait for messages, then scroll to unread or bottom.
  // channelScrolledRef !== channelId handles both first visit and subsequent visits automatically.
  useEffect(() => {
    if (channelScrolledRef.current === channelId) return;
    if (sortedMessages.length === 0) return;
    // If the unread message isn't in sortedMessages yet, wait for WS MessageHistory
    if (firstUnreadMessageId !== undefined && firstUnreadIndex === -1) return;

    channelScrolledRef.current = channelId;
    prevMessagesLengthRef.current = sortedMessages.length;

    isProgrammaticScrollRef.current = true;
    requestAnimationFrame(() => { isProgrammaticScrollRef.current = false; });

    if (firstUnreadIndex !== -1 && unreadDividerRef.current) {
      unreadDividerRef.current.scrollIntoView({ behavior: 'instant', block: 'center' });
    } else {
      messagesEndRef.current?.scrollIntoView({ behavior: 'instant' });
    }
    markAsReadRef.current();
  }, [channelId, sortedMessages.length, firstUnreadIndex, firstUnreadMessageId]);

  // Auto-scroll and mark-as-read only for messages that arrive AFTER the initial scroll
  useEffect(() => {
    if (channelScrolledRef.current !== channelId) return;
    if (sortedMessages.length <= prevMessagesLengthRef.current) return;
    prevMessagesLengthRef.current = sortedMessages.length;

    if (isAtBottom()) {
      messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
      markAsRead();
    }
  }, [channelId, sortedMessages.length, isAtBottom, markAsRead]);

  // Mark as read when user manually scrolls to bottom (ignore programmatic scrolls)
  const handleScroll = useCallback(() => {
    if (isProgrammaticScrollRef.current) return;
    if (isAtBottom()) {
      markAsRead();
    }
  }, [isAtBottom, markAsRead]);

  return (
    <div
      ref={containerRef}
      className="h-full overflow-y-auto flex flex-col-reverse"
      onScroll={handleScroll}
    >
      <div className="flex flex-col">
        {sortedMessages.map((message, index) => (
          <div key={message.id}>
            {firstUnreadIndex !== -1 && index === firstUnreadIndex && (
              <div ref={unreadDividerRef} className="flex items-center my-2 mx-4">
                <div className="flex-1 h-px bg-red-500" />
                <span className="px-3 text-xs font-semibold text-red-500">
                  {t('Message_list.unread_marker')}
                </span>
                <div className="flex-1 h-px bg-red-500" />
              </div>
            )}
            <MessageItem
              message={message}
              onDelete={() => onDeleteMessage(message.id || '')}
              onUpdate={(content) => onUpdateMessage(message.id || '', content)}
              onAddReaction={onAddReaction}
              onRemoveReaction={onRemoveReaction}
            />
          </div>
        ))}
        <div ref={messagesEndRef} />
      </div>
    </div>
  );
}
