// eslint.config.js
import stylistic from '@stylistic/eslint-plugin'

export const baseConfig = {
  env: {
    browser: true,
    es2022: true
  },
  parserOptions: {
    ecmaVersion: 'latest',
    sourceType: 'module',
    ecmaFeatures: {
      jsx: true,
    },
  },
  settings: {
    react: {
      version: 'detect',
    },
  },
  extends: [
    "plugin:react/recommended",
    "plugin:react-hooks/recommended"
  ],
  rules: {}
}

export default [
  stylistic.configs.customize({
    indent: 2,
    quotes: 'single',
    semi: false,
    jsx: true,
  }),
  baseConfig
]
