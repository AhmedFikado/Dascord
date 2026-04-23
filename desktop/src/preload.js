const { contextBridge, ipcRenderer } = require('electron');

contextBridge.exposeInMainWorld('electronAPI', {
  // Send a generic system notification
  notify: (title, body) => ipcRenderer.send('notify', { title, body }),

  // Send a new message notification (i18n handled in main)
  notifyMessage: (senderName, preview) =>
    ipcRenderer.send('notify:message', { senderName, preview }),

  // Get the current locale ('fr' or 'en')
  getLocale: () => ipcRenderer.invoke('get-locale'),

  // Whether we are running inside Electron
  isElectron: true,
});
