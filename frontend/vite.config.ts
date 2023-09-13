import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	server: {
		hmr: {
			port: 5555,
		},
	},
	optimizeDeps: {
		exclude: ['codemirror', '@codemirror/view', '@codemirror/language'],
	},
});
