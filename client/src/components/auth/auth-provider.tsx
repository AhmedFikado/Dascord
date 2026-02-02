'use client';

import { useEffect } from 'react';
import { initializeSession } from '@/app/lib/api/auth/session';

export function AuthProvider({ children }: { children: React.ReactNode }) {
    useEffect(() => {
        initializeSession();
    }, []);

    return <>{children}</>;
}
