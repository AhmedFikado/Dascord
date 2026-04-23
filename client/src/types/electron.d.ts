interface ElectronAPI {
  notify: (title: string, body: string) => void;
  notifyMessage: (senderName: string, preview: string) => void;
  getLocale: () => Promise<"fr" | "en">;
  isElectron: boolean;
}

declare global {
  interface Window {
    electronAPI?: ElectronAPI;
  }
}

export {};
