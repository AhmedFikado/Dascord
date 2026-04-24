"use client";

import { useEffect, useRef } from "react";
import { useWebSocketStore } from "@/store/websocket";

export function useElectronNotifications() {
  const setNotificationCallback = useWebSocketStore((s) => s.setNotificationCallback);
  // Start as false — isWindowFocused() will correct it synchronously on mount.
  // This avoids missing notifications in the brief window before the Promise resolves.
  const windowFocusedRef = useRef(false);

  useEffect(() => {
    if (typeof window === "undefined" || !window.electronAPI) return;

    window.electronAPI.isWindowFocused().then((focused) => {
      windowFocusedRef.current = focused;
    });

    const unsubscribe = window.electronAPI.onWindowFocusChanged((focused) => {
      windowFocusedRef.current = focused;
    });

    return unsubscribe;
  }, []);

  useEffect(() => {
    if (typeof window === "undefined" || !window.electronAPI) return;

    setNotificationCallback((senderName, preview) => {
      if (windowFocusedRef.current) return;
      window.electronAPI!.notifyMessage(senderName, preview);
    });

    return () => setNotificationCallback(null);
  }, [setNotificationCallback]);
}
