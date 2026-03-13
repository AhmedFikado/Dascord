'use client';

import { useState } from 'react';
import { Button } from '../ui/button';
import { Input } from '../ui/input';
import { User, Status } from '@/types/models/user';
import { useSnackbar } from "@/components/shared/error-message";
import { useAuthStore } from '@/app/lib/stores/use-auth-store';
import { useRouter } from 'next/navigation';
import { Dropdown } from '@/components/ui/dropdown';
import { updateUserInfo } from '@/app/lib/api/users';
import i18n from '@/i18n';
import { useTranslation } from 'react-i18next';

interface UserSettingProps {
    user: User;
}

export default function UserSetting({ user }: UserSettingProps) {
    const [username, setUsername] = useState(user.username);
    const [email, setEmail] = useState(user.email);
    const [language, setLanguage] = useState(user.language);
    const { showSnackbar } = useSnackbar();
    const authStore = useAuthStore();
    const router = useRouter();
    const { t } = useTranslation();

    const handleSave = async () => {
        if (!username.trim() || !email.trim()) {
            showSnackbar({ message: t('User_setting.fill_all_fields'), severity: "error" });
            return;
        }
        try {
            const updatedUser = await updateUserInfo({ username, email, language });
            authStore.setUser({
                ...user,
                username: updatedUser.username,
                email: updatedUser.email,
                language: updatedUser.language
            });
            showSnackbar({ message: t('User_setting.profile_updated'), severity: "success" });
        } catch {
            showSnackbar({ message: t('User_setting.update_error'), severity: "error" });
        }
    };

    const handleSaveLanguage = async (language: string) => {
        console.log(language);
        try {
            const updatedUser = await updateUserInfo({ username, email, language });
            authStore.setUser({
                ...user,
                language: updatedUser.language
            });
            console.table(updatedUser);
            setLanguage(language);
            i18n.changeLanguage(language);
            showSnackbar({ message: t('User_setting.profile_updated'), severity: "success" });
        }
        catch {
            showSnackbar({ message: t('User_setting.update_error'), severity: "error" });
        }
    };

    const handleLogout = () => {
        authStore.logout();
        router.push('/login');
    }

    return (
        <div className="h-full bg-background rounded-lg overflow-y-auto p-8">
            <div className="max-w-3xl">
                <h1 className="text-2xl font-bold text-white mb-6">{t('User_setting.settings')}</h1>

                <div className="bg-gray-300 rounded-lg p-6 mb-4">
                    <h2 className="text-lg font-semibold text-white mb-4">{t('User_setting.my_account')}</h2>

                    <div className="space-y-4">
                        <div>
                            <label className="text-sm text-gray-light mb-2">{t('User_setting.username')}</label>
                            <Input
                                type="text"
                                value={username}
                                onChange={(e) => setUsername(e.target.value)}
                            />
                        </div>

                        <div>
                            <label className="text-sm text-gray-light mb-2">{t('User_setting.email')}</label>
                            <Input
                                type="email"
                                value={email}
                                onChange={(e) => setEmail(e.target.value)}
                            />
                        </div>

                        <Button onClick={handleSave} className="bg-blurple hover:bg-blurple/80 text-white">
                            {t('User_setting.save')}
                        </Button>
                    </div>
                </div>

                <div className="bg-gray-300 rounded-lg p-6 mb-4">
                    <h2 className="text-lg font-semibold text-white mb-4">{t('User_setting.my_language')}</h2>

                    <div>
                        <Dropdown
                            label={t('Settings.language')}
                            options={[
                                { label: 'Français', value: 'fr' },
                                { label: 'English', value: 'en' },
                            ]}
                            value={language}
                            placeholder={t('Settings.language')}
                            onChange={handleSaveLanguage}
                        ></Dropdown>
                    </div>


                </div>

                <div className="flex justify-center mt-6">
                    <Button variant="danger" onClick={() => handleLogout()}>
                        {t('User_setting.logout')}
                    </Button>
                </div>
            </div>
        </div>
    );
}
