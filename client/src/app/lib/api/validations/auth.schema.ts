import { z } from 'zod';

export const loginSchema = z.object({
    email: z
        .string()
        .min(1, 'L\'email est requis')
        .email('Format d\'email invalide'),
    password: z
        .string()
        .min(1, 'Le mot de passe est requis'),
});

export const signupSchema = z.object({
    username: z
        .string()
        .min(3, 'Le nom d\'utilisateur doit contenir au moins 3 caractères')
        .max(50, 'Le nom d\'utilisateur ne peut pas dépasser 50 caractères'),
    email: z
        .string()
        .min(1, 'L\'email est requis')
        .email('Format d\'email invalide'),
    password: z
        .string()
        .min(8, 'Le mot de passe doit contenir au moins 8 caractères'),
});

export type LoginFormData = z.infer<typeof loginSchema>;
export type SignupFormData = z.infer<typeof signupSchema>;
