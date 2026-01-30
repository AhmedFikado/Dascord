'use client';

import { useState } from 'react';
import TypingIndicator from './typing-indicator';
import { Status, User } from '@/types/models/user';

interface MessageInputProps {
    channelName?: string;
    onSendMessage?: (message: string) => void;
}

export default function MessageInput({ onSendMessage }: MessageInputProps) {
    const [message, setMessage] = useState('');

    const user: User = {
        id: 1,
        username: 'Alice',
        email: 'alice@gmail.com',
        created_at: new Date(),
        status: Status.ONLINE,
    };

    const websocket = {
        typing: true,
    };



    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        if (message.trim()) {
            onSendMessage?.(message);
            setMessage('');
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

            {websocket.typing && <TypingIndicator user={user} />}

            <form onSubmit={handleSubmit}>
                <div className="flex items-center bg-gray-400 rounded-xl px-4">
                    <input
                        type="text"
                        value={message}
                        onChange={(e) => setMessage(e.target.value)}
                        onKeyDown={handleKeyDown}
                        placeholder={`Envoyez un message`}
                        className="w-full bg-gray-400 py-3 pl-2 text-white placeholder-gray-50 focus:outline-none"
                    />
                </div>
            </form>
        </div>
    );
}