// @ts-check

import js from '@eslint/js';
import globals from 'globals';
import tseslint from 'typescript-eslint';

export default tseslint.config({
    extends: [
        js.configs.recommended,
        ...tseslint.configs.recommended,
        ...tseslint.configs.strict,
        ...tseslint.configs.stylistic,
    ],
    files: ['src/**/*.{ts,tsx}'],
    ignores: ['lib/**/*'],
    languageOptions: {
        ecmaVersion: 2020,
        globals: globals.browser,
    },
    plugins: {},
    rules: {
        '@typescript-eslint/no-unused-vars': [
            'error',
            {
                args: 'all',
                argsIgnorePattern: '^_',
                caughtErrors: 'all',
                caughtErrorsIgnorePattern: '^_',
                destructuredArrayIgnorePattern: '^_',
                varsIgnorePattern: '^_',
                ignoreRestSiblings: true,
            },
        ],
        '@typescript-eslint/no-explicit-any': 'off',
    },
});
