'use client';

import { Server } from "@/types/models/Server";
import { Button } from "../ui/button";
import { Share2 } from 'lucide-react';
import dynamic from 'next/dynamic';

const InvitationDialog = dynamic(
    () => import('./invitation-dialog'),
    { ssr: false }
);

export default function ServerHeaderSide({ server }: { server: Server }) {
    return (
        <header className="flex items-center justify-between">
            <h1 className="text-white text-lg font-bold">{server.name}</h1>
            <InvitationDialog />
        </header>
    );
}