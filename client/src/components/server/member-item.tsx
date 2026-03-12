import { useCurrentUser } from '@/app/lib/hooks/use-current-user';
import { useServerStore } from '@/app/lib/stores/use-server-store';
import UserCard from '@/components/shared/user-card';
import { Button } from '@/components/ui/button';
import { Dialog } from '@/components/ui/dialog';
import { Dropdown } from '@/components/ui/dropdown';
import { Member } from '@/types/models/member';
import { Role } from '@/types/models/role';
import { useState } from 'react';
import MemberContextMenu from './member-context-menu';

interface MemberitemProps {
  member: Member;
  isRole?: boolean;
  serverId?: string;
}

export default function MemberItem({ member, isRole = false, serverId }: MemberitemProps) {
  const [showTransferConfirm, setShowTransferConfirm] = useState(false);
  const [pendingRole, setPendingRole] = useState<Role | null>(null);
  const { userId } = useCurrentUser();
  const members = useServerStore(state => state.members);
  const updateRoleMember = useServerStore(state => state.updateRoleMember);

  const currentMember = members.find(m => m.user_id === userId);
  const currentUserRole = currentMember?.role;

  const changeRole = (role: Role) => {
    if (!serverId) return;

    if (role === Role.OWNER) {
      setPendingRole(role);
      setShowTransferConfirm(true);
      return;
    }

    updateRoleMember(serverId, member.user.id, role);
  };

  const confirmTransferOwnership = async () => {
    if (!serverId || !pendingRole) return;

    try {
      await updateRoleMember(serverId, member.user.id, pendingRole);
      setShowTransferConfirm(false);
      setPendingRole(null);
    } catch (error) {
      console.error('Erreur lors du transfert de propriété:', error);
    }
  };

  const availableRoles = Object.values(Role).filter(role => {
    if (currentUserRole === Role.ADMIN && role === Role.OWNER) {
      return false;
    }
    return true;
  });

  const canModifyRole = () => {
    if (currentUserRole === Role.OWNER) {
      if (member.user_id === userId) {
        return false;
      }
      return true;
    }
    if (currentUserRole === Role.ADMIN) {
      return member.role !== Role.OWNER;
    }
    return false;
  };

  const showDropdown = isRole && canModifyRole();

  const [contextMenu, setContextMenu] = useState<{ x: number; y: number } | null>(null);

  const canShowContextMenu = () => {
    if (member.user_id === userId) return false;

    if (currentUserRole === Role.ADMIN && member.role === Role.OWNER) return false;

    return true;
  };

  const handleContextMenu = (e: React.MouseEvent) => {
    e.preventDefault();
    if (!canShowContextMenu()) return;
    setContextMenu({ x: e.clientX, y: e.clientY });
  };

  return (
    <>
      <div
        className="flex mx-2 rounded hover:bg-hoverSide cursor-pointer group transition-colors justify-between items-center"
        onContextMenu={handleContextMenu}
      >
        <div className="flex items-center gap-3 px-2 py-1.5">
          <UserCard username={member.user.username} size={32} status={member.user.status} />
          <span className="text-sm font-medium text-gray-light group-hover:text-white transition-colors truncate">
            {member.user.username}
          </span>
        </div>
        <div>
          {showDropdown ? (
            <Dropdown
              options={availableRoles.map(role => ({ label: role, value: role }))}
              value={member.role}
              onChange={value => {
                changeRole(value as Role);
              }}
              className=" mr-2"
              width="150px"
            />
          ) : null}
        </div>
      </div>

      <Dialog
        isOpen={showTransferConfirm}
        onClose={() => setShowTransferConfirm(false)}
        title="Transférer la propriété du serveur"
      >
        <div className="space-y-4">
          <p className="text-white text-sm">
            Êtes-vous sûr de vouloir transférer la propriété du serveur à{' '}
            <span className="font-semibold">{member.user.username}</span> ?
          </p>
          <p className="text-gray-50 text-sm">
            Vous deviendrez ADMIN et <span className="font-semibold">{member.user.username}</span>{' '}
            deviendra le nouveau propriétaire (OWNER).
          </p>
          <p className="text-red text-sm font-semibold">Cette action est irréversible !</p>
          <div className="flex gap-4 justify-end mt-6">
            <Button
              variant="secondary"
              onClick={() => {
                setShowTransferConfirm(false);
                setPendingRole(null);
              }}
            >
              Annuler
            </Button>
            <Button variant="danger" onClick={confirmTransferOwnership}>
              Confirmer le transfert
            </Button>
          </div>
        </div>
      </Dialog>
      {contextMenu && serverId && (
        <MemberContextMenu
          targetMemberId={member.user_id}
          serverId={serverId}
          x={contextMenu.x}
          y={contextMenu.y}
          onClose={() => setContextMenu(null)}
        />
      )}
    </>
  );
}
