const { contextBridge, ipcRenderer } = require('electron');

contextBridge.exposeInMainWorld('electronAPI', {
  // Send a generic system notification
  notify: (title, body) => ipcRenderer.send('notify', { title, body }),

  // Send a new message notification (i18n handled in main)
  notifyMessage: (senderName, preview) =>
    ipcRenderer.send('notify:message', { senderName, preview }),

  // Get the current locale ('fr' or 'en')
  getLocale: () => ipcRenderer.invoke('get-locale'),

  // Check if the Electron window currently has OS-level focus
  isWindowFocused: () => ipcRenderer.invoke('is-window-focused'),

  // Subscribe to window focus/blur events (callback receives boolean)
  onWindowFocusChanged: (callback) => {
    const handler = (_event, focused) => callback(focused);
    ipcRenderer.on('window-focus-changed', handler);
    return () => ipcRenderer.removeListener('window-focus-changed', handler);
  },

  // Whether we are running inside Electron
  isElectron: true,
});
