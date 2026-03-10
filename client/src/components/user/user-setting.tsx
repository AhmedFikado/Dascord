'use client';

import { useState } from 'react';
import { Button } from '../ui/button';
import { Input } from '../ui/input';
import { User, Status } from '@/types/models/user';
import { useSnackbar } from "@/components/shared/error-message";
import { useAuthStore } from '@/app/lib/stores/use-auth-store';
import { useRouter } from 'next/navigation';
import { updateUserInfo } from '@/app/lib/api/users';

interface UserSettingProps {
    user: User;
}

export default function UserSetting({ user }: UserSettingProps) {
    const [username, setUsername] = useState(user.username);
    const [email, setEmail] = useState(user.email);
    const { showSnackbar } = useSnackbar();
    const authStore = useAuthStore();
    const router = useRouter();

    const handleSave = async () => {
        if (!username.trim() || !email.trim()) {
            showSnackbar({ message: "Veuillez remplir tous les champs", severity: "error" });
            return;
        }
        try {
            const updatedUser = await updateUserInfo({ username, email });
            authStore.setUser({
                ...user,
                username: updatedUser.username,
                email: updatedUser.email
            });
            console.table(updatedUser);
            showSnackbar({ message: "Profil mis à jour", severity: "success" });
        } catch {
            showSnackbar({ message: "Erreur lors de la mise à jour", severity: "error" });
        }
    };

    const handleLogout = () => {
        authStore.logout();
        router.push('/login');
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

                <div className="flex justify-center mt-6">
                    <Button variant="danger" onClick={() => handleLogout()}>
                        Se déconnecter
                    </Button>
                </div>
            </div>
        </div>
    );
}
