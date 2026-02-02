import MessageItem from './message-item';
import { Message } from '@/types/models/message';

export default function MessageList({ messages }: { messages: Message[] }) {
    return (
        <div className="h-full overflow-y-auto flex flex-col-reverse">
            <div className="flex flex-col">
                {messages.map((message) => (
                    <MessageItem key={message.id} message={message} />
                ))}
            </div>
        </div>
    );
}