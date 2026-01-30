import ServerList from "../server/server-list";
import CreateServerDialog from "../server/create-server-dialog";
import { useServers } from '@/app/lib/hooks/use-servers';

export default function Sidebar() {

    const { servers } = useServers();

    return (
        <aside className="w-18 h-full bg-backgroundSide flex flex-col items-center py-3 gap-2 overflow-y-auto scrollbar-hide flex-shrink-0 border-r border-gray-200">
            <div className="w-12 h-12 bg-gray-400 rounded-2xl mb-1">
                <img
                    src="/icons/logo.png"
                    alt="Logo"
                    className="w-full h-full object-contain rounded-2xl"
                />
            </div>
            <ServerList servers={servers} />
            <CreateServerDialog />
        </aside>
    );
}