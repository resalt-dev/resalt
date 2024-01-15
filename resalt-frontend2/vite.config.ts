import react from '@vitejs/plugin-react';
import { UserConfig, defineConfig } from 'vite';

// https://vitejs.dev/config/
export default defineConfig({
	plugins: [react()],
	server: {
		hmr: {
			port: 5555,
		},
	},
} as UserConfig);
