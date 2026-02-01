'use client';

import { useState } from 'react';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { Trash2, Save, ChevronDown, ChevronUp, Search } from 'lucide-react';
import { Server } from '@/types/models/Server';
import MemberList from './member-list';

interface ServerSettingsProps {
    server: Server;
    onClose: () => void;
    onUpdate?: (updatedServer: Server) => void;
    onDelete?: () => void;
}

export default function ServerSettings({ server, onClose, onUpdate, onDelete }: ServerSettingsProps) {
    const [serverName, setServerName] = useState(server.name);
    const [isLoading, setIsLoading] = useState(false);
    const [showDeleteConfirm, setShowDeleteConfirm] = useState(false);
    const [deleteConfirmText, setDeleteConfirmText] = useState('');
    const [isMembersOpen, setIsMembersOpen] = useState(false);
    const [searchMember, setSearchMember] = useState('');

    const handleSave = async () => {
        if (!serverName.trim()) return;

        setIsLoading(true);
        await new Promise(resolve => setTimeout(resolve, 300));

        const updatedServer = { ...server, name: serverName };
        onUpdate?.(updatedServer);
        setIsLoading(false);
        onClose();
    };

    const handleDelete = async () => {
        if (deleteConfirmText !== server.name) return;

        setIsLoading(true);

        onDelete?.();
        setIsLoading(false);
        onClose();
    };

    const hasChanges = serverName !== server.name;

    return (
        <div className="flex flex-col h-full">
            <div className="flex-1 overflow-y-auto space-y-8">
                <section>
                    <h3 className="text-white text-sm font-semibold uppercase mb-4">
                        Aperçu du serveur
                    </h3>
                    <div className="space-y-4">
                        <Input
                            label="Nom du serveur"
                            value={serverName}
                            onChange={(e) => setServerName(e.target.value)}
                            placeholder="Entrez le nom du serveur"
                            maxLength={100}
                        />
                    </div>
                </section>

                <div className="border-t border-gray-200"></div>

                <section>
                    <button
                        onClick={() => setIsMembersOpen(!isMembersOpen)}
                        className="w-full flex items-center justify-between p-3 bg-gray-400 hover:bg-hoverSide rounded-lg transition-colors"
                    >
                        <h3 className="text-white text-sm font-semibold uppercase">
                            Membres du serveur
                        </h3>
                        {isMembersOpen ? (
                            <ChevronUp className="text-white" size={20} />
                        ) : (
                            <ChevronDown className="text-white" size={20} />
                        )}
                    </button>

                    {isMembersOpen && (
                        <div className="mt-4 space-y-4">
                            <div className="relative ">
                                <Search className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-light" size={18} />
                                <Input
                                    type="text"
                                    value={searchMember}
                                    onChange={(e) => setSearchMember(e.target.value)}
                                    placeholder="Rechercher un membre..."
                                    className="w-9/10 text-white placeholder-gray-50 pl-10 pr-4 py-2 rounded-lg focus:outline-none focus:ring-2 focus:ring-blurple"
                                />
                            </div>

                            <div className="bg-gray-400 rounded-lg p-4 max-h-96 overflow-y-auto">
                                <MemberList searchQuery={searchMember} isRole={true}/>
                            </div>
                        </div>
                    )}
                </section>

                <div className="border-t border-gray-200"></div>

                <section>
                    <div className="flex justify-center mb-4">
                        <Button
                            variant="danger"
                            onClick={() => setShowDeleteConfirm(!showDeleteConfirm)}
                            className="flex items-center gap-2"
                        >
                            <Trash2 size={16} />
                            Supprimer le serveur
                        </Button>
                    </div>

                    {showDeleteConfirm && (
                        <div className="mt-4 pt-4 border-t border-gray-200 space-y-3">
                            <p className="text-white text-sm font-semibold">
                                Êtes-vous sûr de vouloir supprimer ce serveur ?
                            </p>
                            <p className="text-gray-50 text-sm">
                                Tapez <span className="text-white font-semibold">{server.name}</span> pour confirmer
                            </p>
                            <Input
                                value={deleteConfirmText}
                                onChange={(e) => setDeleteConfirmText(e.target.value)}
                                placeholder={server.name}
                            />
                            <div className="flex gap-2 justify-end">
                                <Button
                                    variant="secondary"
                                    onClick={() => {
                                        setShowDeleteConfirm(false);
                                        setDeleteConfirmText('');
                                    }}
                                >
                                    Annuler
                                </Button>
                                <Button
                                    variant="danger"
                                    onClick={handleDelete}
                                    disabled={deleteConfirmText !== server.name}
                                    isLoading={isLoading}
                                >
                                    Supprimer définitivement
                                </Button>
                            </div>
                        </div>
                    )}
                </section>
            </div>

            {hasChanges && (
                <div className="bg-gray-400 p-4 flex items-center justify-between border-t border-gray-200 mt-4">
                    <p className="text-white text-sm">
                        Attention — vous avez des modifications non enregistrées !
                    </p>
                    <div className="flex gap-2">
                        <Button
                            variant="secondary"
                            onClick={() => setServerName(server.name)}
                        >
                            Réinitialiser
                        </Button>
                        <Button
                            variant="primary"
                            onClick={handleSave}
                            isLoading={isLoading}
                            className="flex items-center gap-2"
                        >
                            <Save size={16} />
                            Enregistrer
                        </Button>
                    </div>
                </div>
            )}
        </div>
    );
}