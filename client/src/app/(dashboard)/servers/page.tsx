'use client';

export default function ServersPage() {
    return (
        <div className="flex-1 flex items-center justify-center bg-background">
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