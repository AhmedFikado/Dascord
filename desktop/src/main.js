import { app, BrowserWindow, ipcMain, Notification, shell } from 'electron';
import path from 'node:path';
import { spawn } from 'node:child_process';
import waitOn from 'wait-on';
import started from 'electron-squirrel-startup';

if (started) app.quit();

const NEXT_PORT = 3000;
const NEXT_URL = `http://localhost:${NEXT_PORT}`;
const IS_DEV = process.env.NODE_ENV !== 'production';
const CLIENT_DIR = path.join(__dirname, '../../../client');

let nextProcess = null;
let mainWindow = null;

// Notifications i18n strings
const notifStrings = {
  fr: { newMessage: 'Nouveau message', from: 'de' },
  en: { newMessage: 'New message', from: 'from' },
};

function getLang() {
  const locale = app.getLocale() || 'fr';
  return locale.startsWith('fr') ? 'fr' : 'en';
}

function startNextServer() {
  return new Promise((resolve, reject) => {
    const cmd = process.platform === 'win32' ? 'npm.cmd' : 'npm';
    nextProcess = spawn(cmd, ['run', 'start'], {
      cwd: CLIENT_DIR,
      env: { ...process.env, PORT: String(NEXT_PORT) },
      stdio: IS_DEV ? 'inherit' : 'ignore',
    });

    nextProcess.on('error', reject);

    waitOn({ resources: [`tcp:${NEXT_PORT}`], timeout: 30000 })
      .then(resolve)
      .catch(reject);
  });
}

function createWindow() {
  mainWindow = new BrowserWindow({
    width: 1280,
    height: 800,
    minWidth: 900,
    minHeight: 600,
    title: 'Rust Chat',
    webPreferences: {
      preload: path.join(__dirname, 'preload.js'),
      contextIsolation: true,
      nodeIntegration: false,
    },
    show: false,
  });

  mainWindow.loadURL(NEXT_URL);

  mainWindow.once('ready-to-show', () => {
    mainWindow.show();
  });

  // Open external links in the system browser
  mainWindow.webContents.setWindowOpenHandler(({ url }) => {
    shell.openExternal(url);
    return { action: 'deny' };
  });

  if (IS_DEV) mainWindow.webContents.openDevTools({ mode: 'detach' });
}

// IPC: system notification from renderer
ipcMain.on('notify', (_event, { title, body }) => {
  if (Notification.isSupported()) {
    new Notification({ title, body }).show();
  }
});

// IPC: new message notification (with i18n fallback)
ipcMain.on('notify:message', (_event, { senderName, preview }) => {
  if (!Notification.isSupported()) return;
  const lang = getLang();
  const t = notifStrings[lang] || notifStrings.en;
  new Notification({
    title: `${t.newMessage} ${t.from} ${senderName}`,
    body: preview || '',
  }).show();
});

// IPC: get current locale
ipcMain.handle('get-locale', () => getLang());

app.whenReady().then(async () => {
  try {
    await startNextServer();
    createWindow();
  } catch (err) {
    console.error('Failed to start Next.js server:', err);
    app.quit();
  }

  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) createWindow();
  });
});

app.on('window-all-closed', () => {
  if (nextProcess) nextProcess.kill();
  if (process.platform !== 'darwin') app.quit();
});

app.on('will-quit', () => {
  if (nextProcess) nextProcess.kill();
});
