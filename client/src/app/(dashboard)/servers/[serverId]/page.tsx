'use client';

import { use } from 'react';
import { useRouter } from 'next/navigation';
import { useEffect } from 'react';

export default function ServerPage({
    params
}: {
    params: Promise<{ serverId: string }>
}) {
    const { serverId } = use(params);
    const router = useRouter();

    useEffect(() => {
        router.push(`/servers/${serverId}/channels/1`);
    }, [serverId, router]);

    return null;
}