import prettier from "eslint-config-prettier"
import path from "node:path"
import js from "@eslint/js"
import svelte from "eslint-plugin-svelte"
import importPlugin from "eslint-plugin-import"
import { defineConfig, includeIgnoreFile } from "eslint/config"
import globals from "globals"
import ts from "typescript-eslint"

const gitignorePath = path.resolve(import.meta.dirname, "../../.gitignore")

export default defineConfig(
	includeIgnoreFile(gitignorePath),
	js.configs.recommended,
	ts.configs.recommended,
	svelte.configs.recommended,
	prettier,
	svelte.configs.prettier,
	{
		plugins: { import: importPlugin },
		languageOptions: { globals: { ...globals.browser, ...globals.node } },
		rules: {
			"no-undef": "off",
			"import/no-cycle": ["error", { ignoreExternal: true }],
			"import/no-extraneous-dependencies": "error",
			"svelte/no-navigation-without-resolve": "off",
			"@typescript-eslint/no-explicit-any": "error",
			"@typescript-eslint/no-unused-vars": [
				"error",
				{
					argsIgnorePattern: "^_",
					varsIgnorePattern: "^_",
				},
			],
			"@typescript-eslint/consistent-type-imports": "error",
			"@typescript-eslint/no-non-null-assertion": "off",
		},
	},
	{
		files: ["**/*.ts", "**/*.svelte", "**/*.svelte.ts", "**/*.svelte.js"],
		languageOptions: {
			parserOptions: {
				projectService: true,
				extraFileExtensions: [".svelte"],
				parser: ts.parser,
			},
		},
		rules: {
			"@typescript-eslint/no-floating-promises": "error",
			"@typescript-eslint/no-misused-promises": "error",
			"@typescript-eslint/restrict-template-expressions": "error",
			"@typescript-eslint/restrict-plus-operands": "error",
			"@typescript-eslint/switch-exhaustiveness-check": "error",
			"@typescript-eslint/prefer-readonly": "error",
			"@typescript-eslint/no-unnecessary-type-assertion": "error",
		},
	},
	{
		files: ["**/html-renderer.svelte", "**/manual-article.svelte"],
		rules: {
			"svelte/no-at-html-tags": "off",
		},
	},
	{
		rules: {},
	},
)
