module.exports = {
  parser: '@typescript-eslint/parser', // Specifies the ESLint parser for TypeScript
  parserOptions: {
    ecmaVersion: 'latest',
    sourceType: 'module',
    project: './tsconfig.json', // Important for type-aware linting rules
  },
  env: {
    browser: true,
    node: true,
    es2021: true,
    jest: true,
  },
  extends: [
    'eslint:recommended',
    'plugin:@typescript-eslint/recommended', // Uses the recommended rules from the @typescript-eslint/eslint-plugin
    'airbnb-base',
    'airbnb-typescript/base', // Uses typescript-specific rules from Airbnb
    'plugin:prettier/recommended', // MUST BE LAST
  ],
  plugins: ['@typescript-eslint', 'import'],
  rules: {
    'prettier/prettier': 'error',
    // Your custom rules here
    'no-console': process.env.NODE_ENV === 'production' ? 'error' : 'warn',
    'import/prefer-default-export': 'off',
    '@typescript-eslint/no-unused-vars': 'warn',
  },
  settings: {
    'import/resolver': {
      typescript: {}, // this loads <rootdir>/tsconfig.json to find modules
    },
  },
};
