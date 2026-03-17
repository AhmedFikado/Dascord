'use client';

import { useWebSocketContext } from '@/components/shared/websocket-provider';
import { useWebSocketStore } from '@/store/websocket';
import { useCallback, useEffect, useRef, useState } from 'react';
import TypingIndicator from './typing-indicator';
import { useTranslation } from 'react-i18next';
import GifButton from './gif-button';

interface MessageInputProps {
  channelId: string;
  channelName?: string;
}

interface TypingUser {
  user_id: string;
  username: string;
  timestamp: number;
}

const TYPING_DEBOUNCE = 1000; // 1 seconde
const EMPTY_TYPING_ARRAY: TypingUser[] = [];

export default function MessageInput({ channelId, channelName }: MessageInputProps) {
  const [message, setMessage] = useState('');
  const isTypingRef = useRef(false);
  const typingTimeoutRef = useRef<NodeJS.Timeout | undefined>(undefined);
  const { t } = useTranslation();

  const { sendChannelMessage, sendTyping } = useWebSocketContext();
  const typingUsers =
    useWebSocketStore(state => state.typingByChannel[channelId]) || EMPTY_TYPING_ARRAY;

  // Arrêter l'indicateur de saisie après un certain temps
  const stopTyping = useCallback(() => {
    if (isTypingRef.current) {
      sendTyping(channelId, false);
      isTypingRef.current = false;
    }
  }, [channelId, sendTyping]);

  // Démarrer l'indicateur de saisie
  const startTyping = useCallback(() => {
    if (!isTypingRef.current) {
      sendTyping(channelId, true);
      isTypingRef.current = true;
    }

    // Réinitialiser le timeout
    if (typingTimeoutRef.current) {
      clearTimeout(typingTimeoutRef.current);
    }

    typingTimeoutRef.current = setTimeout(() => {
      stopTyping();
    }, TYPING_DEBOUNCE);
  }, [channelId, sendTyping, stopTyping]);

  // Nettoyer le timeout au démontage
  useEffect(() => {
    return () => {
      if (typingTimeoutRef.current) {
        clearTimeout(typingTimeoutRef.current);
      }
      sendTyping(channelId, false);
    };
  }, [channelId, sendTyping]);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (message.trim()) {
      sendChannelMessage(channelId, message);
      setMessage('');
      stopTyping();
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

      <form onSubmit={handleSubmit} className="relative">
        <div className="flex items-center bg-gray-400 rounded-xl px-4">
          <input
            type="text"
            value={message}
            onChange={handleChange}
            onKeyDown={handleKeyDown}
            placeholder={t('Message_input.Type_your_message_here...')}
            className="w-full bg-gray-400 py-3 pl-2 text-white placeholder-gray-50 focus:outline-none"
          />
          <GifButton channelId={channelId} />
        </div>
      </form>
    </div>
  );
}
