module.exports = {
	root: true,
	parser: '@typescript-eslint/parser',
	// extends: ['eslint:recommended', 'plugin:@typescript-eslint/recommended', 'prettier', 'airbnb-base'],
	extends: ['eslint:recommended', 'plugin:@typescript-eslint/recommended', 'prettier'],
	plugins: ['svelte3', '@typescript-eslint', 'import'],
	ignorePatterns: ['*.cjs'],
	overrides: [{ files: ['*.svelte'], processor: 'svelte3/svelte3' }],
	settings: {
		'svelte3/typescript': () => require('typescript'),
	},
	parserOptions: {
		sourceType: 'module',
		ecmaVersion: 2020,
	},
	env: {
		browser: true,
		es2021: true,
		node: true,
	},
	rules: {
		camelcase: 'error',
		// Indent switches and nested ? : with tabs
		indent: ['error', 'tab', { SwitchCase: 1 }],
		'no-console': 'off',
		'import/no-extraneous-dependencies': 'error',
		'import/no-unresolved': 'off',
		'import/extensions': ['error', 'never'],
		'no-param-reassign': 'error',
		'implicit-arrow-linebreak': 'off',
		'function-paren-newline': 'off',
		'operator-linebreak': ['error', 'after'],
		'no-restricted-syntax': ['error', 'LabeledStatement', 'WithStatement'],

		// Always enforce types
		'@typescript-eslint/no-inferrable-types': 'off',
		'@typescript-eslint/typedef': 'error',

		// Allow any for certain reasons, like Salt permission...
		'@typescript-eslint/no-explicit-any': 'off', // TODO: Can this be fixed with types?

		// Disable operator-linebreak because Pretter doesn't format it
		'operator-linebreak': 'off',

		// Do not prefer ES2015 module syntax when we use 2021 syntax
		'@typescript-eslint/no-namespace': 'off',
	},
};
