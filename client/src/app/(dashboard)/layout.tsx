'use client';

import Sidebar from "@/components/layout/sidebar";
import ServerSidebar from "@/components/server/server-sidebar";
import MemberSidebar from "@/components/server/member-sidebar";
import UserCard from '@/components/shared/user-card';
import { Button } from "@/components/ui/button";
import { Settings } from 'lucide-react';
import { useParams } from 'next/navigation';

export default function DashboardLayout({
    children,
}: {
    children: React.ReactNode;
}) {
    const params = useParams();
    const serverId = params?.serverId ? parseInt(params.serverId as string) : 1;

    return (
        <div className="flex h-screen w-screen overflow-hidden">
            <Sidebar />
            <ServerSidebar serverId={serverId} />

            <div className="fixed bottom-2 rounded-2xl pl-3 py-1 left-3 gap-3 bg-gray-400 w-72 flex items-center justify-between z-50">
                <div className="flex items-center gap-3">
                    <UserCard user={{ username: 'Alice' }} />
                    <div className="flex-1 min-w-0">
                        <span className="font-semibold text-white hover:underline cursor-pointer">
                            Alice
                        </span>
                    </div>
                </div>
                <Button variant="noBackground" width="50px" height="50px">
                    <Settings color="#adadad" size={24} />
                </Button>
            </div>

            {children}

            <MemberSidebar />
        </div>
    );
}