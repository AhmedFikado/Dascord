'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import Link from 'next/link';
import { useAuthStore } from '@/app/lib/stores/use-auth-store';
import { loginSchema, type LoginFormData } from '@/app/lib/api/validations/auth.schema';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';

export default function LoginForm() {
    const router = useRouter();
    const { login, isLoading, error: authError, clearError } = useAuthStore();

    const [formData, setFormData] = useState<LoginFormData>({
        email: '',
        password: '',
    });
    const [errors, setErrors] = useState<Partial<Record<keyof LoginFormData, string>>>({});

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { name, value } = e.target;
        setFormData(prev => ({ ...prev, [name]: value }));
        if (errors[name as keyof LoginFormData]) {
            setErrors(prev => ({ ...prev, [name]: undefined }));
        }
        clearError();
    };

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        setErrors({});
        clearError();

        const result = loginSchema.safeParse(formData);

        if (!result.success) {
            const fieldErrors: Partial<Record<keyof LoginFormData, string>> = {};
            result.error.issues.forEach((err) => {
                const field = err.path[0] as keyof LoginFormData;
                fieldErrors[field] = err.message;
            });
            setErrors(fieldErrors);
            return;
        }

        try {
            await login(formData);
            router.push('/servers');
        } catch (error) {
        }
    };

    return (
        <div className="bg-gray-300 p-8 rounded-lg shadow-md w-full">
            <h1 className="text-2xl font-bold text-center mb-6 text-white">Connexion</h1>

            {authError && (
                <div className="mb-4 p-3 bg-red-100 border border-red-400 text-red-700 rounded">
                    {authError}
                </div>
            )}

            <form onSubmit={handleSubmit} className="space-y-4 w-full">
                <div>
                    <Input
                        label="Email"
                        type="email"
                        required
                        id="email"
                        name="email"
                        value={formData.email}
                        onChange={handleChange}
                        placeholder="votre@email.com"
                    />
                    {errors.email && (
                        <p className="mt-1 text-sm text-red-600">{errors.email}</p>
                    )}
                </div>

                <div>
                    <Input
                        label="Mot de passe"
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

                <div className="flex justify-center">
                    <Button
                        type="submit"
                        disabled={isLoading}
                        variant='primary'
                        height='38px'
                    >
                        {isLoading ? 'Connexion...' : 'Se connecter'}
                    </Button>
                </div>
            </form>

            <p className="mt-4 text-center text-sm text-white">
                Pas encore de compte ?{' '}
                <Link href="/signup" className="text-blue-600 hover:underline">
                    S'inscrire
                </Link>
            </p>
        </div>
    );
}