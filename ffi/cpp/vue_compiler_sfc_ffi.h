/**
 * @file vue_compiler_sfc_ffi.h
 * @brief C FFI interface for the Vue SFC compiler via Static Hermes.
 *
 * This header defines the public C API for interacting with the Vue SFC compiler
 * from Rust or other languages via FFI. The implementation uses a handle-based
 * system where JavaScript objects are exposed as opaque 64-bit handles.
 *
 * ## Handle System
 *
 * - Handle 0 is reserved as the invalid/null handle
 * - Valid handles are returned by parsing and compilation functions
 * - Handles must be explicitly freed with vue_handle_free() to avoid memory leaks
 * - String pointers returned by accessor functions are owned by the handle
 *   and become invalid after the handle is freed
 *
 * ## Thread Safety
 *
 * This API is NOT thread-safe. All operations must be performed from a single thread.
 *
 * ## Typical Usage
 *
 * 1. Parse SFC source with vue_parse()
 * 2. Access descriptor via vue_parse_result_descriptor()
 * 3. Compile script with vue_compile_script() if present
 * 4. Compile template with vue_compile_template()
 * 5. Compile styles with vue_compile_style()
 * 6. Free all handles with vue_handle_free()
 */

#ifndef VUE_COMPILER_SFC_FFI_H
#define VUE_COMPILER_SFC_FFI_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Opaque handle type for JavaScript objects.
 * Handle 0 represents null/invalid.
 */
typedef uint64_t RawHandle;

// ============================================================================
// Handle Management
// ============================================================================

/**
 * Frees a handle and releases its resources.
 *
 * After this call, the handle becomes invalid. Any strings returned by
 * accessor functions for this handle also become invalid.
 *
 * This function is safe to call with handle 0 (no-op).
 *
 * @param handle The handle to free.
 */
void vue_handle_free(RawHandle handle);

// ============================================================================
// Parsing
// ============================================================================

/**
 * Parses a Vue SFC source string.
 *
 * @param source Null-terminated UTF-8 string containing the SFC source.
 * @param filename Null-terminated UTF-8 string with the filename (for errors).
 * @return Handle to the parse result object.
 */
RawHandle vue_parse(const char* source, const char* filename);

/**
 * Gets the descriptor handle from a parse result.
 *
 * @param handle Handle to a parse result from vue_parse().
 * @return Handle to the descriptor, or 0 if not available.
 */
RawHandle vue_parse_result_descriptor(RawHandle handle);

/**
 * Gets the number of parse errors.
 *
 * @param handle Handle to a parse result from vue_parse().
 * @return Number of errors, or 0 if no errors or invalid handle.
 */
size_t vue_parse_result_error_count(RawHandle handle);

/**
 * Gets an error message at the specified index.
 *
 * @param handle Handle to a parse result from vue_parse().
 * @param index Zero-based index of the error.
 * @return Pointer to the error message string. The string is owned by the handle
 *         and becomes invalid when the handle is freed. Returns "" on error.
 */
const char* vue_parse_result_error_message(RawHandle handle, size_t index);

// ============================================================================
// Descriptor Accessors
// ============================================================================

/**
 * Checks if the descriptor has a template block.
 *
 * @param handle Handle to a descriptor from vue_parse_result_descriptor().
 * @return true if the descriptor has a template block, false otherwise.
 */
bool vue_descriptor_has_template(RawHandle handle);

/**
 * Checks if the descriptor has a script block.
 *
 * @param handle Handle to a descriptor from vue_parse_result_descriptor().
 * @return true if the descriptor has a script block, false otherwise.
 */
bool vue_descriptor_has_script(RawHandle handle);

/**
 * Checks if the descriptor has a script setup block.
 *
 * @param handle Handle to a descriptor from vue_parse_result_descriptor().
 * @return true if the descriptor has a script setup block, false otherwise.
 */
bool vue_descriptor_has_script_setup(RawHandle handle);

/**
 * Gets the number of style blocks.
 *
 * @param handle Handle to a descriptor from vue_parse_result_descriptor().
 * @return Number of style blocks, or 0 if invalid handle.
 */
size_t vue_descriptor_style_count(RawHandle handle);

/**
 * Gets the template block handle.
 *
 * @param handle Handle to a descriptor from vue_parse_result_descriptor().
 * @return Handle to the template block, or 0 if not present.
 */
RawHandle vue_descriptor_template(RawHandle handle);

