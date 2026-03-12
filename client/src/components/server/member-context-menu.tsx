import { useCurrentUser } from '@/app/lib/hooks/use-current-user';
import { useServerStore } from '@/app/lib/stores/use-server-store';
import { Role } from '@/types/models/role';
import { useEffect, useRef } from 'react';

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
  const members = useServerStore(state => state.members);
  const currentMember = members.find(m => m.user_id === userId);
  const currentUserRole = currentMember?.role;

  const menuRef = useRef<HTMLDivElement | null>(null);
  const hasPermission = currentUserRole === Role.OWNER || currentUserRole === Role.ADMIN;

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (menuRef.current && !menuRef.current.contains(event.target as Node)) {
        onClose();
      }
    };

    document.addEventListener('mousedown', handleClickOutside);
    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, [onClose]);

  if (!hasPermission) {
    return null;
  }

  const handleKick = () => {
    console.log('Kick member:', targetMemberId);
    onClose();
  };

  const handleTempBan = () => {
    console.log('Temp ban member:', targetMemberId);
    onClose();
  };

  const handlePermBan = () => {
    console.log('Perm ban member:', targetMemberId);
    onClose();
  };

  return (
    <div
      ref={menuRef}
      className="fixed bg-gray-700 text-white rounded-md shadow-lg p-1 z-50 min-w-[200px]"
      style={{
        top: `${y}px`,
        left: `${x}px`,
      }}
    >
      <button
        onClick={handleKick}
        className="w-full text-left px-3 py-2 hover:bg-gray-600 rounded text-red-500"
      >
        Ejecter
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
    </div>
  );
}
