import { app, BrowserWindow } from 'electron';
import path from 'path';
import { isDev } from './util.js';

app.on("ready", () => {
  const mainWindow = new BrowserWindow({});
  if (process.platform == 'win32' || process.platform == 'linux') mainWindow.removeMenu();
  if (isDev()) {
    mainWindow.loadURL('http://localhost:5123');
  } else { 
    mainWindow.loadFile(path.join(app.getAppPath(), '/dist-vue/index.html'));
  }
});
