/**
 * @file runtime_internal.h
 * @brief Internal implementation details for HermesRuntime.
 *
 * This header is NOT part of the public API. It is only included by
 * runtime.cpp and vue_sfc.cpp to access the internal struct.
 */

#ifndef HERMES_RUNTIME_INTERNAL_H
#define HERMES_RUNTIME_INTERNAL_H

#include "runtime.h"

#include <memory>
#include <vector>
#include <string>

#include <hermes/VM/static_h.h>
#include <hermes/hermes.h>
#include <jsi/jsi.h>

/**
 * Entry in the handle table.
 *
 * Each entry contains:
 * - value: The JSI Value wrapped in a shared_ptr for safe copying
 * - cached_strings: Cached string extractions (owned by this entry)
 */
struct HandleEntry {
    /// The JavaScript value this handle refers to.
    std::shared_ptr<facebook::jsi::Value> value;

    /// Cached string values extracted from this object.
    /// Strings are stored here so their pointers remain valid until the handle is freed.
    std::vector<std::string> cached_strings;
};

/**
 * Internal implementation of HermesRuntime.
 */
struct HermesRuntimeImpl {
    // Hermes runtime
    SHRuntime* sh_runtime;
    facebook::hermes::HermesRuntime* jsi_runtime;

    // Handle table
    std::vector<HandleEntry> handles;
    std::vector<uint64_t> free_list;

    // Cached Vue compiler function references
    std::unique_ptr<facebook::jsi::Function> parse_fn;
    std::unique_ptr<facebook::jsi::Function> compile_script_fn;
    std::unique_ptr<facebook::jsi::Function> compile_template_fn;
    std::unique_ptr<facebook::jsi::Function> compile_style_fn;

    // -------------------------------------------------------------------------
    // Handle Management Methods
    // -------------------------------------------------------------------------

    /**
     * Allocates a new handle for a JavaScript value.
     *
     * @param val The JSI Value to store (moved).
     * @return A 1-indexed handle ID. Never returns 0.
     */
    HermesHandle allocate_handle(facebook::jsi::Value&& val) {
        auto entry = HandleEntry{
            std::make_shared<facebook::jsi::Value>(std::move(val)),
            {}
        };

        // Reuse a free slot if available
        if (!free_list.empty()) {
            uint64_t idx = free_list.back();
            free_list.pop_back();
            handles[idx] = std::move(entry);
            return idx + 1;  // Convert to 1-indexed
        }

        // Allocate a new slot
        handles.push_back(std::move(entry));
        return handles.size();  // 1-indexed
    }

    /**
     * Gets the handle entry for a given handle ID.
     *
     * @param handle The 1-indexed handle ID.
     * @return Pointer to the entry, or nullptr if the handle is invalid.
     */
    HandleEntry* get_handle(HermesHandle handle) {
        if (handle == 0 || handle > handles.size()) {
            return nullptr;
        }
        auto* entry = &handles[handle - 1];
        // Check if the slot has been freed (empty value)
        if (!entry->value) {
            return nullptr;
        }
        return entry;
    }

    /**
     * Frees a handle and releases its resources.
     *
     * @param handle The 1-indexed handle ID.
     */
    void free_handle(HermesHandle handle) {
        if (handle == 0 || handle > handles.size()) {
            return;
        }

        // Clear the entry and add to free list
        handles[handle - 1] = HandleEntry{};
        free_list.push_back(handle - 1);
    }

    /**
     * Caches a string in a handle entry and returns a pointer to it.
     *
     * @param handle The handle to cache the string in.
     * @param str The string to cache.
     * @return Pointer to the cached string, or "" if the handle is invalid.
     */
    const char* cache_string(HermesHandle handle, const std::string& str) {
        auto* entry = get_handle(handle);
        if (!entry) {
            return "";
        }
        entry->cached_strings.push_back(str);
        return entry->cached_strings.back().c_str();
    }

    // -------------------------------------------------------------------------
    // Runtime Access
    // -------------------------------------------------------------------------

    facebook::hermes::HermesRuntime& runtime() {
        return *jsi_runtime;
    }
};

#endif /* HERMES_RUNTIME_INTERNAL_H */