/**
 * Gets the script setup block handle.
 *
 * @param handle Handle to a descriptor from vue_parse_result_descriptor().
 * @return Handle to the script setup block, or 0 if not present.
 */
RawHandle vue_descriptor_script_setup(RawHandle handle);

/**
 * Gets a style block handle at the specified index.
 *
 * @param handle Handle to a descriptor from vue_parse_result_descriptor().
 * @param index Zero-based index of the style block.
 * @return Handle to the style block, or 0 if index out of bounds.
 */
RawHandle vue_descriptor_style_at(RawHandle handle, size_t index);

// ============================================================================
// Block Accessors
// ============================================================================

/**
 * Gets the content of a block.
 *
 * Works for template, script, and style blocks.
 *
 * @param handle Handle to a block (template, script, or style).
 * @return Pointer to the content string. The string is owned by the handle
 *         and becomes invalid when the handle is freed. Returns "" on error.
 */
const char* vue_block_content(RawHandle handle);

/**
 * Gets the lang attribute of a block.
 *
 * @param handle Handle to a block (template, script, or style).
 * @return Pointer to the lang string. The string is owned by the handle
 *         and becomes invalid when the handle is freed. Returns "" if not set.
 */
const char* vue_block_lang(RawHandle handle);

/**
 * Checks if a style block is scoped.
 *
 * @param handle Handle to a style block.
 * @return true if the style block is scoped, false otherwise.
 */
bool vue_style_is_scoped(RawHandle handle);

// ============================================================================
// Script Compilation
// ============================================================================

/**
 * Compiles the script blocks of an SFC.
 *
 * @param desc_handle Handle to the SFC descriptor.
 * @param id Null-terminated scope ID string (e.g., "data-v-abc123").
 * @param is_prod Whether to compile for production.
 * @return Handle to the compilation result.
 */
RawHandle vue_compile_script(RawHandle desc_handle, const char* id, bool is_prod);

/**
 * Gets the compiled script content.
 *
 * @param handle Handle to a script compilation result.
 * @return Pointer to the compiled content string. The string is owned by the handle
 *         and becomes invalid when the handle is freed. Returns "" on error.
 */
const char* vue_script_result_content(RawHandle handle);

/**
 * Gets the bindings handle from a script result.
 *
 * The bindings can be passed to vue_compile_template() for optimization.
 *
 * @param handle Handle to a script compilation result.
 * @return Handle to the bindings object, or 0 if not available.
 */
RawHandle vue_script_result_bindings(RawHandle handle);

// ============================================================================
// Template Compilation
// ============================================================================

/**
 * Compiles a Vue template to a render function.
 *
 * @param source Null-terminated template source string.
 * @param filename Null-terminated filename string.
 * @param id Null-terminated scope ID string (e.g., "data-v-abc123").
 * @param scoped Whether to add scoped attribute selectors.
 * @param bindings_handle Handle to bindings object from vue_script_result_bindings(),
 *                        or 0 for none.
 * @return Handle to the compilation result.
 */
RawHandle vue_compile_template(
    const char* source,
    const char* filename,
    const char* id,
    bool scoped,
    RawHandle bindings_handle
);

/**
 * Gets the compiled template code (render function).
 *
 * @param handle Handle to a template compilation result.
 * @return Pointer to the render function code string. The string is owned by the handle
 *         and becomes invalid when the handle is freed. Returns "" on error.
 */
const char* vue_template_result_code(RawHandle handle);

/**
 * Gets the number of template compilation errors.
 *
 * @param handle Handle to a template compilation result.
 * @return Number of errors, or 0 if no errors or invalid handle.
 */
size_t vue_template_result_error_count(RawHandle handle);

// ============================================================================
// Style Compilation
// ============================================================================

/**
 * Compiles a CSS style block.
 *
 * @param source Null-terminated CSS source string.
 * @param filename Null-terminated filename string.
 * @param id Null-terminated scope ID string (e.g., "data-v-abc123").
 * @param scoped Whether to add scoped attribute selectors.
 * @return Handle to the compilation result.
 */
RawHandle vue_compile_style(
    const char* source,
    const char* filename,
    const char* id,
    bool scoped
);

/**
 * Gets the compiled CSS code.
 *
 * @param handle Handle to a style compilation result.
 * @return Pointer to the compiled CSS string. The string is owned by the handle
 *         and becomes invalid when the handle is freed. Returns "" on error.
 */
const char* vue_style_result_code(RawHandle handle);

#ifdef __cplusplus
}
#endif

#endif /* VUE_COMPILER_SFC_FFI_H */
