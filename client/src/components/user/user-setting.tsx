'use client';

import { useState, useRef } from 'react';
import { Button } from '../ui/button';
import { Input } from '../ui/input';
import { User } from '@/types/models/user';
import { useSnackbar } from "@/components/shared/error-message";
import { useAuthStore } from '@/app/lib/stores/use-auth-store';
import { useRouter } from 'next/navigation';
import { Dropdown } from '@/components/ui/dropdown';
import { updateUserInfo, uploadAvatar, getMe } from '@/app/lib/api/users';
import i18n from '@/i18n';
import { useTranslation } from 'react-i18next';
import UserCard from '../shared/user-card';

interface UserSettingProps {
    user: User;
}

export default function UserSetting({ user }: UserSettingProps) {
    const [username, setUsername] = useState(user.username);
    const [email, setEmail] = useState(user.email);
    const [language, setLanguage] = useState(user.language);
    const [avatarPreview, setAvatarPreview] = useState<string | null>(null);
    const [selectedFile, setSelectedFile] = useState<File | null>(null);
    const fileInputRef = useRef<HTMLInputElement>(null);
    const { showSnackbar } = useSnackbar();
    const authStore = useAuthStore();
    const router = useRouter();
    const { t } = useTranslation();

    const handleAvatarClick = () => {
        fileInputRef.current?.click();
    };

    const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const file = e.target.files?.[0];
        if (file && ['image/png', 'image/jpeg', 'image/webp'].includes(file.type)) {
            setSelectedFile(file);
            const reader = new FileReader();
            reader.onloadend = () => {
                setAvatarPreview(reader.result as string);
            };
            reader.readAsDataURL(file);
        }
    };

    const handleConfirmAvatar = async () => {
        if (!selectedFile) return;
        try {
            await uploadAvatar(selectedFile);
            const updatedUser = await getMe();
            authStore.setUser(updatedUser);
            showSnackbar({ message: t('User_setting.profile_updated'), severity: "success" });
            setAvatarPreview(null);
            setSelectedFile(null);
        } catch (error) {
            showSnackbar({ message: t('User_setting.update_error'), severity: "error" });
        }
    };

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
                            <label className="text-sm text-gray-light mb-2 block">{t('User_setting.avatar')}</label>
                            <div className="flex items-center justify-center gap-4 p-4 rounded-lg">
                                <div
                                    onClick={handleAvatarClick}
                                    className="relative cursor-pointer group"
                                >
                                    <div className="w-20 h-20 flex items-center justify-center rounded-full overflow-hidden border-2 border-gray-100 group-hover:border-purple transition-colors">
                                        {avatarPreview ? (
                                            <img
                                                src={avatarPreview}
                                                alt="Avatar preview"
                                                className="w-full h-full object-cover"
                                            />
                                        ) : (
                                            <div className="scale-[2.5]">
                                                <UserCard username={user.username} avatarId={user.avatar_id} />
                                            </div>
                                        )}
                                    </div>
                                    <div className="absolute inset-0 bg-black/50 rounded-full opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center text-center px-2">
                                        <span className="text-white text-xs font-medium leading-tight">{t('User_setting.change_avatar')}</span>
                                    </div>
                                </div>
                                <input
                                    ref={fileInputRef}
                                    type="file"
                                    accept="image/png,image/jpeg,image/webp"
                                    onChange={handleFileChange}
                                    className="hidden"
                                />
                                {avatarPreview && (
                                    <Button
                                        onClick={handleConfirmAvatar}
                                        className="bg-green-500 hover:bg-green-600 text-white"
                                    >
                                        {t('User_setting.confirm_avatar')}
                                    </Button>
                                )}
                            </div>
                        </div>

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
