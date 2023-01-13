import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import autoprefixer from 'autoprefixer';
import sveltePreprocess from 'svelte-preprocess';

const tagsRegex1 = /(>)[\s]*([<{])/g;
const tagsRegex2 = /({[/:][a-z]+})[\s]*([<{])/g;
const tagsRegex3 = /({[#:][a-z]+ .+?})[\s]*([<{])/g;
const tagsRegex4 = /([>}])[\s]+(<|{[/#:][a-z][^}]*})/g;
const tagsReplace = '$1$2';

const production = process.env.NODE_ENV === 'production';

// https://vitejs.dev/config/
export default defineConfig({
	optimizeDeps: {
		exclude: ['svelte-routing', 'svelte-navigator'],
	},
	build: {
		outDir: 'build',
	},
	server: {
		hmr: production ? false : {
			port: 5555,
		},
	},
	plugins: [
		svelte({
			preprocess: sveltePreprocess({
				sourceMap: !production,
				scss: {
					includePaths: ['src/layout/include'],
					prependData: "@use 'src/styles/include/_include.scss';",
				},
				postcss: {
					plugins: [autoprefixer()],
				},
				// Remove redundant whitespaces that affects the layout
				// https://github.com/sveltejs/svelte/issues/189#issuecomment-1126375112
				replace: [
					[tagsRegex1, tagsReplace],
					[tagsRegex2, tagsReplace],
					[tagsRegex3, tagsReplace],
					[tagsRegex4, tagsReplace]
				],
			}),
			compilerOptions: {
				// enable run-time checks when not in production
				dev: !production,
			},
			emitCss: true,
		}),
	],
});
