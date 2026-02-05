import { Member } from "@/types/models/member";
import { User, Status } from "../../types/models/user";
import { Dropdown } from "@/components/ui/dropdown";
import { useServerStore } from "@/app/lib/stores/use-server-store";
import { Role } from "@/types/models/role";


interface MemberitemProps {
    member: Member;
    isRole?: boolean;
    serverId?: string;
}

export default function MemberItem({ member, isRole = false, serverId }: MemberitemProps) {

    const updateRoleMember = useServerStore((state) => state.updateRoleMember);

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
        updateRoleMember(serverId, member.user.id, role);
    }

    return (
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
                {isRole === true ? (<Dropdown
                    options={Object.values(Role).map((role) => ({ label: role, value: role }))}
                    value={member.role}
                    onChange={(value) => {
                        changeRole(value as Role);
                    }}
                    className=" mr-2"
                    width="150px"
                />) : null}

            </div>
        </div>
    );
}