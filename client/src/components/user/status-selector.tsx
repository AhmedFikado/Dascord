'use client';

import { Circle } from 'lucide-react';
import { Status } from '@/types/models/status';
import { useTranslation } from 'react-i18next';

interface StatusSelectorProps {
    currentStatus: Status;
    onStatusChange: (status: Status) => void;
}

export default function StatusSelector({ currentStatus, onStatusChange }: StatusSelectorProps) {
    const { t } = useTranslation();
    const statuses = [
        { value: Status.ONLINE, label: t('Status_selector.online'), color: 'text-green' },
        { value: Status.OFFLINE, label: t('Status_selector.offline'), color: 'text-gray-50' },
        { value: Status.INACTIVE, label: t('Status_selector.inactive'), color: 'text-orange-400' },
        { value: Status.DONOTDISTURB, label: t('Status_selector.do_not_disturb'), color: 'text-red-400' },
    ];

    return (
        <div className="p-2 border-b border-gray-200">
            <p className="text-xs text-gray-light px-2 mb-2 uppercase font-semibold">
                {t('Status_selector.set_status')}
            </p>
            {statuses.map(({ value, label, color }) => (
                <button
                    key={value}
                    onClick={() => onStatusChange(value)}
                    className="w-full flex items-center gap-3 px-3 py-2 hover:bg-hoverSide rounded-md transition-colors"
                >
                    <Circle
                        size={12}
                        className={`${color} ${value === currentStatus ? 'fill-current' : ''}`}
                    />
                    <span className="text-white text-sm">{label}</span>
                </button>
            ))}
        </div>
    );
}