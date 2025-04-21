import tsParser from '@typescript-eslint/parser';
import eslintPluginTypescript from '@typescript-eslint/eslint-plugin';
import eslintPluginReact from 'eslint-plugin-react';

export default [
  {
    ignores: ['**/.next/**/*', '**/node_modules/**/*', 'eslint.config.js', '.eslintrc.js'],
  },
  {
    files: ['**/*.{js,jsx,ts,tsx}'],
    languageOptions: {
      parser: tsParser,
      parserOptions: {
        project: './tsconfig.json',
      },
    },
    plugins: {
      '@typescript-eslint': eslintPluginTypescript,
      react: eslintPluginReact,
    },
    rules: {
      // Add your rules here
      'react/react-in-jsx-scope': 'off',
    },
  },
];
