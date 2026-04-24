interface ElectronAPI {
  notify: (title: string, body: string) => void;
  notifyMessage: (senderName: string, preview: string) => void;
  getLocale: () => Promise<"fr" | "en">;
  isWindowFocused: () => Promise<boolean>;
  onWindowFocusChanged: (callback: (focused: boolean) => void) => () => void;
  isElectron: boolean;
}

declare global {
  interface Window {
    electronAPI?: ElectronAPI;
  }
}

export {};
