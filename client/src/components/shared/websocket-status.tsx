'use client';

import { useWebSocketStore } from '@/store/websocket';
import { WebSocketStatus } from '@/types/websocket';

export function WebSocketStatusIndicator() {
  const status = useWebSocketStore(state => state.status);
  const error = useWebSocketStore(state => state.error);

  const getStatusColor = () => {
    switch (status) {
      case WebSocketStatus.CONNECTED:
        return 'bg-green-500';
      case WebSocketStatus.CONNECTING:
      case WebSocketStatus.RECONNECTING:
        return 'bg-yellow-500';
      case WebSocketStatus.ERROR:
      case WebSocketStatus.DISCONNECTED:
        return 'bg-red-500';
      default:
        return 'bg-gray-500';
    }
  };

  const getStatusText = () => {
    switch (status) {
      case WebSocketStatus.CONNECTED:
        return 'Connecté';
      case WebSocketStatus.CONNECTING:
        return 'Connexion...';
      case WebSocketStatus.RECONNECTING:
        return 'Reconnexion...';
      case WebSocketStatus.ERROR:
        return 'Erreur';
      case WebSocketStatus.DISCONNECTED:
        return 'Déconnecté';
      default:
        return 'Inconnu';
    }
  };

  return (
    <div className="flex items-center gap-2 text-xs text-gray-400">
      <div className={`w-2 h-2 rounded-full ${getStatusColor()}`} />
      <span>{getStatusText()}</span>
      {error && <span className="text-red-500">({error})</span>}
    </div>
  );
}
