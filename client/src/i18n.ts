import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';
import { getMe } from './app/lib/api/users';

import fr from '../public/locales/fr/common.json';
import en from '../public/locales/en/common.json';



i18n.use(initReactI18next).init({
    resources: {
        fr: { common: fr },
        en: { common: en },
    },
    lng: 'fr',
    fallbackLng: 'fr',
    ns: ['common'],
    defaultNS: 'common',
    interpolation: {
        escapeValue: false,
    },
});

getMe().then(user => {
    if (user?.language) {
        i18n.changeLanguage(user.language);
    }
}).catch(() => {});

export default i18n;