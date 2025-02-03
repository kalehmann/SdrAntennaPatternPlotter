import js from "@eslint/js";
import globals from 'globals';
import svelte from 'eslint-plugin-svelte';
import ts from 'typescript-eslint';
import svelteConfig from './svelte.config.js';

export default [
    {
	ignores: [
	    '**/build/*',
	    '**/.svelte-kit/*',
	],
    },
    js.configs.recommended,
    ...ts.configs.recommended,
    ...svelte.configs.recommended,
    {
	languageOptions: {
	    globals: {
		...globals.browser,
	    }
	}
    },
    {
	files: ['**/*.svelte'],
	languageOptions: {
	    parserOptions: {
		projectService: true,
		extraFileExtensions: ['.svelte'],
		parser: ts.parser,
		svelteConfig,
	    }
	},
    },
    {
	files: [
	    '*.svelte',
	    '**/*.svelte',
	    '*.svelte.ts',
	    '**/*.svelte.ts',
	    '*.svelte.js',
	    '**/*.svelte.js'
	],
	languageOptions: {
	    parserOptions: {
		svelteConfig
	    }
	}
    },
];
