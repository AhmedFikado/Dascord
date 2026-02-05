import { Member } from "@/types/models/member";
import { User, Status } from "../../types/models/user";
import { Dropdown } from "@/components/ui/dropdown";
import { useServerStore } from "@/app/lib/stores/use-server-store";
import { Role } from "@/types/models/role";
import { useState } from "react";
import { Dialog } from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { useCurrentUser } from "@/app/lib/hooks/use-current-user";


interface MemberitemProps {
    member: Member;
    isRole?: boolean;
    serverId?: string;
}

export default function MemberItem({ member, isRole = false, serverId }: MemberitemProps) {

    const [showTransferConfirm, setShowTransferConfirm] = useState(false);
    const [pendingRole, setPendingRole] = useState<Role | null>(null);
    const { userId } = useCurrentUser();
    const members = useServerStore((state) => state.members);
    const updateRoleMember = useServerStore((state) => state.updateRoleMember);

    const currentMember = members.find(m => m.user_id === userId);
    const currentUserRole = currentMember?.role;

    const getStatusIndicator = (status: Status) => {
        const baseClass = "absolute -bottom-0.5 -right-0.5 w-3 h-3 rounded-full border-2 border-backgroundSide";

        switch (status) {
            case Status.ONLINE:
                return <div className={baseClass}><div className="w-full h-full rounded-full bg-green"></div></div>;

            case Status.OFFLINE:
                return <div className={baseClass}><div className="w-full h-full rounded-full bg-gray-50"></div></div>;

            default:
                return <div className={baseClass}><div className="w-full h-full rounded-full bg-gray-50"></div></div>;
        }
    };

    const changeRole = (role: Role) => {
        if (!serverId) return;

        if (role === Role.OWNER) {
            setPendingRole(role);
            setShowTransferConfirm(true);
            return;
        }

        updateRoleMember(serverId, member.user.id, role);
    }

    const confirmTransferOwnership = () => {
        if (!serverId || !pendingRole) return;
        updateRoleMember(serverId, member.user.id, pendingRole);
        setShowTransferConfirm(false);
        setPendingRole(null);
    }

    const availableRoles = Object.values(Role).filter(role => {
        if (currentUserRole === Role.ADMIN && role === Role.OWNER) {
            return false;
        }
        return true;
    });

    const canModifyRole = () => {
        if (currentUserRole === Role.OWNER) {
            return true;
        }
        if (currentUserRole === Role.ADMIN) {
            return member.role !== Role.OWNER;
        }
        return false;
    };

    const showDropdown = isRole && canModifyRole();

    return (
        <>
            <div className="flex mx-2 rounded hover:bg-hoverSide cursor-pointer group transition-colors justify-between items-center">
                <div className="flex items-center gap-3 px-2 py-1.5">
                    <div className="relative flex-shrink-0">
                        <div className="w-8 h-8 rounded-full bg-blurple flex items-center justify-center text-white font-semibold text-sm">
                            {member.user.username.charAt(0).toUpperCase()}
                        </div>
                        {getStatusIndicator(member.user.status)}
                    </div>
                    <span className="text-sm font-medium text-gray-light group-hover:text-white transition-colors truncate">
                        {member.user.username}
                    </span>
                </div>
                <div>
                    {showDropdown ? (<Dropdown
                        options={availableRoles.map((role) => ({ label: role, value: role }))}
                        value={member.role}
                        onChange={(value) => {
                            changeRole(value as Role);
                        }}
                        className=" mr-2"
                        width="150px"
                    />) : null}

                </div>
            </div>

            <Dialog
                isOpen={showTransferConfirm}
                onClose={() => setShowTransferConfirm(false)}
                title="Transférer la propriété du serveur"
            >
                <div className="space-y-4">
                    <p className="text-white text-sm">
                        Êtes-vous sûr de vouloir transférer la propriété du serveur à <span className="font-semibold">{member.user.username}</span> ?
                    </p>
                    <p className="text-gray-50 text-sm">
                        Vous deviendrez ADMIN et <span className="font-semibold">{member.user.username}</span> deviendra le nouveau propriétaire (OWNER).
                    </p>
                    <p className="text-red text-sm font-semibold">
                        Cette action est irréversible !
                    </p>
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
                        <Button
                            variant="danger"
                            onClick={confirmTransferOwnership}
                        >
                            Confirmer le transfert
                        </Button>
                    </div>
                </div>
            </Dialog>
        </>
    );
}