module.exports = {
	root: true,
	parser: '@typescript-eslint/parser',
	extends: ['eslint:recommended', 'plugin:@typescript-eslint/recommended', 'prettier', 'airbnb-base'],
	plugins: ['svelte3', '@typescript-eslint'],
	ignorePatterns: ['*.cjs'],
	overrides: [{ files: ['*.svelte'], processor: 'svelte3/svelte3' }],
	settings: {
		'svelte3/typescript': () => require('typescript')
	},
	parserOptions: {
		sourceType: 'module',
		ecmaVersion: 2020
	},
	env: {
		browser: true,
		es20121: true,
		node: true
	},
	rules: {
		'camelcase': 'error',
        'indent': ['error', 4],
        'no-console': 'off',
        'import/no-extraneous-dependencies': 'off',
        'import/no-unresolved': 'off',
        'import/extensions': 'off',
        'no-param-reassign': 'off',
        'implicit-arrow-linebreak': 'off',
        'function-paren-newline': 'off',
        'operator-linebreak': ['error', 'after'],
        'no-restricted-syntax': ['error', 'LabeledStatement', 'WithStatement'],
        'jsx-a11y/click-events-have-key-events': 'off',
        'lit-a11y/click-events-have-key-events': 'off',
	}
};
