/**
 * @file vue_compile_template.h
 * @brief Template compilation API for the Vue SFC compiler FFI.
 *
 * This header provides functions for compiling Vue templates to render functions.
 *
 * ## Typical Usage
 *
 * 1. Parse the SFC with vue_parse()
 * 2. Get the descriptor and template block
 * 3. Optionally compile scripts with vue_compile_script() to get bindings
 * 4. Compile template with vue_compile_template()
 * 5. Get the render function code with vue_template_result_code()
 * 6. Free all handles with vue_handle_free()
 */

#ifndef VUE_COMPILE_TEMPLATE_H
#define VUE_COMPILE_TEMPLATE_H

#include "vue_compiler_sfc_ffi.h"

#ifdef __cplusplus
extern "C" {
#endif

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

#ifdef __cplusplus
}
#endif

#endif /* VUE_COMPILE_TEMPLATE_H */
