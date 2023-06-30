module.exports = {
	root: true,
	parser: '@typescript-eslint/parser',
	extends: [
		'plugin:svelte/recommended',
		'eslint:recommended',
		'plugin:@typescript-eslint/recommended',
		'prettier',
	],
	plugins: ['@typescript-eslint', 'import'],
	ignorePatterns: ['*.cjs'],
	overrides: [
		{
			files: ['*.svelte'],
			parser: 'svelte-eslint-parser',
			// Parse the `<script>` in `.svelte` as TypeScript by adding the following configuration.
			parserOptions: {
				parser: '@typescript-eslint/parser',
			},
		},
	],
	parserOptions: {
		sourceType: 'module',
		ecmaVersion: 2020,
		project: './tsconfig.json',
		extraFileExtensions: ['.svelte'], // This is a required setting in `@typescript-eslint/parser` v4.24.0.
	},
	env: {
		browser: true,
		es2021: true,
		node: true,
	},
	rules: {
		camelcase: 'error',
		// Indent switches and nested ? : with tabs
		indent: [
			'error',
			'tab',
			{
				SwitchCase: 1,
				ignoredNodes: [
					'ConditionalExpression',
					'ObjectExpression',
					'MemberExpression',
					'CallExpression',
				],
			},
		],
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

		// Allow functions inside .svelte files
		'no-inner-declarations': 'off',
	},
};
