'use client';

import { use } from 'react';

export default function ServerSettingsPage({
    params
}: {
    params: Promise<{ serverId: string }>
}) {
    const { serverId } = use(params);

    return (
        <div className="flex-1 p-8">
            <h1 className="text-white text-2xl font-bold mb-4">
                Paramètres du serveur
            </h1>
        </div>
    );
}