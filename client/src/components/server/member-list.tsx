import { Member } from "@/types/models/member";
import MemberItem from "./member-item";
import { useTranslation } from 'react-i18next';

interface MemberListProps {
    searchQuery?: string;
    isRole?: boolean;
    listMembers: Member[];
    serverId?: string;
}

export default function MemberList({ searchQuery = '', isRole = false, listMembers, serverId }: MemberListProps) {

    const { t } = useTranslation();
    const filterUsers = listMembers.filter(member =>
        member?.user?.username?.toLowerCase().includes(searchQuery.toLowerCase())
    );
    const usersOwner = listMembers.filter(member => member?.role?.includes('OWNER'));
    const usersAdmin = listMembers.filter(member => member?.role?.includes('ADMIN'));
    const usersMember = listMembers.filter(member => member?.role?.includes('MEMBER'));


    return (
        <div className="flex flex-col py-2">
            <h3 className="px-4 py-2 text-xs font-semibold text-gray-50 uppercase">
                {t('Member_list.members')} — {listMembers.length}
            </h3>

            {isRole ? (
                filterUsers.map((member) => (
                    <MemberItem key={member.user.id} member={member} isRole={true} serverId={serverId} />
                ))
            ) : (
                <>
                    <div className="ml-6">
                        <h4 className="py-2 text-[10px] font-semibold text-gray-50 uppercase">Owner</h4>
                        {usersOwner.map((member) => (
                            <MemberItem key={member.user.id} member={member} serverId={serverId} />
                        ))}
                            
                        <h4 className="py-2 mt-2 text-[10px] font-semibold text-gray-50 uppercase">Admin</h4>
                        {usersAdmin.map((member) => (
                            <MemberItem key={member.user.id} member={member} serverId={serverId} />
                        ))}
                            <h4 className="py-2 mt-2 text-[10px] font-semibold text-gray-50 uppercase">Member</h4>
                        {usersMember.map((member) => (
                            <MemberItem key={member.user.id} member={member} serverId={serverId} />
                        ))}
                    </div>
                </>
            )}
        </div>
    );

}