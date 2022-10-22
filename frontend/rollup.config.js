import { terser } from 'rollup-plugin-terser';
import autoprefixer from 'autoprefixer';
import commonjs from '@rollup/plugin-commonjs';
import scss from 'rollup-plugin-scss';
import livereload from 'rollup-plugin-livereload';
import polyfills from 'rollup-plugin-node-polyfills';
import resolve from '@rollup/plugin-node-resolve';
import svelte from 'rollup-plugin-svelte';
import sveltePreprocess from 'svelte-preprocess';
import typescript from '@rollup/plugin-typescript';

const production = !process.env.ROLLUP_WATCH;

function serve() {
    let server;

    function toExit() {
        if (server) server.kill(0);
    }

    return {
        writeBundle() {
            if (server) return;
            try {
                // eslint-disable-next-line global-require
                server = require('child_process').spawn(
                    'npm',
                    ['run', 'start', '--', '--dev'],
                    {
                        stdio: ['ignore', 'inherit', 'inherit'],
                        shell: true,
                    },
                );
            } catch (e) {
                console.log(e);
            }
            console.log(server);

            process.on('SIGTERM', toExit);
            process.on('exit', toExit);
        },
    };
}

export default {
    input: 'src/main.ts',
    output: {
        sourcemap: !production,
        format: 'iife',
        name: 'app',
        file: 'public/build/bundle.js',
        inlineDynamicImports: true,
    },
    plugins: [
        polyfills(),

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
        // we'll extract any component CSS out into
        // a separate file - better for performance
        scss({
            output: 'public/build/bundle.css',
        }),

        // If you have external dependencies installed from
        // npm, you'll most likely need these plugins. In
        // some cases you'll need additional configuration -
        // consult the documentation for details:
        // https://github.com/rollup/plugins/tree/master/packages/commonjs
        resolve({
            browser: true,
            dedupe: ['svelte'],
        }),
        commonjs(),
        typescript({
            sourceMap: !production,
            inlineSources: !production,
        }),

        // In dev mode, call `npm run start` once
        // the bundle has been generated
        !production && serve(),

        // Watch the `public` directory and refresh the
        // browser on changes when not in production
        //! production && livereload('public'),
        !production &&
            livereload({
                watch: 'public',
                delay: 500,
                port: 35730,
            }),

        // If we're building for production (npm run build
        // instead of npm run dev), minify
        production && terser(),
    ],
    watch: {
        clearScreen: false,
    },
};
