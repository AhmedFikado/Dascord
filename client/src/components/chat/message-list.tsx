import MessageItem from './message-item';
import { Message } from '@/types/models/message';
import { Button } from '../ui/button';

interface MessageListProps {
    messages: Message[];
    onDeleteMessage: (id: string) => void;
}

export default function MessageList({ messages, onDeleteMessage }: MessageListProps) {
    return (
        <div className="h-full overflow-y-auto flex flex-col-reverse">
            <div className="flex flex-col">
                {messages.map((message) => (
                    <MessageItem
                        key={message.id}
                        message={message}
                        onDelete={() => onDeleteMessage(message.id || '')}
                    />
                ))}
            </div>
        </div>
    );
}