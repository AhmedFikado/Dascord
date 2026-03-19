import { useCurrentUser } from '@/app/lib/hooks/use-current-user';
import { useServerStore } from '@/app/lib/stores/use-server-store';
import { usePrivateChannelStore } from '@/app/lib/stores/use-private-channel-store';
import { Role } from '@/types/models/role';
import { useEffect, useRef, useState } from 'react';
import { useRouter } from 'next/navigation';
import { useTranslation } from 'react-i18next';
import PermanentBanDialog from './permanent-ban-dialog';
import TempBanDialog from './temp-ban-dialog';

interface MemberContextMenuProps {
  targetMemberId: string;
  serverId: string;
  onClose: () => void;
  x: number;
  y: number;
}

export default function MemberContextMenu({
  targetMemberId,
  serverId,
  onClose,
  x,
  y,
}: MemberContextMenuProps) {
  const { userId } = useCurrentUser();
  const router = useRouter();
  const { t } = useTranslation();
  const members = useServerStore(state => state.members);
  const currentMember = members.find(m => m.user_id === userId);
  const currentUserRole = currentMember?.role;
  const { createOrGetPrivateChannel } = usePrivateChannelStore();

  const menuRef = useRef<HTMLDivElement | null>(null);
  const hasPermission = currentUserRole === Role.OWNER || currentUserRole === Role.ADMIN;

  const [showTempBan, setShowTempBan] = useState(false);
  const [showPermBan, setShowPermBan] = useState(false);
  const isDialogOpen = showTempBan || showPermBan;

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (isDialogOpen) {
        return;
      }

      if (menuRef.current && !menuRef.current.contains(event.target as Node)) {
        onClose();
      }
    };

    document.addEventListener('mousedown', handleClickOutside);
    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, [isDialogOpen, onClose]);

  const kickMember = useServerStore(state => state.kickMember);
  const banMember = useServerStore(state => state.banMember);

  const handleKick = async () => {
    await kickMember(serverId, targetMemberId);
    onClose();
  };

  const handleTempBan = () => {
    setShowTempBan(true);
  };

  const handlePermBan = () => {
    setShowPermBan(true);
  };

  const handleSendMessage = async () => {
    try {
      if (!userId || !targetMemberId) {
        throw new Error('User IDs are missing');
      }

      console.log('Opening/Creating DM with:', targetMemberId);
      const channel = await createOrGetPrivateChannel(userId, targetMemberId);
      console.log('Channel obtained:', channel.id);
      router.push(`/dms/${channel.id}`);
      onClose();
    } catch (error: any) {
      console.error('Erreur lors de l\'ouverture du message privé:', error);
      // Still try to navigate even if there was an error, as the channel might exist
      // This will be handled by the store's error state
    }
  };

  return (
    <>
      {!isDialogOpen && (
        <div
          ref={menuRef}
          className="fixed bg-gray-700 text-white rounded-md shadow-lg p-1 z-50 min-w-[200px]"
          style={{
            top: `${y}px`,
            left: `${x}px`,
          }}
        >
          <button
            onClick={handleSendMessage}
            className="w-full text-left px-3 py-2 hover:bg-gray-600 rounded text-blue-400"
          >
            {t('DM.send_private_message')}
          </button>
          {hasPermission && (
            <>
              <div className="border-t border-gray-600 my-1"></div>
              <button
                onClick={handleKick}
                className="w-full text-left px-3 py-2 hover:bg-gray-600 rounded text-red-500"
              >
                Expulser
              </button>
              <button
                onClick={handleTempBan}
                className="w-full text-left px-3 py-2 hover:bg-gray-600 rounded text-red-500"
              >
                Bannir temporairement
              </button>
              <button
                onClick={handlePermBan}
                className="w-full text-left px-3 py-2 hover:bg-gray-600 rounded text-red-500"
              >
                Bannir définitivement
              </button>
            </>
          )}
        </div>
      )}

      <TempBanDialog
        isOpen={showTempBan}
        onClose={() => {
          setShowTempBan(false);
          onClose();
        }}
        onConfirm={async duration => {
          const durationMap: Record<string, number> = {
            '1h': 1,
            '10h': 10,
            '24h': 24,
            '48h': 48,
            '1w': 168,
          };
          const hours = durationMap[duration] ?? 24;
          const expiresAt = new Date(Date.now() + hours * 3600 * 1000).toISOString();
          await banMember(serverId, targetMemberId, 'Temporary', expiresAt);
          setShowTempBan(false);
          onClose();
        }}
      />
      <PermanentBanDialog
        isOpen={showPermBan}
        onClose={() => {
          setShowPermBan(false);
          onClose();
        }}
        onConfirm={async () => {
          await banMember(serverId, targetMemberId, 'Permanent');
          setShowPermBan(false);
          onClose();
        }}
      />
    </>
  );
}
