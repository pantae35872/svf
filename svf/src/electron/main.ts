import { app, BrowserWindow } from 'electron';
import path from 'path';
import { isDev } from './util.js';
import { createServer } from 'http-server';


app.on("ready", () => {
  const mainWindow = new BrowserWindow({});
  if (process.platform == 'win32' || process.platform == 'linux') mainWindow.removeMenu();
  if (isDev()) {
    mainWindow.loadURL('http://localhost:5123');
  } else { 
    const serverPath = path.join(app.getAppPath(), 'dist-vue');
    let server = createServer({ root: serverPath });
    server.listen(3000, () => {
      mainWindow.loadURL("http://localhost:3000");
    })

    app.on('will-quit', () => {
      server.close();
    });
  }
});
