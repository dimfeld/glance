import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
  plugins: [sveltekit()],
  optimizeDeps: {
    exclude: ['filigree-svelte'],
  },
  server: {
    proxy: {
      '/api': {
        target: 'http://localhost:6749',
      },
    },
  },
});
