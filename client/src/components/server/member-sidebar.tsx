import MemberList from "./member-list";
import { useMembers } from "@/app/lib/hooks/use-members";

export default function MemberSidebar({ serverId }: { serverId: string }) {
    const { members } = useMembers(serverId);

    return (
        <aside className="w-60 h-full flex flex-col bg-background border-l border-gray-200 overflow-y-auto scrollbar-hide flex-shrink-0">
            <div className="flex-1 overflow-y-auto">
                <MemberList listMembers={members} />
            </div>
        </aside>
    );
}