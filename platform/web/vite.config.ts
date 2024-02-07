import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
  plugins: [sveltekit()],
  optimizeDeps: {
    exclude: ['filigree-web'],
  },
  server: {
    proxy: {
      '/api': {
        target: 'http://localhost:6749',
        changeOrigin: true,
      },
    },
  },
});
