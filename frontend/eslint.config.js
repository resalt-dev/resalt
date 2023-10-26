// eslint.config.js

import globals from 'globals';

// Imports
import js from '@eslint/js';
import prettier from 'eslint-config-prettier';

// Plugins
import sveltePlugin from 'eslint-plugin-svelte';
import typescriptPlugin from '@typescript-eslint/eslint-plugin';

// Parsers
import svelteParser from 'svelte-eslint-parser';
import typescriptParser from '@typescript-eslint/parser';

export default [
	{
		ignores: ['.svelte-kit/**/*', 'build/**/*', 'node_modules/**/*'],
	},

	// JavaScript
	{
		files: ['src/**/*.js'],
		rules: {
			...js.configs.recommended.rules,
			...prettier.rules,
		},
	},

	// TypeScript
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
				project: './tsconfig.json',
				extraFileExtensions: ['.svelte'],
			},
		},
		plugins: {
			svelte: sveltePlugin,
			'@typescript-eslint': typescriptPlugin,
		},
		rules: {
			...js.configs.recommended.rules,
			...typescriptPlugin.configs.recommended.rules,
			...sveltePlugin.configs.recommended.rules,
			...prettier.rules,
		},
	},

	// Svelte
	{
		files: ['src/**/*.svelte'],
		languageOptions: {
			globals: {
				...globals.browser,
				...globals.es2021,
				...globals.node,
			},
			parser: svelteParser,
			parserOptions: {
				parser: typescriptParser,
				project: './tsconfig.json',
				extraFileExtensions: ['.svelte'],
			},
		},
		plugins: {
			svelte: sveltePlugin,
			'@typescript-eslint': typescriptPlugin,
		},
		rules: {
			...typescriptPlugin.configs.recommended.rules,
			...sveltePlugin.configs.recommended.rules,
			...prettier.rules,
		},
	},
];
