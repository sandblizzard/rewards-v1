import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte()],
  optimizeDeps: {
    include: ['@project-serum/anchor', '@solana/web3.js', 'buffer'],
    // ... use the same implementation from the SvelteKit ui
  },
  define: {
    // This makes @project-serum/anchor 's process error not happen since it replaces all instances of process.env.BROWSER with true
    'process.env.BROWSER': true,
    'process.env.NODE_DEBUG': JSON.stringify(''),
  },
});
