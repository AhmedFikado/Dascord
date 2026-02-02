'use client';

import { AppRouterCacheProvider } from '@mui/material-nextjs/v15-appRouter';
import { SnackbarProvider } from '@/components/shared/error-message';
import { ReactNode } from 'react';

export function Providers({ children }: { children: ReactNode }) {
    return (
        <AppRouterCacheProvider>
            <SnackbarProvider>
                {children}
            </SnackbarProvider>
        </AppRouterCacheProvider>
    );
}