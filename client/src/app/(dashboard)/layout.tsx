'use client';

import Sidebar from "@/components/layout/sidebar";
import ServerSidebar from "@/components/server/server-sidebar";
import MemberSidebar from "@/components/server/member-sidebar";
import { useParams } from 'next/navigation';
import UserPanel from "@/components/user/user-panel";
import { useServerStore } from '@/app/lib/stores/use-server-store';
import { useState, createContext, useContext, useEffect } from 'react';

interface MobileNavContextType {
    openNav: () => void;
    openMembers: () => void;
}

const MobileNavContext = createContext<MobileNavContextType>({
    openNav: () => {},
    openMembers: () => {},
});

export const useMobileNav = () => useContext(MobileNavContext);

export default function DashboardLayout({
    children,
}: {
    children: React.ReactNode;
}) {
    const params = useParams();
    const serverId = params?.serverId ? (params.serverId as string) : null;
    const [isMembersOpen, setIsMembersOpen] = useState(false);
    const [isNavOpen, setIsNavOpen] = useState(false);
    const { setCurrentServer, servers } = useServerStore();

    useEffect(() => {
        if (serverId && servers.length > 0) {
            const server = servers.find(s => s.id === serverId);
            if (server) {
                setCurrentServer(server);
            }
        } else {
            setCurrentServer(null);
        }
    }, [serverId, servers, setCurrentServer]);

    return (
        <MobileNavContext.Provider value={{
            openNav: () => setIsNavOpen(true),
            openMembers: () => setIsMembersOpen(true),
        }}>
            <div className="flex h-screen w-screen overflow-hidden">
                <div className="hidden md:flex h-full flex-shrink-0">
                    <Sidebar />
                </div>

                {serverId && (
                    <div className="hidden md:flex h-full flex-shrink-0">
                        <ServerSidebar serverId={serverId} />
                    </div>
                )}

                <div className="hidden md:flex flex-shrink-0">
                    <UserPanel />
                </div>

                <main className="flex-1 min-w-0 h-full overflow-hidden">
                    {children}
                </main>

                {serverId && (
                    <div className="hidden lg:flex h-full flex-shrink-0">
                        <MemberSidebar serverId={serverId} />
                    </div>
                )}

                {isNavOpen && (
                    <div
                        className="md:hidden fixed inset-0 z-40 bg-black/50"
                        onClick={() => setIsNavOpen(false)}
                    >
                        <div
                            className="absolute left-0 top-0 h-full flex"
                            onClick={e => e.stopPropagation()}
                        >
                            <Sidebar />
                            {serverId && <ServerSidebar serverId={serverId} />}
                            <UserPanel />
                        </div>
                    </div>
                )}

                {serverId && isMembersOpen && (
                    <div
                        className="lg:hidden fixed inset-0 z-40 bg-black/50"
                        onClick={() => setIsMembersOpen(false)}
                    >
                        <div
                            className="absolute right-0 top-0 h-full"
                            onClick={e => e.stopPropagation()}
                        >
                            <MemberSidebar serverId={serverId} />
                        </div>
                    </div>
                )}
            </div>
        </MobileNavContext.Provider>
    );
}
