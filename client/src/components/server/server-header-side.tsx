'use client';

import { Server } from "@/types/models/Server";
import { Button } from "../ui/button";
import { ChevronDown, ChevronUp, Settings, LogOut } from 'lucide-react';
import dynamic from 'next/dynamic';
import ServerSettings from "./server-settings";
import { Dialog } from '../ui/dialog';
import { use, useState } from "react";
import { useCurrentUser } from '@/app/lib/hooks/use-current-user';
import { useServerStore } from "@/app/lib/stores/use-server-store";
import { useSnackbar } from "@/components/shared/error-message";
import { useRouter } from 'next/navigation';
import { Role } from '@/types/models/role';

const InvitationDialog = dynamic(
    () => import('./invitation-dialog'),
    { ssr: false }
);

export default function ServerHeaderSide({ server }: { server: Server }) {

    const [isOpenMenu, setIsOpenMenu] = useState(false);
    const [isSettingsOpen, setIsSettingsOpen] = useState(false);
    const { userId } = useCurrentUser();
    const deleteServer = useServerStore((state) => state.deleteServer);
    const leaveServer = useServerStore((state) => state.leaveServer);
    const members = useServerStore((state) => state.members);
    const { showSnackbar } = useSnackbar();
    const router = useRouter();

    const currentMember = members.find(m => m.user_id === userId);
    const isOwner = currentMember?.role === Role.OWNER;
    const canManageServer = currentMember ? 
        (currentMember.role === Role.OWNER || currentMember.role === Role.ADMIN) : false;

    const handleServerSettings = () => {
        setIsOpenMenu(false);
        setIsSettingsOpen(true);
    }

    const handleLeaveServer = async () => {
        setIsOpenMenu(false);
        try {
            await leaveServer(server.id);
            showSnackbar({ message: "Vous avez quitté le serveur avec succès.", severity: "success" });
            router.push('/servers');
        } catch (error) {
            showSnackbar({ message: "Erreur lors de la quitter le serveur.", severity: "error" });
        }
    }

    const handleUpdateServer = (updatedServer: Server) => {
        console.log('Serveur mis à jour:', updatedServer);
    }

    const handleDeleteServer = async () => {
        try {
            await deleteServer(server.id);
            showSnackbar({ message: "Vous avez supprimé votre serveur avec succès.", severity: "success" });
            router.push('/servers');
        } catch (error) {
            showSnackbar({ message: "Erreur lors de la suppression du serveur.", severity: "error" });
        }
    }

    return (
        <>
            <div className="flex items-center justify-between">
                <Button
                    onClick={() => setIsOpenMenu(!isOpenMenu)}
                    variant="noBackground"
                    width={"170px"}
                    className="flex cursor-pointer justify-start px-[8px]">
                    <h1 className="text-white text-base font-bold truncate flex-1 text-left">{server.name}</h1>

                    {isOpenMenu ? (
                        <ChevronUp className="text-white flex-shrink-0" size={20} />
                    ) : (
                        <ChevronDown className="text-white flex-shrink-0" size={20} />
                    )}
                </Button>

                {canManageServer && <InvitationDialog server={server} />}
            </div>

            {isOpenMenu && (
                <>
                    <div
                        className="fixed inset-0 z-10"
                        onClick={() => setIsOpenMenu(false)}
                    />

                    <div className="absolute top-12 left-20 mt-2 w-56 bg-gray-300 rounded-lg shadow-lg overflow-hidden z-20">
                        {canManageServer && (
                            <Button
                                onClick={handleServerSettings}
                                variant="noBackground"
                                className="justify-start px-4 py-3 "
                                width={"225px"}
                            >
                                <Settings size={18} className="text-gray-light mr-3" />
                                <span className="text-white text-sm">Paramètres du serveur</span>
                            </Button>
                        )}

                        {isOwner && <div className="border-t border-gray-200" />}

                        {!isOwner && (
                            <Button
                                onClick={handleLeaveServer}
                                variant="noBackground"
                                className="justify-start px-4 py-3 "
                                width={"225px"}
                            >
                                <LogOut size={18} className="text-red mr-3" />
                                <span className="text-red text-sm font-semibold">Quitter le serveur</span>
                            </Button>
                        )}

                    </div>
                </>
            )}

            <Dialog
                isOpen={isSettingsOpen}
                onClose={() => setIsSettingsOpen(false)}
                title="Paramètres du serveur"
                size="xl"
            >
                <ServerSettings
                    server={server}
                    onClose={() => setIsSettingsOpen(false)}
                    onUpdate={handleUpdateServer}
                    onDelete={handleDeleteServer}
                />
            </Dialog>
        </>
    );
}