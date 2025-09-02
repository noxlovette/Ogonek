import js from "@eslint/js";
import prettier from "eslint-config-prettier";
import svelte from "eslint-plugin-svelte";
import globals from "globals";
import ts from "typescript-eslint";

export default [
  js.configs.recommended,
  ...ts.configs.recommended,
  ...svelte.configs["flat/recommended"],
  ...svelte.configs.prettier,
  prettier,
  ...svelte.configs["flat/prettier"],
  {
    languageOptions: {
      globals: { ...globals.browser, ...globals.node },
    },
  },
  {
    files: ["**/*.svelte", "**/*.ts"],
    languageOptions: { parserOptions: { parser: ts.parser } },
    rules: {
      "svelte/no-at-html-tags": "off",
      "@typescript-eslint/no-explicit-any": "off",
    },
  },
  {
    ignores: ["build/", ".svelte-kit/", "dist/", "src/lib/paraglide"],
  },
];
