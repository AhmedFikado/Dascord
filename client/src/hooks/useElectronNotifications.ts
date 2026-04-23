"use client";

import { useEffect } from "react";
import { useWebSocketStore } from "@/store/websocket";
import { useAuthStore } from "@/app/lib/stores/use-auth-store";

export function useElectronNotifications() {
  const messagesByChannel = useWebSocketStore((s) => s.messagesByChannel);
  const currentUserId = useAuthStore((s) => s.userId);

  useEffect(() => {
    if (typeof window === "undefined" || !window.electronAPI) return;

    const allMessages = Object.values(messagesByChannel).flat();
    if (allMessages.length === 0) return;

    const latest = allMessages[allMessages.length - 1];
    if (!latest || latest.user_id === currentUserId) return;

    // Only notify when the document is hidden (app is in background)
    if (!document.hidden) return;

    window.electronAPI.notifyMessage(
      latest.username,
      latest.content.slice(0, 100)
    );
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [messagesByChannel]);
}
