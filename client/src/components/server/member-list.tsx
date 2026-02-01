import { User, Status } from "../../types/models/user";
import MemberItem from "./member-item";

interface MemberListProps {
    searchQuery?: string;
    isRole?: boolean;
}

export default function MemberList({ searchQuery = '', isRole = false }: MemberListProps) {

    const Users: User[] = [
        {
            id: 1,
            username: 'Ethan',
            email: 'ethan@gmail.com',
            created_at: new Date(),
            status: Status.ONLINE
        },
        {
            id: 2,
            username: 'Ahmed',
            email: 'ahmed@gmail.com',
            created_at: new Date(),
            status: Status.OFFLINE
        },
        {
            id: 3,
            username: 'Alexis',
            email: 'alexis@gmail.com',
            created_at: new Date(),
            status: Status.ONLINE
        },
    ];

    const filterUsers = Users.filter(user =>
        user.username.toLowerCase().includes(searchQuery.toLowerCase())
    );


    return (
        <div className="flex flex-col py-2">
            <h3 className="px-4 py-2 text-xs font-semibold text-gray-50 uppercase">
                Membres — {Users.length}
            </h3>

            {isRole ? (
                filterUsers.map((user) => (
                    <MemberItem key={user.id} user={user} isRole={true} />
                ))
            ) : (
                filterUsers.map((user) => (
                    <MemberItem key={user.id} user={user} />
                ))
            )}
        </div>
    );

}