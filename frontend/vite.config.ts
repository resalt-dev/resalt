import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import liveReload from 'vite-plugin-live-reload'
import autoprefixer from 'autoprefixer';
import sveltePreprocess from 'svelte-preprocess';

const production = process.env.NODE_ENV === 'production';

// https://vitejs.dev/config/
export default defineConfig({
	optimizeDeps:{
		exclude:['svelte-routing']
	},
	build: {
		outDir: 'build',
	},
	plugins: [
		!production && liveReload('./src/**/*'),
		svelte({
			preprocess: sveltePreprocess({
				sourceMap: !production,
				scss: {
					includePaths: ['src/layout/include'],
					prependData: '@use \'src/styles/include/_include.scss\';',
				},
				postcss: {
					plugins: [
						autoprefixer(),
					],
				},
			}),
			compilerOptions: {
				// enable run-time checks when not in production
				dev: !production,
			},
			emitCss: true,
		}),
	],
})
