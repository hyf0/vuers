/**
 * @file vue_handle.h
 * @brief Handle management for the Vue SFC compiler FFI.
 *
 * This header provides the handle management API for the Vue SFC compiler.
 * Handles are opaque 64-bit integers that reference JavaScript objects in the
 * Hermes runtime.
 *
 * ## Handle Lifecycle
 *
 * - Handles are returned by parsing and compilation functions
 * - Handle 0 is reserved as the invalid/null handle
 * - Handles must be explicitly freed with vue_handle_free() to avoid memory leaks
 * - String pointers returned by accessor functions are owned by the handle
 *   and become invalid after the handle is freed
 */

#ifndef VUE_HANDLE_H
#define VUE_HANDLE_H

#include "vue_compiler_sfc_ffi.h"

#ifdef __cplusplus
extern "C" {
#endif

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

#ifdef __cplusplus
}
#endif

#endif /* VUE_HANDLE_H */
