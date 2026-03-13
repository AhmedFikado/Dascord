'use client';

import { Dialog } from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { useState } from "react";
import { Plus, ChevronRight } from "lucide-react";
import CreateServerDialog from "../server/create-server-dialog";
import JoinServerDialog from "../server/join-server-dialog";
import { useTranslation } from 'react-i18next';

export default function JoinCreateServer() {
    const { t } = useTranslation();
    const [isMainDialogOpen, setIsMainDialogOpen] = useState(false);
    const [isCreateDialogOpen, setIsCreateDialogOpen] = useState(false);
    const [isJoinDialogOpen, setIsJoinDialogOpen] = useState(false);

    const handleCreateOwn = () => {
        setIsMainDialogOpen(false);
        setIsCreateDialogOpen(true);
    };

    const handleJoin = () => {
        setIsMainDialogOpen(false);
        setIsJoinDialogOpen(true);
    };

    const handleBackToMain = () => {
        setIsJoinDialogOpen(false);
        setIsCreateDialogOpen(false);
        setIsMainDialogOpen(true);
    };

    return (
        <>
            <Button
                width="45px"
                height="45px"
                style={{ borderRadius: '1rem', backgroundColor: '#5865F2', color: 'white', padding: 0 }}
                onClick={() => setIsMainDialogOpen(true)}
            >
                <Plus />
            </Button>

            <Dialog
                isOpen={isMainDialogOpen}
                onClose={() => setIsMainDialogOpen(false)}
                title={t('Join_create_server_dialog.create_your_server')}
                size="md"
            >
                <div className="flex flex-col gap-4">
                    <p className="text-center text-gray-light text-sm mb-4">
                        {t('Join_create_server_dialog.server_description')}
                    </p>

                    <button
                        onClick={handleCreateOwn}
                        className="w-full bg-gray-400 hover:bg-hoverSide rounded-lg p-4 flex items-center justify-between group transition-colors"
                    >
                        <div className="flex items-center gap-3">
                            <div className="w-12 h-12 bg-white rounded-full flex items-center justify-center">
                                <Plus className="text-[#5865F2]" size={28} />
                            </div>
                            <span className="text-white font-semibold">{t('Join_create_server_dialog.create_mine')}</span>
                        </div>
                        <ChevronRight className="text-gray-light group-hover:text-white transition-colors" size={24} />
                    </button>

                    <div className="relative my-4">
                        <div className="absolute inset-0 flex items-center">
                            <div className="w-full border-t border-gray-200"></div>
                        </div>
                        <div className="relative flex justify-center">
                            <span className="bg-gray-300 px-4 text-white font-bold text-sm">
                                {t('Join_create_server_dialog.already_have_invitation')}
                            </span>
                        </div>
                    </div>

                    <Button
                        onClick={handleJoin}
                        variant="secondary"
                        className="w-full"
                    >
                        {t('Join_create_server_dialog.join_server')}
                    </Button>
                </div>
            </Dialog>

            <CreateServerDialog
                isOpen={isCreateDialogOpen}
                onClose={() => setIsCreateDialogOpen(false)}
                onBack={handleBackToMain}
            />

            <JoinServerDialog
                isOpen={isJoinDialogOpen}
                onClose={() => setIsJoinDialogOpen(false)}
                onBack={handleBackToMain}
            />
        </>
    );
}