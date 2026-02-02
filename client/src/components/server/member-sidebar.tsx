import MemberList from "./member-list";

export default function MemberSidebar() {

    return (
        <aside className="w-60 h-full flex flex-col bg-background border-l border-gray-200 overflow-y-auto scrollbar-hide flex-shrink-0">
            <div className="flex-1 overflow-y-auto">
                <MemberList />
            </div>
        </aside>
    );

}