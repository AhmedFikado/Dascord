'use client';

import Sidebar from "@/components/layout/sidebar";
import ServerSidebar from "@/components/server/server-sidebar";
import MemberSidebar from "@/components/server/member-sidebar";
import { useParams } from 'next/navigation';
import UserPanel from "@/components/user/user-panel";

export default function DashboardLayout({
    children,
}: {
    children: React.ReactNode;
}) {
    const params = useParams();
    const serverId = params?.serverId ? (params.serverId as string) : '1';

    return (
        <div className="flex h-screen w-screen overflow-hidden">
            <Sidebar />
            <ServerSidebar serverId={serverId} />

            <UserPanel />

            {children}

            <MemberSidebar />
        </div>
    );
}