/**
 * @file vue_compiler_sfc_ffi.h
 * @brief C FFI interface for the Vue SFC compiler via Static Hermes.
 *
 * This header is the single entry point for the Vue SFC compiler FFI. It defines
 * the core RawHandle type and includes all sub-headers for the complete API.
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
 *
 * ## Sub-headers
 *
 * - vue_handle.h: Handle management (vue_handle_free)
 * - vue_parse.h: Parsing and descriptor/block accessors
 * - vue_compile_script.h: Script compilation
 * - vue_compile_template.h: Template compilation
 * - vue_compile_style.h: Style compilation
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

#ifdef __cplusplus
}
#endif

/* Include all sub-headers */
#include "vue_handle.h"
#include "vue_parse.h"
#include "vue_compile_script.h"
#include "vue_compile_template.h"
#include "vue_compile_style.h"

#endif /* VUE_COMPILER_SFC_FFI_H */
