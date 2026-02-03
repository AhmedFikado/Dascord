import { Message } from '../../types/models/message';
import UserCard from '../shared/user-card';

export default function MessageItem({ message }: { message: Message }) {

    const formatDate = (dateStr: string) => {
        return new Intl.DateTimeFormat('fr-FR', {
            day: '2-digit',
            month: '2-digit',
            year: 'numeric',
            hour: '2-digit',
            minute: '2-digit'
        }).format(new Date(dateStr));
    };

    return (
        <div className="flex gap-4 px-4 py-2 hover:bg-gray-400/30 group">

            <UserCard username={message.username} />

            <div className="flex-1 min-w-0">
                <div className="flex items-baseline gap-2 mb-0.5">
                    <span className="font-semibold text-white cursor-pointer">
                        {message.username}
                    </span>

                    <span className="text-xs text-gray-50">
                        {formatDate(message.created_at)}
                    </span>
                </div>

                <div className="text-gray-light leading-relaxed break-words">
                    {message.content}
                </div>
            </div>
        </div>
    );
}