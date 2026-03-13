import { useTranslation } from 'react-i18next';

export default function TypingIndicator({ username }: { username: string }) {
    const { t } = useTranslation();

    return (
        <div>
            <p className="text-gray-light ml-6 text-xs mb-1">{username} {t('Typing_indicator.is_typing...')}</p>
        </div>
    );
}