'use client';

import { Server } from "@/types/models/Server";

export default function ServerItem({ server }: { server: Server }) {

    const getInitials = (name: string): string => {
        const words = name.trim().split(' ');

        if (words.length === 1) {
            return words[0][0].toUpperCase();
        } else {
            return words
                .slice(0, 2)
                .map(word => word[0].toUpperCase())
                .join('');
        }
    };

    return (
        <div
            className="w-12 h-12 rounded-2xl bg-gray-300 flex items-center justify-center text-white font-bold text-lg cursor-pointer mb-2"
            title={server.name}
        >
            {getInitials(server.name)}
        </div>
    );
}