/**
 * @file vue_compile_script.h
 * @brief Script compilation API for the Vue SFC compiler FFI.
 *
 * This header provides functions for compiling Vue SFC script blocks,
 * including both regular `<script>` and `<script setup>` blocks.
 *
 * ## Typical Usage
 *
 * 1. Parse the SFC with vue_parse()
 * 2. Get the descriptor with vue_parse_result_descriptor()
 * 3. Check for script blocks with vue_descriptor_has_script() or vue_descriptor_has_script_setup()
 * 4. Compile scripts with vue_compile_script()
 * 5. Get bindings with vue_script_result_bindings() for template optimization
 * 6. Free all handles with vue_handle_free()
 */

#ifndef VUE_COMPILE_SCRIPT_H
#define VUE_COMPILE_SCRIPT_H

#include "vue_compiler_sfc_ffi.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Script Compilation
// ============================================================================

/**
 * Compiles the script blocks of an SFC.
 *
 * @param desc_handle Handle to the SFC descriptor.
 * @param id Scope ID string (not null-terminated, e.g., "data-v-abc123").
 * @param id_len Length of the id string in bytes.
 * @param is_prod Whether to compile for production.
 * @return Handle to the compilation result.
 */
RawHandle vue_compile_script(
    RawHandle desc_handle,
    const char* id, size_t id_len,
    bool is_prod);

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

#ifdef __cplusplus
}
#endif

#endif /* VUE_COMPILE_SCRIPT_H */
