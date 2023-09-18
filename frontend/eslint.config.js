// eslint.config.js

import globals from 'globals';

// Imports
import typescript from '@typescript-eslint';
import sveltePlugin from 'eslint-plugin-svelte';
import prettier from 'eslint-config-prettier';

// Plugins
import typescriptPlugin from '@typescript-eslint/eslint-plugin';

// Parsers
import svelteParser from 'svelte-eslint-parser';
import typescriptParser from '@typescript-eslint/parser';

export default [
	'eslint:recommended',
	typescriptPlugin.configs.recommended,
	sveltePlugin.configs.recommended,
	prettier,
	{
		files: ['src/**/*.ts'],
		languageOptions: {
			globals: {
				...globals.browser,
				...globals.es2021,
				...globals.node,
			},
			parser: typescriptParser,
			parserOptions: {
				sourceType: 'module',
				ecmaVersion: 2022,
				extraFileExtensions: ['.svelte'],
			},
		},
		plugins: {
			typescript: typescriptPlugin,
		},
	},
	{
		files: ['src/**/*.svelte'],
		languageOptions: {
			parser: svelteParser,
			parserOptions: {
				parser: typescriptParser,
			},
		},
	},
];
