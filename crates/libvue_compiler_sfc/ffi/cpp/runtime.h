/**
 * @file runtime.h
 * @brief Hermes runtime abstraction for FFI.
 *
 * Provides an opaque HermesRuntime type that owns:
 * - Static Hermes runtime (SHRuntime*)
 * - JSI runtime (HermesRuntime*)
 * - Handle table for JS object management
 * - Cached Vue compiler function references
 *
 * ## Thread Safety
 *
 * - A single HermesRuntime must only be used from one thread at a time
 * - Multiple HermesRuntime instances can be used in parallel from different threads
 */

#ifndef HERMES_RUNTIME_H
#define HERMES_RUNTIME_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Types
// ============================================================================

/**
 * Opaque handle to a JavaScript value in the Hermes runtime.
 *
 * Handles are 64-bit identifiers that reference entries in an internal handle
 * table. They provide a way to pass JS objects across the FFI boundary.
 *
 * - Handle 0 represents invalid/null
 * - Non-zero values are 1-indexed into the handle table
 */
typedef uint64_t HermesHandle;

/**
 * Opaque pointer to a Hermes runtime instance.
 *
 * The internal structure contains:
 * - SHRuntime* (Static Hermes runtime)
 * - HermesRuntime* (JSI runtime)
 * - Handle table (vector of JS Values)
 * - Free list for handle reuse
 * - Cached Vue compiler function references
 */
typedef struct HermesRuntimeImpl* HermesRuntime;

// ============================================================================
// Runtime Lifecycle
// ============================================================================

/**
 * Creates a new Hermes runtime instance.
 *
 * This function:
 * 1. Initializes a fresh Static Hermes runtime
 * 2. Loads the Vue compiler unit
 * 3. Caches references to Vue compiler functions (parse, compileScript, etc.)
 *
 * The runtime owns its own JavaScript heap and can execute JS code
 * independently. Each runtime must be used from only one thread at a time.
 *
 * @return Pointer to the runtime, or NULL on failure.
 */
HermesRuntime hermes_runtime_create(void);

/**
 * Destroys a Hermes runtime and releases all resources.
 *
 * All handles created by this runtime become invalid after this call.
 * Safe to call with NULL (no-op).
 *
 * @param rt The runtime to destroy.
 */
void hermes_runtime_destroy(HermesRuntime rt);

// ============================================================================
// Handle Management
// ============================================================================

/**
 * Frees a handle and releases its associated JavaScript value.
 *
 * After calling this function, the handle becomes invalid. Any strings
 * that were cached in this handle also become invalid.
 *
 * Safe to call with handle 0 (no-op).
 *
 * @param rt The runtime that owns this handle.
 * @param handle The handle to free.
 */
void hermes_handle_free(HermesRuntime rt, HermesHandle handle);

/**
 * Checks if a handle is valid (non-null).
 *
 * @param handle The handle to check.
 * @return true if valid, false if null/invalid.
 */
static inline bool hermes_handle_is_valid(HermesHandle handle) {
    return handle != 0;
}

#ifdef __cplusplus
}
#endif

#endif /* HERMES_RUNTIME_H */
