/**
 * @file vue_compile_style.h
 * @brief Style compilation API for the Vue SFC compiler FFI.
 *
 * This header provides functions for compiling Vue SFC style blocks,
 * including support for scoped CSS.
 *
 * ## Typical Usage
 *
 * 1. Parse the SFC with vue_parse()
 * 2. Get the descriptor with vue_parse_result_descriptor()
 * 3. Get style count with vue_descriptor_style_count()
 * 4. For each style block, get content and check if scoped
 * 5. Compile style with vue_compile_style()
 * 6. Get the compiled CSS with vue_style_result_code()
 * 7. Free all handles with vue_handle_free()
 */

#ifndef VUE_COMPILE_STYLE_H
#define VUE_COMPILE_STYLE_H

#include "vue_compiler_sfc_ffi.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Style Compilation
// ============================================================================

/**
 * Compiles a CSS style block.
 *
 * @param source CSS source string (not null-terminated).
 * @param source_len Length of the source string in bytes.
 * @param filename Filename string (not null-terminated).
 * @param filename_len Length of the filename string in bytes.
 * @param id Scope ID string (not null-terminated, e.g., "data-v-abc123").
 * @param id_len Length of the id string in bytes.
 * @param scoped Whether to add scoped attribute selectors.
 * @return Handle to the compilation result.
 */
RawHandle vue_compile_style(
    const char* source, size_t source_len,
    const char* filename, size_t filename_len,
    const char* id, size_t id_len,
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

#endif /* VUE_COMPILE_STYLE_H */
