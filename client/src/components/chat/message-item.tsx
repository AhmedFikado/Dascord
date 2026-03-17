import { useCurrentUser } from '@/app/lib/hooks/use-current-user';
import { useServerStore } from '@/app/lib/stores/use-server-store';
import { Button } from '@/components/ui/button';
import { Role } from '@/types/models/role';
import { Edit, Trash2, X, Check } from 'lucide-react';
import { useState } from 'react';
import { Message } from '../../types/models/message';
import UserCard from '../shared/user-card';
import { useTranslation } from 'react-i18next';

interface MessageItemProps {
  message: Message;
  onDelete: () => void;
  onUpdate: (content: string) => void;
}

export default function MessageItem({ message, onDelete, onUpdate }: MessageItemProps) {
  const { user } = useCurrentUser();
  const currentUserId = user?.id;
  const members = useServerStore(state => state.members);
  const [isEditing, setIsEditing] = useState(false);
  const [editContent, setEditContent] = useState(message.content);
  const [isActionsVisible, setIsActionsVisible] = useState(false);
  const { t } = useTranslation();

  const currentMember = members.find(m => m.user_id === currentUserId);
  const isAdminOrOwner = currentMember
    ? currentMember.role === Role.OWNER || currentMember.role === Role.ADMIN
    : false;

  const formatDate = (dateStr: string) => {
    return new Intl.DateTimeFormat('fr-FR', {
      day: '2-digit',
      month: '2-digit',
      year: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    }).format(new Date(dateStr));
  };
  const isOwnerMessage = message.user_id === currentUserId;
  const canDeleteMessage = isOwnerMessage || isAdminOrOwner;
  const isSystemMessage = message.username === 'Système';

  const handleSaveEdit = () => {
    if (editContent.trim() && editContent !== message.content) {
      onUpdate(editContent.trim());
    }
    setIsEditing(false);
  };

  const handleCancelEdit = () => {
    setEditContent(message.content);
    setIsEditing(false);
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSaveEdit();
    } else if (e.key === 'Escape') {
      handleCancelEdit();
    }
  };

  const isGifUrl = (text: string) => {
    return text.trim().includes('giphy.com/media');
  };

  return (
    <div className={`relative flex gap-4 px-4 py-2 lg:hover:bg-gray-400/50 group ${isActionsVisible ? 'bg-gray-400/50 lg:bg-transparent' : ''}`}
      onClick={() => setIsActionsVisible(v => !v)}>
      {canDeleteMessage && !isEditing && (
        <div className={`absolute -top-4 right-4 ${isActionsVisible ? 'flex lg:hidden' : 'hidden'} lg:group-hover:flex bg-gray-300 border border-gray-200 rounded-lg shadow-lg`}>
          {isOwnerMessage && (
            <Button
              className="p-2 hover:bg-hoverSide rounded-l-lg transition-colors"
              variant="noBackground"
              onClick={() => setIsEditing(true)}
            >
              <Edit size={16} className="text-gray-light hover:text-white" />
            </Button>
          )}
          <Button
            className={`p-2 hover:bg-red/20 ${isOwnerMessage ? 'rounded-r-lg' : 'rounded-lg'} transition-colors`}
            variant="noBackground"
            onClick={onDelete}
          >
            <Trash2 size={16} className="text-red" />
          </Button>
        </div>
      )}

      <UserCard username={message.username} />

      <div className="flex-1 min-w-0">
        <div className="flex items-baseline gap-2 mb-0.5">
          <span className="font-semibold text-white cursor-pointer">{message.username}</span>
          <span className="text-xs text-gray-50">{formatDate(message.created_at)}</span>
          {message.updated_at && message.updated_at !== message.created_at && (
            <span className="text-xs text-gray-50 italic">{t('Message_item.modified')}</span>
          )}
        </div>
        {isEditing ? (
          <div className="flex items-center gap-2">
            <input
              type="text"
              value={editContent}
              onChange={(e) => setEditContent(e.target.value)}
              onKeyDown={handleKeyDown}
              className="flex-1 px-3 py-2 bg-gray-300 text-white rounded-md border border-gray-200 focus:outline-none focus:border-purple"
              autoFocus
            />
            <Button
              className="p-2 hover:bg-green-500/20 rounded-lg transition-colors"
              variant="noBackground"
              onClick={handleSaveEdit}
            >
              <Check size={16} className="text-green-500" />
            </Button>
            <Button
              className="p-2 hover:bg-red/20 rounded-lg transition-colors"
              variant="noBackground"
              onClick={handleCancelEdit}
            >
              <X size={16} className="text-red" />
            </Button>
          </div>
        ) : (
          <div
            className={`leading-relaxed break-words ${isSystemMessage ? 'text-gray-light italic' : 'text-white'
              }`}
          >
            {isGifUrl(message.content) ? (
              <img
                src={message.content}
                alt="GIF"
                className="xs:max-[100px] sm:max-w-sm rounded-md mt-2 object-contain bg-gray-300"
              />
            ) : (
              message.content
            )}
          </div>
        )}
      </div>
    </div>
  );
}
