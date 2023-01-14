module.exports = {
	parser: '@typescript-eslint/parser',
	parserOptions: {
		project: 'tsconfig.json',
		tsconfigRootDir : __dirname, 
		sourceType: 'module',
	},
	plugins: ['@typescript-eslint/eslint-plugin'],
	extends: [
		'plugin:@typescript-eslint/recommended',
	],
	root: true,
	env: {
		node: true,
	},
	ignorePatterns: ['.eslintrc.js', 'node_modules', 'dist'],
	rules: {
		'@typescript-eslint/explicit-function-return-type': ['error'],
		'@typescript-eslint/explicit-module-boundary-types': ['error'],
		'@typescript-eslint/no-explicit-any': ['error'],
		'eol-last': ['error'],
		'semi': ['error'],
		'object-curly-spacing': ['error', 'always'],
		'comma-dangle': ['error', 'always-multiline'],
		'@typescript-eslint/explicit-member-accessibility': ['error'],
		'no-multiple-empty-lines': ['error', { 'max': 1 }],
		'no-trailing-spaces': ['error'],
	},
};
