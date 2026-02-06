'use client';

import { useRouter } from 'next/navigation';
import { useEffect } from 'react';
import { Loading as LoadingSpinner } from '../components/shared/loading-spinner';

export default function Home() {
    const router = useRouter();

    useEffect(() => {
        router.push('/servers');
    }, [router]);

    return (
        <div className="flex items-center justify-center h-screen">
            <LoadingSpinner size="lg" />
        </div>
    );
}