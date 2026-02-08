import type { ReactNode } from 'react';

export default function AuthLayout({ children }: { children: ReactNode }) {
    return (
        <div className="min-h-screen flex items-center justify-center bg-white">
            <div className="max-w-md w-full space-y-8 p-8">
                {children}
            </div>
        </div>
    );
}
