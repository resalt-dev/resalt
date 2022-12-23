import { vitePreprocess } from '@sveltejs/vite-plugin-svelte'
import adapter from '@sveltejs/adapter-auto';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  onwarn: (warning, handler) => {
    if (warning.code.startsWith('a11y-')) {
      return;
    }
    handler(warning);
  },
  // Consult https://svelte.dev/docs#compile-time-svelte-preprocess
  // for more information about preprocessors
  preprocess: vitePreprocess(),
  kit: {
    adapter: adapter()
  }
};

export default config;