import { useWebSocketStore } from '@/store/websocket';
import { ClientMessage, ServerMessage, WebSocketConfig, WebSocketStatus } from '@/types/websocket';
import { useCallback, useEffect, useRef } from 'react';

//reconnectInterval: Délai entre chaque tentative de reconnexion (3 secondes)
//maxReconnectAttempts: Nombre maximum de tentatives de reconnexion (5)
const DEFAULT_RECONNECT_INTERVAL = 3000;
const DEFAULT_MAX_ATTEMPTS = 5;

export function useWebSocket(token: string | null, config?: WebSocketConfig) {
  const wsRef = useRef<WebSocket | null>(null);
  const reconnectTimeoutRef = useRef<NodeJS.Timeout | undefined>(undefined);
  const reconnectAttemptsRef = useRef(0);

  const { setStatus, setError, handleServerMessage, reset } = useWebSocketStore();

  // Nettoyer tous les timeouts actifs
  const clearTimeouts = useCallback(() => {
    if (reconnectTimeoutRef.current) {
      clearTimeout(reconnectTimeoutRef.current);
    }
  }, []);

  // Envoyer un message au serveur via WebSocket
  const sendMessage = useCallback((message: ClientMessage) => {
    if (wsRef.current?.readyState === WebSocket.OPEN) {
      wsRef.current.send(JSON.stringify(message));
    }
  }, []);

  // Établir la connexion WebSocket
  const connect = useCallback(() => {
    if (!token) return;
    if (wsRef.current?.readyState === WebSocket.OPEN) return;

    const reconnectInterval = config?.reconnectInterval || DEFAULT_RECONNECT_INTERVAL;
    const maxAttempts = config?.maxReconnectAttempts || DEFAULT_MAX_ATTEMPTS;
    const wsUrl = config?.url || getWebSocketUrl();

    try {
      const url = `${wsUrl}?token=${token}`;
      setStatus(
        reconnectAttemptsRef.current > 0 ? WebSocketStatus.RECONNECTING : WebSocketStatus.CONNECTING
      );

      const ws = new WebSocket(url);
      wsRef.current = ws;

      // Connexion établie avec succès
      ws.onopen = () => {
        setStatus(WebSocketStatus.CONNECTED);
        setError(null);
        reconnectAttemptsRef.current = 0;
      };

      // Réception d'un message du serveur
      ws.onmessage = async event => {
        try {
          const message: ServerMessage = JSON.parse(event.data);
          handleServerMessage(message);

          if (message.type === 'NewMessage') {
            try {
              const { isPermissionGranted, requestPermission, sendNotification } =
                await import('@tauri-apps/plugin-notification');
              const granted =
                (await isPermissionGranted()) || (await requestPermission()) === 'granted';
              if (granted) {
                sendNotification({
                  title: message.payload.username,
                  body: message.payload.content,
                });
              }
            } catch (e) {
              console.error('Notification error:', e);
            }
          }
        } catch (error) {
          console.error('Failed to parse WebSocket message:', error);
        }
      };

      // Erreur de connexion
      ws.onerror = () => {
        setStatus(WebSocketStatus.ERROR);
        setError('WebSocket connection error');
      };

      // Connexion fermée
      ws.onclose = event => {
        clearTimeouts();

        // Tenter une reconnexion si la fermeture n'était pas volontaire
        if (!event.wasClean && reconnectAttemptsRef.current < maxAttempts) {
          reconnectAttemptsRef.current++;
          reconnectTimeoutRef.current = setTimeout(() => {
            connect();
          }, reconnectInterval);
          setStatus(WebSocketStatus.RECONNECTING);
        } else {
          // Abandon après trop de tentatives ou fermeture volontaire
          setStatus(WebSocketStatus.DISCONNECTED);
          if (reconnectAttemptsRef.current >= maxAttempts) {
            setError('Max reconnection attempts reached');
          }
        }
      };
    } catch {
      setStatus(WebSocketStatus.ERROR);
      setError('Failed to create WebSocket connection');
    }
  }, [token, config, setStatus, setError, handleServerMessage, clearTimeouts]);

  // Fermer la connexion WebSocket
  const disconnect = useCallback(() => {
    clearTimeouts();
    if (wsRef.current) {
      wsRef.current.close();
      wsRef.current = null;
    }
    reset();
  }, [clearTimeouts, reset]);

  // Rejoindre un channel (écouter les messages)
  const joinChannel = useCallback(
    (channelId: string) => {
      sendMessage({
        type: 'JoinChannel',
        payload: { channel_id: channelId },
      });
    },
    [sendMessage]
  );

  // Quitter un channel (arrêter d'écouter)
  const leaveChannel = useCallback(
    (channelId: string) => {
      sendMessage({
        type: 'LeaveChannel',
        payload: { channel_id: channelId },
      });
    },
    [sendMessage]
  );

  // Envoyer un message dans un channel
  const sendChannelMessage = useCallback(
    (channelId: string, content: string) => {
      sendMessage({
        type: 'SendMessage',
        payload: { channel_id: channelId, content },
      });
    },
    [sendMessage]
  );

  // Envoyer l'indicateur "en train de taper"
  const sendTyping = useCallback(
    (channelId: string, isTyping: boolean) => {
      sendMessage({
        type: 'Typing',
        payload: { channel_id: channelId, is_typing: isTyping },
      });
    },
    [sendMessage]
  );

  // Connexion automatique quand le token est disponible
  useEffect(() => {
    if (token) {
      connect();
    }
    return () => {
      disconnect();
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [token]);

  return {
    connect,
    disconnect,
    joinChannel,
    leaveChannel,
    sendChannelMessage,
    sendTyping,
  };
}

//Construire l'URL WebSocket pour le développement
//En production, il faudra adapter pour utiliser wss:// (WebSocket sécurisé)

function getWebSocketUrl(): string {
  if (typeof window === 'undefined') return '';

  // Pour le moment, on utilise uniquement ws:// en développement
  const host = process.env.NEXT_PUBLIC_WS_URL || 'localhost:8080';

  return `ws://${host}/ws`;
}
