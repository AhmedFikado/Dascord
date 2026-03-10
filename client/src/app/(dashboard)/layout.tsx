'use client';

import Sidebar from "@/components/layout/sidebar";
import ServerSidebar from "@/components/server/server-sidebar";
import MemberSidebar from "@/components/server/member-sidebar";
import { useParams } from 'next/navigation';
import UserPanel from "@/components/user/user-panel";
import { useServerStore } from '@/app/lib/stores/use-server-store';
import { useEffect } from 'react';

export default function DashboardLayout({
    children,
}: {
    children: React.ReactNode;
}) {
    const params = useParams();
    const serverId = params?.serverId ? (params.serverId as string) : null;
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
        <div className="flex h-screen w-screen overflow-hidden">
            <Sidebar />
            {serverId && <ServerSidebar serverId={serverId} />}

            <UserPanel />

            {children}

            {serverId && <MemberSidebar serverId={serverId} />}
        </div>
    );
}