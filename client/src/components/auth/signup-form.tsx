'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import Link from 'next/link';
import { useAuthStore } from '@/app/lib/stores/use-auth-store';
import { signupSchema, type SignupFormData } from '@/app/lib/api/validations/auth.schema';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Dropdown } from '@/components/ui/dropdown';
import i18n from '@/i18n';
import { useTranslation } from 'react-i18next';

export default function SignupForm() {
    const router = useRouter();
    const { signup, isLoading, error: authError, clearError } = useAuthStore();
    const { t } = useTranslation();

    const [formData, setFormData] = useState<SignupFormData>({
        username: '',
        email: '',
        password: '',
        language: 'fr',
    });
    const [errors, setErrors] = useState<Partial<Record<keyof SignupFormData, string>>>({});

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { name, value } = e.target;
        setFormData(prev => ({ ...prev, [name]: value }));
        if (errors[name as keyof SignupFormData]) {
            setErrors(prev => ({ ...prev, [name]: undefined }));
        }
        clearError();
    };

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        setErrors({});
        clearError();

        const result = signupSchema.safeParse(formData);

        if (!result.success) {
            const fieldErrors: Partial<Record<keyof SignupFormData, string>> = {};
            result.error.issues.forEach((err) => {
                const field = err.path[0] as keyof SignupFormData;
                fieldErrors[field] = err.message;
            });
            setErrors(fieldErrors);
            return;
        }

        try {
            await signup(formData);
            router.push('/servers');
        } catch (error) {
        }
    };

    return (
        <div className="bg-gray-300 p-8 rounded-lg shadow-md w-full">
            <h1 className="text-2xl font-bold text-center mb-6 text-white">{t('Sign_Up.title')}</h1>

            {authError && (
                <div className="mb-4 p-3 bg-red-100 border border-red-400 text-red-700 rounded">
                    {authError}
                </div>
            )}

            <form onSubmit={handleSubmit} className="space-y-4 w-full">

                <div>
                    <Input
                        label={t('Sign_Up.username')}
                        type="text"
                        required
                        id="username"
                        name="username"
                        value={formData.username}
                        onChange={handleChange}
                        placeholder={t('Sign_Up.username')}
                    />
                    {errors.username && (
                        <p className="mt-1 text-sm text-red-600">{errors.username}</p>
                    )}
                </div>

                <div>
                    <Input
                        label={t('Sign_Up.email')}
                        type="email"
                        required
                        id="email"
                        name="email"
                        value={formData.email}
                        onChange={handleChange}
                        placeholder="email@email.com"
                    />
                    {errors.email && (
                        <p className="mt-1 text-sm text-red-600">{errors.email}</p>
                    )}
                </div>

                <div>
                    <Input
                        label={t('Sign_Up.password')}
                        type="password"
                        required
                        id="password"
                        name="password"
                        value={formData.password}
                        onChange={handleChange}
                        placeholder="••••••••"
                    />
                    {errors.password && (
                        <p className="mt-1 text-sm text-red-600">{errors.password}</p>
                    )}
                </div>

                <div>
                    <Dropdown
                        label={t('Sign_Up.language')}
                        options={[
                            { label: 'Français', value: 'fr' },
                            { label: 'English', value: 'en' },
                        ]}
                        value={formData.language}
                        placeholder={t('Sign_Up.language')}
                        onChange={(value) => {
                            setFormData(prev => ({ ...prev, language: value }));
                            i18n.changeLanguage(value);
                        }}
                    ></Dropdown>

                </div>

                <div className="flex justify-center">
                    <Button
                        type="submit"
                        disabled={isLoading}
                        variant='primary'
                        height='38px'
                    >
                        {isLoading ? t('Sign_Up.inscription') : t('Sign_Up.title')}
                    </Button>
                </div>
            </form>

            <p className="mt-4 text-center text-sm text-white">
                {t('Sign_Up.login_link')}{' '}
                <Link href="/login" className="text-blue-600 hover:underline">
                    {t('Sign_Up.login')}
                </Link>
            </p>
        </div>
    );
}