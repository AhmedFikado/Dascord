'use client';

import UserCard from '@/components/shared/user-card';
import { Button } from "@/components/ui/button";
import { CircleUser } from 'lucide-react';
import { Status } from '@/types/models/status';
import { useTranslation } from 'react-i18next';

interface UserPanelBarProps {
    username: string;
    status: Status;
    onClick: () => void;
}

export default function UserPanelBar({ username, status, onClick }: UserPanelBarProps) {
    const { t } = useTranslation();
    const getStatusLabel = (status: Status) => {
        switch (status) {
            case Status.ONLINE: return t('User_panel_bar.online');
            case Status.OFFLINE: return t('User_panel_bar.offline');
        }
    };

    return (
        <div
            onClick={onClick}
            className="fixed bottom-2 rounded-2xl pl-3 py-1 left-3 gap-3 bg-gray-400 w-72 flex items-center justify-between z-50 cursor-pointer hover:bg-gray-300 transition-colors"
        >
            <div className="flex items-center gap-3">
                <UserCard username={username} />
                <div className="flex-1 min-w-0">
                    <span className="font-semibold text-white">{username}</span>
                    <div className='flex'>
                        <p className="text-xs text-gray-light">
                            {(status === Status.ONLINE) ? (
                                <span className="ml-1 h-2 w-2 bg-green-500 rounded-full inline-block"></span>
                            ) : (
                                <span className="ml-1 h-2 w-2 bg-gray-500 rounded-full inline-block"></span>
                            )}
                        </p>
                        <p className="text-xs text-gray-light ml-1">
                            {getStatusLabel(status)}
                        </p>
                    </div>
                </div>
            </div>
            <Button
                variant="noBackground"
                width="50px"
                height="50px"
            >
                <CircleUser color="#adadad" size={24} />
            </Button>
        </div>
    );
}