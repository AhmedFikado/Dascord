import { getToken } from '@/app/lib/api/auth/token';
import { useState } from 'react';

export function useAuthToken() {
  // Initialiser directement avec la valeur du cookie auth_token
  const [token] = useState<string | null>(() => {
    if (typeof window === 'undefined') return null;
    return getToken();
  });

  return token;
}
