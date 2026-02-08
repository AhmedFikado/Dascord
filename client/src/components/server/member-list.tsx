import { Member } from "@/types/models/member";
import { User, Status } from "../../types/models/user";
import MemberItem from "./member-item";

interface MemberListProps {
    searchQuery?: string;
    isRole?: boolean;
    listMembers: Member[];
    serverId?: string;
}

export default function MemberList({ searchQuery = '', isRole = false, listMembers, serverId }: MemberListProps) {

    const filterUsers = listMembers.filter(member =>
        member?.user?.username?.toLowerCase().includes(searchQuery.toLowerCase())
    );


    return (
        <div className="flex flex-col py-2">
            <h3 className="px-4 py-2 text-xs font-semibold text-gray-50 uppercase">
                Membres — {listMembers.length}
            </h3>

            {isRole ? (
                filterUsers.map((member) => (
                    <MemberItem key={member.user.id} member={member} isRole={true} serverId={serverId} />
                ))
            ) : (
                filterUsers.map((member) => (
                    <MemberItem key={member.user.id} member={member} serverId={serverId} />
                ))
            )}
        </div>
    );

}