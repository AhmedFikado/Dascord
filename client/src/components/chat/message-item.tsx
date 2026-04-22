import { useCurrentUser } from '@/app/lib/hooks/use-current-user';
import { useMessageStore } from '@/app/lib/stores/use-messages-store';
import { usePrivateChannelStore } from '@/app/lib/stores/use-private-channel-store';
import { useServerStore } from '@/app/lib/stores/use-server-store';
import { Button } from '@/components/ui/button';
import { Role } from '@/types/models/role';
import EmojiPicker, { EmojiClickData } from 'emoji-picker-react';
import { Check, Edit, SmilePlus, Trash2, X } from 'lucide-react';
import { useEffect, useRef, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { Message } from '../../types/models/message';
import UserCard from '../shared/user-card';
import MemberContextMenu from '../server/member-context-menu';

interface MessageItemProps {
  message: Message;
  onDelete: () => void;
  onUpdate: (content: string) => void;
  onAddReaction?: (messageId: string, reaction: string) => void;
  onRemoveReaction?: (messageId: string, reaction: string) => void;
}

export default function MessageItem({ message, onDelete, onUpdate, onAddReaction, onRemoveReaction }: MessageItemProps) {
  const { user } = useCurrentUser();
  const currentUserId = user?.id;
  const members = useServerStore(state => state.members);
  const currentServer = useServerStore(state => state.currentServer);
  const privateChannels = usePrivateChannelStore(state => state.privateChannels);
  const currentPrivateChannel = usePrivateChannelStore(state => state.currentPrivateChannel);
  const [contextMenu, setContextMenu] = useState<{ x: number; y: number } | null>(null);
  const [isEditing, setIsEditing] = useState(false);
  const [editContent, setEditContent] = useState(message.content);
  const [isActionsVisible, setIsActionsVisible] = useState(false);
  const [showEmojiPicker, setShowEmojiPicker] = useState(false);
  const emojiPickerRef = useRef<HTMLDivElement>(null);
  const { addReaction, removeReaction } = useMessageStore();
  const { t } = useTranslation();

  useEffect(() => {
    const handleClickOutside = (e: MouseEvent) => {
      if (emojiPickerRef.current && !emojiPickerRef.current.contains(e.target as Node)) {
        setShowEmojiPicker(false);
      }
    };
    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);

  const handleEmojiClick = (emojiData: EmojiClickData) => {
    if (!currentUserId || !message.id) return;
    if (onAddReaction) {
      onAddReaction(message.id, emojiData.emoji);
    } else {
      addReaction(message.id, emojiData.emoji, currentUserId);
    }
    setShowEmojiPicker(false);
  };

  const handleToggleReaction = (emoji: string) => {
    if (!currentUserId || !message.id) return;
    const users = message.reactions?.[emoji] || [];
    if (users.includes(currentUserId)) {
      if (onRemoveReaction) onRemoveReaction(message.id, emoji);
      else removeReaction(message.id, emoji, currentUserId);
    } else {
      if (onAddReaction) onAddReaction(message.id, emoji);
      else addReaction(message.id, emoji, currentUserId);
    }
  };

  const currentMember = members.find(m => m.user_id === currentUserId);
  const isAdminOrOwner = currentMember
    ? currentMember.role === Role.OWNER || currentMember.role === Role.ADMIN
    : false;

  const formatDate = (dateStr: string) => {
    if (!dateStr) return '';
    const date = new Date(dateStr);
    if (isNaN(date.getTime())) return '';
    return new Intl.DateTimeFormat('fr-FR', {
      day: '2-digit',
      month: '2-digit',
      year: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    }).format(date);
  };
  const handleUserContextMenu = (e: React.MouseEvent) => {
    if (!currentServer || message.user_id === currentUserId || isSystemMessage) return;
    const currentMember = members.find(m => m.user_id === currentUserId);
    const targetMember = members.find(m => m.user_id === message.user_id);
    if (currentMember?.role === Role.ADMIN && targetMember?.role === Role.OWNER) return;
    e.preventDefault();
    e.stopPropagation();
    setContextMenu({ x: e.clientX, y: e.clientY });
  };

  const isOwnerMessage = message.user_id === currentUserId;
  const canDeleteMessage = isOwnerMessage || isAdminOrOwner;
  const isSystemMessage = message.username === 'Système';
  const targetMember = members.find(m => m.user_id === message.user_id);
  const dmChannel =
    privateChannels.find(channel => channel.id === message.channel_id)
    || (currentPrivateChannel?.id === message.channel_id ? currentPrivateChannel : undefined);
  const messageAvatarId = message.user_id === currentUserId
    ? user?.avatar_id
    : targetMember?.user.avatar_id || dmChannel?.recipient_user?.avatar_id;

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
    <>
    <div
      className={`relative flex gap-4 px-4 py-2 lg:hover:bg-gray-400/50 group ${isActionsVisible ? 'bg-gray-400/50 lg:bg-transparent' : ''}`}
      onClick={() => setIsActionsVisible(v => !v)}
    >
      {canDeleteMessage && !isEditing && (
        <div
          className={`absolute -top-4 right-4 ${isActionsVisible ? 'flex lg:hidden' : 'hidden'} lg:group-hover:flex bg-gray-300 border border-gray-200 rounded-lg shadow-lg`}
        >
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

      <div onContextMenu={handleUserContextMenu} className="cursor-pointer flex-shrink-0">
        <UserCard username={message.username} avatarId={messageAvatarId} />
      </div>

      <div className="flex-1 min-w-0">
        <div className="flex items-baseline gap-2 mb-0.5">
          <span
            className="font-semibold text-white cursor-pointer"
            onContextMenu={handleUserContextMenu}
          >{message.username}</span>
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
              onChange={e => setEditContent(e.target.value)}
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
            className={`leading-relaxed break-words ${
              isSystemMessage ? 'text-gray-light italic' : 'text-white'
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

        {!isSystemMessage && (
          <div className="flex flex-wrap items-center gap-1 mt-1">
            {Object.entries(message.reactions || {}).map(([emoji, users]) => (
              <button
                key={emoji}
                onClick={e => {
                  e.stopPropagation();
                  handleToggleReaction(emoji);
                }}
                className={`flex items-center gap-1 px-2 py-0.5 rounded-full text-sm border transition-colors ${currentUserId && users.includes(currentUserId)
                    ? 'bg-purple/30 border-purple text-white'
                    : 'bg-gray-300 border-gray-200 text-gray-light hover:border-purple'
                }`}
              >
                <span>{emoji}</span>
                <span>{users.length}</span>
              </button>
            ))}

            <div className="relative" ref={emojiPickerRef}>
              <button
                onClick={e => {
                  e.stopPropagation();
                  setShowEmojiPicker(v => !v);
                }}
                className="flex items-center p-1 rounded-full text-gray-light lg:hover:text-white lg:hover:bg-gray-300 transition-colors lg:opacity-0 lg:group-hover:opacity-100"
              >
                <SmilePlus size={16} />
              </button>
              {showEmojiPicker && (
                <div className="absolute bottom-8 left-0 z-50">
                  <EmojiPicker onEmojiClick={handleEmojiClick} />
                </div>
              )}
            </div>
          </div>
        )}
      </div>
    </div>

      {contextMenu && currentServer && (
        <MemberContextMenu
          targetMemberId={message.user_id}
          serverId={currentServer.id}
          x={contextMenu.x}
          y={contextMenu.y}
          onClose={() => setContextMenu(null)}
        />
      )}
    </>
  );
}
