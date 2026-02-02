'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import Link from 'next/link';
import { useAuthStore } from '@/app/lib/stores/use-auth-store';
import { signupSchema, type SignupFormData } from '@/app/lib/api/validations/auth.schema';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';

export default function SignupForm() {
    const router = useRouter();
    const { signup, isLoading, error: authError, clearError } = useAuthStore();

    const [formData, setFormData] = useState<SignupFormData>({
        username: '',
        email: '',
        password: '',
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
        <div className="bg-gray-300 p-8 rounded-lg shadow-md">
            <h1 className="text-2xl font-bold text-center mb-6 text-white">Inscription</h1>

            {authError && (
                <div className="mb-4 p-3 bg-red-100 border border-red-400 text-red-700 rounded">
                    {authError}
                </div>
            )}

            <form onSubmit={handleSubmit} className="space-y-4 w-80">

                <div>
                    <Input
                        label="Username"
                        type="text"
                        required
                        id="username"
                        name="username"
                        value={formData.username}
                        onChange={handleChange}
                        placeholder="votre nom d'utilisateur"
                    />
                    {errors.username && (
                        <p className="mt-1 text-sm text-red-600">{errors.username}</p>
                    )}
                </div>

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
                        {isLoading ? 'Inscription...' : "S'inscrire"}
                    </Button>
                </div>
            </form>

            <p className="mt-4 text-center text-sm text-white">
                Déjà un compte ?{' '}
                <Link href="/login" className="text-blue-600 hover:underline">
                    Se connecter
                </Link>
            </p>
        </div>
    );
}