import { Message } from '../../types/models/message';
import UserCard from '../shared/user-card';
import { useCurrentUser } from '@/app/lib/hooks/use-current-user';
import { Trash2, Edit } from 'lucide-react';
import { Button } from "@/components/ui/button";
import { useServerStore } from '@/app/lib/stores/use-server-store';
import { Role } from '@/types/models/role';

interface MessageItemProps {
    serverId: string;
    message: Message;
    onDelete: () => void;
}

export default function MessageItem({ serverId, message, onDelete }: MessageItemProps) {

    const { user } = useCurrentUser();
    const currentUserId = user?.id;
    const members = useServerStore((state) => state.members);
    
    const currentMember = members.find(m => m.user_id === currentUserId);
    const isAdminOrOwner = currentMember ? 
        (currentMember.role === Role.OWNER || currentMember.role === Role.ADMIN) : false;

    const formatDate = (dateStr: string) => {
        return new Intl.DateTimeFormat('fr-FR', {
            day: '2-digit',
            month: '2-digit',
            year: 'numeric',
            hour: '2-digit',
            minute: '2-digit'
        }).format(new Date(dateStr));
    };
    const isOwnerMessage = message.user_id === currentUserId;
    const canDeleteMessage = isOwnerMessage || isAdminOrOwner;

    return (

        <div className="relative flex gap-4 px-4 py-2 hover:bg-gray-400/50 group">
            {canDeleteMessage && (
                <div className="absolute -top-4 right-4 hidden group-hover:flex bg-gray-300 border border-gray-200 rounded-lg shadow-lg">
                    {isOwnerMessage && (
                        <Button className="p-2 hover:bg-hoverSide rounded-l-lg transition-colors"
                            variant="noBackground"
                            onClick={() => {/* TODO: implement edit */ }}>
                            <Edit size={16} className="text-gray-light hover:text-white" />
                        </Button>
                    )}
                    <Button className={`p-2 hover:bg-red/20 ${isOwnerMessage ? 'rounded-r-lg' : 'rounded-lg'} transition-colors`}
                        variant="noBackground"
                        onClick={onDelete}>
                        <Trash2 size={16} className="text-red" />
                    </Button>
                </div>
            )}

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
                <div className="text-white leading-relaxed break-words">
                    {message.content}
                </div>
            </div>
        </div>
    );
}