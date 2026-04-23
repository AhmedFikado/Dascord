import { defineConfig } from 'vite';

// https://vitejs.dev/config
export default defineConfig({
  build: {
    rollupOptions: {
      // Keep native Node.js modules external (not bundled)
      external: ['wait-on', 'electron-squirrel-startup'],
    },
  },
});
