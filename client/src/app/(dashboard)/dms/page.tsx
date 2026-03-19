'use client';

import DMPageHeader from '@/components/dm/dm-page-header';
import { useTranslation } from 'react-i18next';

export default function DMsPage() {
  const { t } = useTranslation();
  return (
    <div className="flex-1 flex flex-col bg-background h-full min-w-0">
      <DMPageHeader />
      <div className="flex-1 flex items-center justify-center">
        <p className="text-gray-light text-xl">
          {t('DM.select_conversation')}
        </p>
      </div>
    </div>
  );
}
