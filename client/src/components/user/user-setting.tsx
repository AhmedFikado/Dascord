'use client';

import { useState } from 'react';
import { Button } from '../ui/button';
import { Input } from '../ui/input';
import { User, Status } from '@/types/models/user';
import { useSnackbar } from "@/components/shared/error-message";

interface UserSettingProps {
    user: User;
    onUpdate?: (updatedUser: Partial<User>) => Promise<void>;
}

export default function UserSetting({ user, onUpdate }: UserSettingProps) {
    const [username, setUsername] = useState(user.username);
    const [email, setEmail] = useState(user.email);
    const [currentPassword, setCurrentPassword] = useState('');
    const [newPassword, setNewPassword] = useState('');
    const [confirmPassword, setConfirmPassword] = useState('');
    const { showSnackbar } = useSnackbar();

    const handleSave = async () => {
        if (!username.trim() || !email.trim()) {
            showSnackbar({ message: "Veuillez remplir tous les champs", severity: "error" });
            return;
        }
        try {
            await onUpdate?.({ username, email });
            showSnackbar({ message: "Profil mis à jour", severity: "success" });
        } catch {
            showSnackbar({ message: "Erreur lors de la mise à jour", severity: "error" });
        }
    };

    const handlePasswordChange = async () => {
        if (newPassword !== confirmPassword) {
            showSnackbar({ message: "Les mots de passe ne correspondent pas", severity: "error" });
            return;
        }
        showSnackbar({ message: "Mot de passe modifié avec succès !", severity: "success" });
        setCurrentPassword('');
        setNewPassword('');
        setConfirmPassword('');
    };

    const handleLogout = () => {
        //à faire 
    }

    return (
        <div className="h-full bg-background rounded-lg overflow-y-auto p-8">
            <div className="max-w-3xl">
                <h1 className="text-2xl font-bold text-white mb-6">Paramètres</h1>

                <div className="bg-gray-300 rounded-lg p-6 mb-4">
                    <h2 className="text-lg font-semibold text-white mb-4">Mon compte</h2>

                    <div className="space-y-4">
                        <div>
                            <label className="text-sm text-gray-light mb-2">Nom d'utilisateur</label>
                            <Input
                                type="text"
                                value={username}
                                onChange={(e) => setUsername(e.target.value)}
                            />
                        </div>

                        <div>
                            <label className="text-sm text-gray-light mb-2">Email</label>
                            <Input
                                type="email"
                                value={email}
                                onChange={(e) => setEmail(e.target.value)}
                            />
                        </div>

                        <Button onClick={handleSave} className="bg-blurple hover:bg-blurple/80 text-white">
                            Enregistrer
                        </Button>
                    </div>
                </div>

                <div className="bg-gray-300 rounded-lg p-6 mb-4">
                    <h2 className="text-lg font-semibold text-white mb-4">Changer le mot de passe</h2>

                    <div className="space-y-4">
                        <div>
                            <label className="text-sm text-gray-light mb-2">Mot de passe actuel</label>
                            <Input
                                type="password"
                                value={currentPassword}
                                onChange={(e) => setCurrentPassword(e.target.value)}
                            />
                        </div>

                        <div>
                            <label className="text-sm text-gray-light mb-2">Nouveau mot de passe</label>
                            <Input
                                type="password"
                                value={newPassword}
                                onChange={(e) => setNewPassword(e.target.value)}
                            />
                        </div>

                        <div>
                            <label className="text-sm text-gray-light mb-2">Confirmer</label>
                            <Input
                                type="password"
                                value={confirmPassword}
                                onChange={(e) => setConfirmPassword(e.target.value)}
                            />
                        </div>

                        <Button onClick={handlePasswordChange} className="bg-blurple hover:bg-blurple/80 text-white">
                            Modifier
                        </Button>
                    </div>
                </div>

                <div className="flex justify-center mt-6">
                    <Button variant="danger" onClick={() => handleLogout()}>
                        Se déconnecter
                    </Button>
                </div>
            </div>
        </div>
    );
}
