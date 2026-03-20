'use client';

import { useWebSocketContext } from '@/components/shared/websocket-provider';
import { useWebSocketStore } from '@/store/websocket';
import { useCallback, useEffect, useRef, useState } from 'react';
import TypingIndicator from './typing-indicator';
import GifButton from './gif-button';
import { useTranslation } from 'react-i18next';

interface PrivateMessageInputProps {
  channelId: string;
  onSend: (content: string) => Promise<void>;
  placeholder?: string;
}

interface TypingUser {
  user_id: string;
  username: string;
  timestamp: number;
}

const TYPING_DEBOUNCE = 1000;
const EMPTY_TYPING_ARRAY: TypingUser[] = [];

export default function PrivateMessageInput({
  channelId,
  onSend,
  placeholder,
}: PrivateMessageInputProps) {
  const { t } = useTranslation();
  const resolvedPlaceholder = placeholder ?? t('DM.input_placeholder');
  const [message, setMessage] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const isTypingRef = useRef(false);
  const typingTimeoutRef = useRef<NodeJS.Timeout | undefined>(undefined);

  const { sendTyping } = useWebSocketContext();
  const typingUsers =
    useWebSocketStore(state => state.typingByChannel[channelId]) || EMPTY_TYPING_ARRAY;

  const stopTyping = useCallback(() => {
    if (isTypingRef.current) {
      sendTyping(channelId, false);
      isTypingRef.current = false;
    }
  }, [channelId, sendTyping]);

  const startTyping = useCallback(() => {
    if (!isTypingRef.current) {
      sendTyping(channelId, true);
      isTypingRef.current = true;
    }
    if (typingTimeoutRef.current) {
      clearTimeout(typingTimeoutRef.current);
    }
    typingTimeoutRef.current = setTimeout(stopTyping, TYPING_DEBOUNCE);
  }, [channelId, sendTyping, stopTyping]);

  useEffect(() => {
    return () => {
      if (typingTimeoutRef.current) clearTimeout(typingTimeoutRef.current);
      sendTyping(channelId, false);
    };
  }, [channelId, sendTyping]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (message.trim() && !isLoading) {
      setIsLoading(true);
      try {
        await onSend(message.trim());
        setMessage('');
        stopTyping();
      } catch (error) {
        console.error("Erreur lors de l'envoi du message:", error);
      } finally {
        setIsLoading(false);
      }
    }
  };

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setMessage(e.target.value);
    if (e.target.value.length > 0) {
      startTyping();
    } else {
      stopTyping();
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSubmit(e);
    }
  };

  return (
    <div className="px-4 py-2 flex-shrink-0 w-full">
      {typingUsers.length > 0 && (
        <div className="mb-2">
          {typingUsers.map(user => (
            <TypingIndicator key={user.user_id} username={user.username} />
          ))}
        </div>
      )}

      <form onSubmit={handleSubmit}>
        <div className="flex items-center bg-gray-400 rounded-xl px-4 gap-2">
          <input
            type="text"
            value={message}
            onChange={handleChange}
            onKeyDown={handleKeyDown}
            placeholder={resolvedPlaceholder}
            disabled={isLoading}
            className="w-full bg-gray-400 py-3 pl-2 text-white placeholder-gray-50 focus:outline-none disabled:opacity-50"
          />
          <GifButton channelId={channelId} />
          <button
            type="submit"
            disabled={!message.trim() || isLoading}
            className="flex-shrink-0 px-3 py-2 bg-blurple text-white rounded-lg hover:opacity-90 disabled:opacity-50 transition-opacity"
          >
            {isLoading ? '...' : t('DM.send_button')}
          </button>
        </div>
      </form>
    </div>
  );
}
