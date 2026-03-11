'use client';

import { useMobileNav } from '@/app/(dashboard)/layout';
import { Menu } from 'lucide-react';

export default function ServersPage() {

    const { openNav } = useMobileNav();

    return (
        <div className="h-full flex items-center justify-center bg-background">

            <button
                className="md:hidden mr-3 p-1 flex-shrink-0 absolute top-4 left-4 z-10"
                onClick={openNav}
                aria-label="Ouvrir la navigation"
            >
                <Menu size={20} className="text-white" />
            </button>

            <div className="text-center">
                <h2 className="text-2xl font-semibold text-gray-50 mb-2">
                    Aucun serveur sélectionné
                </h2>
                <p className="text-gray-100">
                    Sélectionnez un serveur dans la barre latérale pour commencer
                </p>
            </div>
        </div>
    );
}