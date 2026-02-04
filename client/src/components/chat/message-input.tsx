'use client';

import { useWebSocketContext } from '@/components/shared/websocket-provider';
import { useWebSocketStore } from '@/store/websocket';
import { useCallback, useEffect, useRef, useState } from 'react';
import TypingIndicator from './typing-indicator';

interface MessageInputProps {
  channelId: string;
  channelName?: string;
}

const TYPING_DEBOUNCE = 1000; // 1 seconde

export default function MessageInput({ channelId, channelName }: MessageInputProps) {
  const [message, setMessage] = useState('');
  const [isTyping, setIsTyping] = useState(false);
  const typingTimeoutRef = useRef<NodeJS.Timeout | undefined>(undefined);

  const { sendChannelMessage, sendTyping } = useWebSocketContext();
  const typingUsers = useWebSocketStore(state => state.typingByChannel[channelId] || []);

  // Arrêter l'indicateur de saisie après un certain temps
  const stopTyping = useCallback(() => {
    if (isTyping) {
      sendTyping(channelId, false);
      setIsTyping(false);
    }
  }, [channelId, isTyping, sendTyping]);

  // Démarrer l'indicateur de saisie
  const startTyping = useCallback(() => {
    if (!isTyping) {
      sendTyping(channelId, true);
      setIsTyping(true);
    }

    // Réinitialiser le timeout
    if (typingTimeoutRef.current) {
      clearTimeout(typingTimeoutRef.current);
    }

    typingTimeoutRef.current = setTimeout(() => {
      stopTyping();
    }, TYPING_DEBOUNCE);
  }, [channelId, isTyping, sendTyping, stopTyping]);

  // Nettoyer le timeout au démontage
  useEffect(() => {
    return () => {
      if (typingTimeoutRef.current) {
        clearTimeout(typingTimeoutRef.current);
      }
      if (isTyping) {
        sendTyping(channelId, false);
      }
    };
  }, [channelId, isTyping, sendTyping]);

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

      <form onSubmit={handleSubmit}>
        <div className="flex items-center bg-gray-400 rounded-xl px-4">
          <input
            type="text"
            value={message}
            onChange={handleChange}
            onKeyDown={handleKeyDown}
            placeholder={`Envoyez un message${channelName ? ` dans #${channelName}` : ''}`}
            className="w-full bg-gray-400 py-3 pl-2 text-white placeholder-gray-50 focus:outline-none"
          />
        </div>
      </form>
    </div>
  );
}
