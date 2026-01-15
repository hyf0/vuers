/**
 * @file vue_parse.h
 * @brief Parsing API for the Vue SFC compiler FFI.
 *
 * This header provides functions for parsing Vue Single File Components (SFC)
 * and accessing the resulting descriptor and block information.
 *
 * ## Typical Usage
 *
 * 1. Parse SFC source with vue_parse()
 * 2. Access descriptor via vue_parse_result_descriptor()
 * 3. Check for blocks using vue_descriptor_has_* functions
 * 4. Access block content via vue_block_* functions
 * 5. Free all handles with vue_handle_free()
 */

#ifndef VUE_PARSE_H
#define VUE_PARSE_H

#include "vue_compiler_sfc_ffi.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Parsing
// ============================================================================

/**
 * Parses a Vue SFC source string.
 *
 * @param source UTF-8 string containing the SFC source (not null-terminated).
 * @param source_len Length of the source string in bytes.
 * @param filename UTF-8 string with the filename (not null-terminated).
 * @param filename_len Length of the filename string in bytes.
 * @return Handle to the parse result object.
 */
RawHandle vue_parse(
    const char* source, size_t source_len,
    const char* filename, size_t filename_len);

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

#ifdef __cplusplus
}
#endif

#endif /* VUE_PARSE_H */
