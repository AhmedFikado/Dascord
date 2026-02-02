'use client';

import { useRouter } from 'next/navigation';
import { useEffect } from 'react';

export default function ServersPage() {
    const router = useRouter();

    useEffect(() => {
        router.push('/servers/1/channels/1');
    }, [router]);

    return null;
}