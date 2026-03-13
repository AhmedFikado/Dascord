'use client';

import { AuthProvider } from '@/components/auth/auth-provider';
import { SnackbarProvider } from '@/components/shared/error-message';
import { WebSocketProvider } from '@/components/shared/websocket-provider';
import { useAuthToken } from '@/hooks/useAuthToken';
import { AppRouterCacheProvider } from '@mui/material-nextjs/v16-appRouter';
import { ReactNode } from 'react';
import { I18nextProvider } from 'react-i18next';
import i18n from '@/i18n'

function WebSocketWrapper({ children }: { children: ReactNode }) {
  const token = useAuthToken();

  return <WebSocketProvider token={token}>{children}</WebSocketProvider>;
}

export function Providers({ children }: { children: ReactNode }) {
  return (
    <AppRouterCacheProvider>
      <AuthProvider>
        <SnackbarProvider>
            <I18nextProvider i18n={i18n}>
            <WebSocketWrapper>{children}</WebSocketWrapper>
            </I18nextProvider>
        </SnackbarProvider>
      </AuthProvider>
    </AppRouterCacheProvider>
  );
}
