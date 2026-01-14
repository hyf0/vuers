/**
 * @file vue-compiler.js
 * @description JavaScript entry point for the Vue SFC compiler FFI.
 *
 * This file is compiled to native code via Static Hermes and provides the bridge
 * between the C++ wrapper and the @vue/compiler-sfc package.
 *
 * All functions are exposed on globalThis and return raw JavaScript objects
 * (not JSON strings) for efficient access from the C++ layer via JSI.
 *
 * Error Handling:
 * - All functions catch exceptions and return error objects instead of throwing
 * - Parse errors are returned in the `errors` array of the result
 * - This ensures the FFI layer never encounters uncaught exceptions
 */

import {
    parse as sfcParse,
    compileScript as sfcCompileScript,
    compileTemplate as sfcCompileTemplate,
    compileStyle as sfcCompileStyle,
} from '@vue/compiler-sfc';

// ============================================================================
// Parse
// ============================================================================

/**
 * Parses a Vue Single File Component source string.
 *
 * @param {string} source - The SFC source code.
 * @param {string} filename - The filename (used for error messages and source maps).
 * @returns {Object} Parse result with `descriptor` and `errors` properties.
 *
 * @example
 * const result = parse('<template><div>Hello</div></template>', 'App.vue');
 * if (result.errors.length === 0) {
 *   console.log(result.descriptor.template.content);
 * }
 */
globalThis.parse = function(source, filename) {
    try {
        return sfcParse(source, { filename, sourceMap: true });
    } catch (e) {
        return {
            descriptor: null,
            errors: [{ message: e.message }],
        };
    }
};

// ============================================================================
// Script Compilation
// ============================================================================

/**
 * Compiles the script blocks of an SFC descriptor.
 *
 * Processes both `<script>` and `<script setup>` blocks, combining them into
 * a single output with binding metadata for template optimization.
 *
 * @param {Object} descriptor - The SFC descriptor from parseRaw().
 * @param {string} id - Scope ID for the component (e.g., "data-v-abc123").
 * @param {boolean} isProd - Whether to compile for production (enables optimizations).
 * @returns {Object} Compilation result with `content`, `bindings`, `map`, and `warnings`.
 *
 * @example
 * const scriptResult = compileScript(descriptor, 'data-v-abc123', false);
 * console.log(scriptResult.content); // Compiled JavaScript
 * console.log(scriptResult.bindings); // { msg: 'setup-ref', count: 'setup-ref' }
 */
globalThis.compileScript = function(descriptor, id, isProd) {
    try {
        const result = sfcCompileScript(descriptor, {
            id,
            isProd,
            sourceMap: true,
        });
        return {
            content: result.content,
            bindings: result.bindings || null,
            map: result.map || null,
            warnings: result.warnings || [],
        };
    } catch (e) {
        return {
            content: '',
            bindings: null,
            errors: [{ message: e.message }],
        };
    }
};

// ============================================================================
// Template Compilation
// ============================================================================

/**
 * Compiles a Vue template to a render function.
 *
 * @param {string} source - The template source code.
 * @param {string} filename - The filename (used for error messages).
 * @param {string} id - Scope ID for scoped styles (e.g., "data-v-abc123").
 * @param {boolean} scoped - Whether the component has scoped styles.
 * @param {Object|null} bindings - Binding metadata from compileScript() for optimization.
 * @returns {Object} Compilation result with `code`, `ast`, `preamble`, `map`, `errors`, and `tips`.
 *
 * @example
 * const templateResult = compileTemplate(
 *   '<div>{{ msg }}</div>',
 *   'App.vue',
 *   'data-v-abc123',
 *   true,
 *   { msg: 'setup-ref' }
 * );
 * console.log(templateResult.code); // render function code
 */
globalThis.compileTemplate = function(source, filename, id, scoped, bindings) {
    try {
        const result = sfcCompileTemplate({
            source,
            filename,
            id,
            scoped,
            slotted: false,
            isProd: false,
            ssr: false,
            compilerOptions: bindings ? { bindingMetadata: bindings } : {},
        });
        return {
            code: result.code,
            ast: result.ast || null,
            preamble: result.preamble || null,
            map: result.map || null,
            errors: result.errors || [],
            tips: result.tips || [],
        };
    } catch (e) {
        return {
            code: '',
            errors: [{ message: e.message }],
            tips: [],
        };
    }
};

// ============================================================================
// Style Compilation
// ============================================================================

/**
 * Compiles a CSS style block, optionally adding scoped attribute selectors.
 *
 * @param {string} source - The CSS source code.
 * @param {string} filename - The filename (used for error messages).
 * @param {string} id - Scope ID for scoped styles (e.g., "data-v-abc123").
 * @param {boolean} scoped - Whether to add scoped attribute selectors.
 * @returns {Object} Compilation result with `code`, `errors`, and `dependencies`.
 *
 * @example
 * const styleResult = compileStyle(
 *   '.container { color: red; }',
 *   'App.vue',
 *   'data-v-abc123',
 *   true
 * );
 * // Output: .container[data-v-abc123] { color: red; }
 * console.log(styleResult.code);
 */
globalThis.compileStyle = function(source, filename, id, scoped) {
    try {
        const result = sfcCompileStyle({
            source,
            filename,
            id,
            scoped,
            isProd: false,
        });
        return {
            code: result.code,
            errors: result.errors || [],
            dependencies: result.dependencies || [],
        };
    } catch (e) {
        return {
            code: '',
            errors: [{ message: e.message }],
            dependencies: [],
        };
    }
};
