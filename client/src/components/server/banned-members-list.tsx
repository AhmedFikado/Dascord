'use client';

import { useEffect, useState, useCallback } from 'react';
import { Button } from '@/components/ui/button';
import { BannedMember } from '@/types/models/BannedMember';
import { useServerStore } from '@/app/lib/stores/use-server-store';
import { useTranslation } from 'react-i18next';

interface BannedMembersListProps {
    serverId: string;
}

function formatTimeRemaining(expiresAt: string, t: (key: string) => string): string {
    const now = new Date();
    const expiry = new Date(expiresAt);
    const diffMs = expiry.getTime() - now.getTime();

    if (diffMs <= 0) return t('Server_settings.ban_expired');

    const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24));
    const diffHours = Math.floor((diffMs % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60));
    const diffMinutes = Math.floor((diffMs % (1000 * 60 * 60)) / (1000 * 60));

    if (diffDays > 0) return `${diffDays}j ${diffHours}h`;
    if (diffHours > 0) return `${diffHours}h ${diffMinutes}min`;
    return `${diffMinutes}min`;
}

export default function BannedMembersList({ serverId }: BannedMembersListProps) {
    const { t } = useTranslation();
    const [bannedMembers, setBannedMembers] = useState<BannedMember[]>([]);
    const [isLoading, setIsLoading] = useState(true);
    const [unbanning, setUnbanning] = useState<string | null>(null);
    const { getBannedMembers, unbanMember } = useServerStore();

    const loadBannedMembers = useCallback(async () => {
        try {
            setIsLoading(true);
            const data = await getBannedMembers(serverId);
            setBannedMembers(data);
        } catch {
            setBannedMembers([]);
        } finally {
            setIsLoading(false);
        }
    }, [serverId, getBannedMembers]);

    useEffect(() => {
        loadBannedMembers();
    }, [loadBannedMembers]);

    const handleUnban = async (userId: string) => {
        setUnbanning(userId);
        try {
            await unbanMember(serverId, userId);
            setBannedMembers(prev => prev.filter(m => m.user_id !== userId));
        } finally {
            setUnbanning(null);
        }
    };

    if (isLoading) {
        return (
            <div className="flex items-center justify-center py-6">
                <div className="w-5 h-5 border-2 border-blurple border-t-transparent rounded-full animate-spin" />
            </div>
        );
    }

    if (bannedMembers.length === 0) {
        return (
            <p className="text-gray-50 text-sm text-center py-4">
                {t('Server_settings.no_banned_members')}
            </p>
        );
    }

    return (
        <div className="space-y-2">
            {bannedMembers.map((member) => (
                <div
                    key={member.user_id}
                    className="flex items-center justify-between p-3 bg-gray-300 rounded-lg"
                >
                    <div className="flex flex-col min-w-0">
                        <span className="text-white text-sm font-medium truncate">
                            {member.username}
                        </span>
                        <span className="text-gray-50 text-xs">
                            {t('Server_settings.banned_at')} {new Date(member.banned_at).toLocaleDateString()}
                        </span>
                        {member.ban_type === 'Temporary' && member.expires_at && (
                            <span className="text-yellow-400 text-xs">
                                {t('Server_settings.unban_in')} {formatTimeRemaining(member.expires_at, t)}
                            </span>
                        )}
                        {member.ban_type === 'Permanent' && (
                            <span className="text-red-400 text-xs">
                                {t('Server_settings.ban_permanent')}
                            </span>
                        )}
                    </div>
                    <Button
                        variant="secondary"
                        onClick={() => handleUnban(member.user_id)}
                        isLoading={unbanning === member.user_id}
                        className="ml-3 shrink-0 text-xs"
                    >
                        {t('Server_settings.unban')}
                    </Button>
                </div>
            ))}
        </div>
    );
}
