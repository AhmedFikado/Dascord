import { getToken } from '@/app/lib/api/auth/token';
import { useEffect, useState } from 'react';

export function useAuthToken() {
  const [token, setToken] = useState<string | null>(() => {
    if (typeof window === 'undefined') return null;
    return getToken();
  });

  useEffect(() => {
    const interval = setInterval(() => {
      const currentToken = getToken();
      setToken(currentToken);
    }, 1000);

    return () => clearInterval(interval);
  }, []);

  return token;
}
