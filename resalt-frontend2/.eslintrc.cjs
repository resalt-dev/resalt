module.exports = {
	root: true,
	env: { browser: true, es2020: true },
	extends: [
		'eslint:recommended',
		// 'plugin:@typescript-eslint/recommended',
		'plugin:@typescript-eslint/strict-type-checked',
		'plugin:react-hooks/recommended',
	],
	ignorePatterns: ['dist', '.eslintrc.cjs', 'package.json', 'vite.config.ts'],
	parser: '@typescript-eslint/parser',
	parserOptions: {
		project: 'tsconfig.json',
		tsconfigRootDir: __dirname,
	},
	plugins: ['react-refresh'],
	rules: {
		// 'typescript-eslint/no-unnecessary-condition': 'error',
	},
};
