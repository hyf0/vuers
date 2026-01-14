/**
 * @file vue_compiler_sfc_ffi.cpp
 * @brief C++ FFI wrapper for the Vue SFC compiler via Static Hermes.
 *
 * This file provides the bridge between Rust and the Vue compiler running in the
 * Hermes JavaScript runtime. It implements a handle-based API that allows Rust code
 * to work with JavaScript objects without directly managing JSI values.
 *
 * ## Architecture
 *
 * The wrapper maintains:
 * - A single Hermes runtime instance (lazily initialized)
 * - Cached references to the JavaScript compiler functions
 * - A handle table that maps integer IDs to JSI Value objects
 *
 * ## Handle System
 *
 * JavaScript objects are exposed to Rust as opaque 64-bit handles:
 * - Handle 0 is reserved as the invalid/null handle
 * - Valid handles are 1-indexed into the handle table
 * - Each handle entry stores the JSI Value and any cached string extractions
 * - Handles must be explicitly freed to avoid memory leaks
 *
 * ## Thread Safety
 *
 * This code is NOT thread-safe. The Hermes runtime and all handle operations
 * must be accessed from a single thread. The static variables are intentionally
 * not protected by mutexes for performance reasons.
 *
 * ## Memory Management
 *
 * - String pointers returned by accessor functions are owned by the handle
 * - Strings are cached in the handle entry's `strings` vector
 * - When a handle is freed, all its cached strings become invalid
 * - The runtime and function pointers are intentionally leaked to avoid
 *   destruction order issues at program exit
 */

#include "vue_compiler_sfc_ffi.h"

#include <stdlib.h>
#include <string.h>
#include <memory>
#include <vector>
#include <string>

#include <hermes/VM/static_h.h>
#include <hermes/hermes.h>
#include <jsi/jsi.h>

// ============================================================================
// External Declarations
// ============================================================================

/**
 * Declaration for the `vue_compiler` unit created by Static Hermes.
 * This is the compiled JavaScript code from vue-compiler.js.
 */
extern "C" SHUnit sh_export_vue_compiler;

// ============================================================================
// Global State
// ============================================================================

/// The Static Hermes runtime handle.
static SHRuntime *s_shRuntime = nullptr;

/// The Hermes runtime instance (owned by s_shRuntime).
static facebook::hermes::HermesRuntime *s_hermes = nullptr;

/**
 * Cached JavaScript function references.
 *
 * These are intentionally raw pointers that are never freed. This avoids
 * destruction order issues when the program exits, as the Hermes runtime
 * may be destroyed before these function objects.
 */
static facebook::jsi::Function* s_parseFn = nullptr;
static facebook::jsi::Function* s_compileScriptFn = nullptr;
static facebook::jsi::Function* s_compileTemplateFn = nullptr;
static facebook::jsi::Function* s_compileStyleFn = nullptr;

// ============================================================================
// Handle Table
// ============================================================================

/**
 * Entry in the handle table.
 *
 * Each entry contains:
 * - value: The JSI Value wrapped in a shared_ptr for safe copying
 * - strings: Cached string extractions (owned by this entry)
 */
struct HandleEntry {
    /// The JavaScript value this handle refers to.
    std::shared_ptr<facebook::jsi::Value> value;

    /// Cached string values extracted from this object.
    /// Strings are stored here so their pointers remain valid until the handle is freed.
    std::vector<std::string> strings;
};

/// The handle table mapping handle IDs to entries.
/// Handles are 1-indexed, so handle N refers to s_handles[N-1].
static std::vector<HandleEntry> s_handles;

/// Free list of available handle slots for reuse.
/// Contains 0-based indices into s_handles.
static std::vector<uint64_t> s_free_list;

// ============================================================================
// Handle Management (Internal)
// ============================================================================

/**
 * Allocates a new handle for a JavaScript value.
 *
 * @param val The JSI Value to store (moved).
 * @return A 1-indexed handle ID. Never returns 0.
 */
static uint64_t allocate_handle(facebook::jsi::Value&& val) {
    auto entry = HandleEntry{
        std::make_shared<facebook::jsi::Value>(std::move(val)),
        {}
    };

    // Reuse a free slot if available
    if (!s_free_list.empty()) {
        uint64_t idx = s_free_list.back();
        s_free_list.pop_back();
        s_handles[idx] = std::move(entry);
        return idx + 1;  // Convert to 1-indexed
    }

    // Allocate a new slot
    s_handles.push_back(std::move(entry));
    return s_handles.size();  // 1-indexed
}

/**
 * Gets the handle entry for a given handle ID.
 *
 * @param handle The 1-indexed handle ID.
 * @return Pointer to the entry, or nullptr if the handle is invalid.
 */
static HandleEntry* get_handle(uint64_t handle) {
    if (handle == 0 || handle > s_handles.size()) {
        return nullptr;
    }
    auto* entry = &s_handles[handle - 1];
    // Check if the slot has been freed (empty value)
    if (!entry->value) {
        return nullptr;
    }
    return entry;
}

/**
 * Caches a string in a handle entry and returns a pointer to it.
 *
 * The string is copied into the handle's string cache, ensuring the pointer
 * remains valid until the handle is freed.
 *
 * @param handle The handle to cache the string in.
 * @param str The string to cache.
 * @return Pointer to the cached string, or "" if the handle is invalid.
 */
static const char* cache_string(uint64_t handle, const std::string& str) {
    auto* entry = get_handle(handle);
    if (!entry) {
        return "";
    }
    entry->strings.push_back(str);
    return entry->strings.back().c_str();
}

// ============================================================================
// Runtime Initialization
// ============================================================================

/**
 * Initializes the Hermes runtime and caches JavaScript function references.
 *
 * This function is idempotent - it only initializes on the first call.
 * Subsequent calls return immediately.
 *
 * On initialization failure, the function calls abort() as there is no
 * recovery path.
 */
static void init_runtime() {
    if (s_shRuntime != nullptr) {
        return;
    }

    // Initialize the Static Hermes runtime
    s_shRuntime = _sh_init(0, nullptr);
    s_hermes = _sh_get_hermes_runtime(s_shRuntime);

    // Load the compiled Vue compiler unit
    if (!_sh_initialize_units(s_shRuntime, 1, &sh_export_vue_compiler)) {
        // Fatal error: cannot continue without the compiler
        abort();
    }

    // Cache references to the JavaScript functions
    // These are allocated with `new` and intentionally never freed
    auto global = s_hermes->global();
    s_parseFn = new facebook::jsi::Function(
        global.getPropertyAsFunction(*s_hermes, "parse"));
    s_compileScriptFn = new facebook::jsi::Function(
        global.getPropertyAsFunction(*s_hermes, "compileScript"));
    s_compileTemplateFn = new facebook::jsi::Function(
        global.getPropertyAsFunction(*s_hermes, "compileTemplate"));
    s_compileStyleFn = new facebook::jsi::Function(
        global.getPropertyAsFunction(*s_hermes, "compileStyle"));
}

// ============================================================================
// FFI Functions - Handle Management
// ============================================================================

/**
 * Frees a handle and releases its resources.
 *
 * After this call, the handle becomes invalid. Any strings returned by
 * accessor functions for this handle also become invalid.
 *
 * This function is safe to call with handle 0 (no-op).
 */
extern "C" void vue_handle_free(uint64_t handle) {
    if (handle == 0 || handle > s_handles.size()) {
        return;
    }

    // Clear the entry and add to free list
    s_handles[handle - 1] = HandleEntry{};
    s_free_list.push_back(handle - 1);
}

// ============================================================================
// FFI Functions - Parsing
// ============================================================================

/**
 * Parses a Vue SFC source string.
 *
 * @param source Null-terminated UTF-8 string containing the SFC source.
 * @param filename Null-terminated UTF-8 string with the filename (for errors).
 * @return Handle to the parse result object.
 */
extern "C" uint64_t vue_parse(const char* source, const char* filename) {
    init_runtime();
    auto& rt = *s_hermes;

    auto jsSource = facebook::jsi::String::createFromUtf8(rt, source);
    auto jsFilename = facebook::jsi::String::createFromUtf8(rt, filename);
    auto result = s_parseFn->call(rt, jsSource, jsFilename);

    return allocate_handle(std::move(result));
}

/**
 * Gets the descriptor handle from a parse result.
 */
extern "C" uint64_t vue_parse_result_descriptor(uint64_t handle) {
    auto* entry = get_handle(handle);
    if (!entry) {
        return 0;
    }

    auto& rt = *s_hermes;
    auto obj = entry->value->getObject(rt);
    auto desc = obj.getProperty(rt, "descriptor");

    if (desc.isNull() || desc.isUndefined()) {
        return 0;
    }

    return allocate_handle(std::move(desc));
}

/**
 * Gets the number of parse errors.
 */
extern "C" size_t vue_parse_result_error_count(uint64_t handle) {
    auto* entry = get_handle(handle);
    if (!entry) {
        return 0;
    }

    auto& rt = *s_hermes;
    auto obj = entry->value->getObject(rt);
    auto errors = obj.getProperty(rt, "errors");

    if (!errors.isObject()) {
        return 0;
    }

    return errors.getObject(rt).getArray(rt).size(rt);
}

/**
 * Gets an error message at the specified index.
 */
extern "C" const char* vue_parse_result_error_message(uint64_t handle, size_t index) {
    auto* entry = get_handle(handle);
    if (!entry) {
        return "";
    }

    auto& rt = *s_hermes;
    auto obj = entry->value->getObject(rt);
    auto errors = obj.getProperty(rt, "errors").getObject(rt).getArray(rt);

    if (index >= errors.size(rt)) {
        return "";
    }

    auto err = errors.getValueAtIndex(rt, index).getObject(rt);
    auto msg = err.getProperty(rt, "message").getString(rt).utf8(rt);
    return cache_string(handle, msg);
}

// ============================================================================
// FFI Functions - Descriptor Accessors
// ============================================================================

/**
 * Checks if the descriptor has a template block.
 */
extern "C" bool vue_descriptor_has_template(uint64_t handle) {
    auto* entry = get_handle(handle);
    if (!entry) {
        return false;
    }

    auto& rt = *s_hermes;
    auto obj = entry->value->getObject(rt);
    auto tmpl = obj.getProperty(rt, "template");
    return !tmpl.isNull() && !tmpl.isUndefined();
}

/**
 * Checks if the descriptor has a script block.
 */
extern "C" bool vue_descriptor_has_script(uint64_t handle) {
    auto* entry = get_handle(handle);
    if (!entry) {
        return false;
    }

    auto& rt = *s_hermes;
    auto obj = entry->value->getObject(rt);
    auto script = obj.getProperty(rt, "script");
    return !script.isNull() && !script.isUndefined();
}

/**
 * Checks if the descriptor has a script setup block.
 */
extern "C" bool vue_descriptor_has_script_setup(uint64_t handle) {
    auto* entry = get_handle(handle);
    if (!entry) {
        return false;
    }

    auto& rt = *s_hermes;
    auto obj = entry->value->getObject(rt);
    auto script = obj.getProperty(rt, "scriptSetup");
    return !script.isNull() && !script.isUndefined();
}

/**
 * Gets the number of style blocks.
 */
extern "C" size_t vue_descriptor_style_count(uint64_t handle) {
    auto* entry = get_handle(handle);
    if (!entry) {
        return 0;
    }

    auto& rt = *s_hermes;
    auto obj = entry->value->getObject(rt);
    auto styles = obj.getProperty(rt, "styles");

    if (!styles.isObject()) {
        return 0;
    }

    return styles.getObject(rt).getArray(rt).size(rt);
}

/**
 * Gets the template block handle.
 */
extern "C" uint64_t vue_descriptor_template(uint64_t handle) {
    auto* entry = get_handle(handle);
    if (!entry) {
        return 0;
    }

    auto& rt = *s_hermes;
    auto obj = entry->value->getObject(rt);
    auto tmpl = obj.getProperty(rt, "template");

    if (tmpl.isNull() || tmpl.isUndefined()) {
        return 0;
    }

    return allocate_handle(std::move(tmpl));
}

/**
 * Gets the script setup block handle.
 */
extern "C" uint64_t vue_descriptor_script_setup(uint64_t handle) {
    auto* entry = get_handle(handle);
    if (!entry) {
        return 0;
    }

    auto& rt = *s_hermes;
    auto obj = entry->value->getObject(rt);
    auto script = obj.getProperty(rt, "scriptSetup");

    if (script.isNull() || script.isUndefined()) {
        return 0;
    }

    return allocate_handle(std::move(script));
}

/**
 * Gets a style block handle at the specified index.
 */
extern "C" uint64_t vue_descriptor_style_at(uint64_t handle, size_t index) {
    auto* entry = get_handle(handle);
    if (!entry) {
        return 0;
    }

    auto& rt = *s_hermes;
    auto obj = entry->value->getObject(rt);
    auto styles = obj.getProperty(rt, "styles").getObject(rt).getArray(rt);

    if (index >= styles.size(rt)) {
        return 0;
    }

    auto style = styles.getValueAtIndex(rt, index);
    return allocate_handle(std::move(style));
}

// ============================================================================
// FFI Functions - Block Accessors
// ============================================================================

/**
 * Gets the content of a block.
 *
 * Works for template, script, and style blocks.
 */
extern "C" const char* vue_block_content(uint64_t handle) {
    auto* entry = get_handle(handle);
    if (!entry) {
        return "";
    }

    auto& rt = *s_hermes;
    auto obj = entry->value->getObject(rt);
    auto content = obj.getProperty(rt, "content");

    if (!content.isString()) {
        return "";
    }

    return cache_string(handle, content.getString(rt).utf8(rt));
}

/**
 * Gets the lang attribute of a block.
 */
extern "C" const char* vue_block_lang(uint64_t handle) {
    auto* entry = get_handle(handle);
    if (!entry) {
        return "";
    }

    auto& rt = *s_hermes;
    auto obj = entry->value->getObject(rt);
    auto lang = obj.getProperty(rt, "lang");

    if (!lang.isString()) {
        return "";
    }

    return cache_string(handle, lang.getString(rt).utf8(rt));
}

/**
 * Checks if a style block is scoped.
 */
extern "C" bool vue_style_is_scoped(uint64_t handle) {
    auto* entry = get_handle(handle);
    if (!entry) {
        return false;
    }

    auto& rt = *s_hermes;
    auto obj = entry->value->getObject(rt);
    auto scoped = obj.getProperty(rt, "scoped");
    return scoped.isBool() && scoped.getBool();
}

// ============================================================================
// FFI Functions - Script Compilation
// ============================================================================

/**
 * Compiles the script blocks of an SFC.
 *
 * @param desc_handle Handle to the SFC descriptor.
 * @param id Null-terminated scope ID string.
 * @param is_prod Whether to compile for production.
 * @return Handle to the compilation result.
 */
extern "C" uint64_t vue_compile_script(uint64_t desc_handle, const char* id, bool is_prod) {
    init_runtime();

    auto* entry = get_handle(desc_handle);
    if (!entry) {
        return 0;
    }

    auto& rt = *s_hermes;
    auto jsId = facebook::jsi::String::createFromUtf8(rt, id);
    auto result = s_compileScriptFn->call(rt, *entry->value, jsId, is_prod);

    return allocate_handle(std::move(result));
}

/**
 * Gets the compiled script content.
 */
extern "C" const char* vue_script_result_content(uint64_t handle) {
    auto* entry = get_handle(handle);
    if (!entry) {
        return "";
    }

    auto& rt = *s_hermes;
    auto obj = entry->value->getObject(rt);
    auto content = obj.getProperty(rt, "content");

    if (!content.isString()) {
        return "";
    }

    return cache_string(handle, content.getString(rt).utf8(rt));
}

/**
 * Gets the bindings handle from a script result.
 */
extern "C" uint64_t vue_script_result_bindings(uint64_t handle) {
    auto* entry = get_handle(handle);
    if (!entry) {
        return 0;
    }

    auto& rt = *s_hermes;
    auto obj = entry->value->getObject(rt);
    auto bindings = obj.getProperty(rt, "bindings");

    if (bindings.isNull() || bindings.isUndefined()) {
        return 0;
    }

    return allocate_handle(std::move(bindings));
}

// ============================================================================
// FFI Functions - Template Compilation
// ============================================================================

/**
 * Compiles a Vue template to a render function.
 *
 * @param source Null-terminated template source string.
 * @param filename Null-terminated filename string.
 * @param id Null-terminated scope ID string.
 * @param scoped Whether to add scoped attribute selectors.
 * @param bindings_handle Handle to bindings object, or 0 for none.
 * @return Handle to the compilation result.
 */
extern "C" uint64_t vue_compile_template(
    const char* source,
    const char* filename,
    const char* id,
    bool scoped,
    uint64_t bindings_handle
) {
    init_runtime();
    auto& rt = *s_hermes;

    auto jsSource = facebook::jsi::String::createFromUtf8(rt, source);
    auto jsFilename = facebook::jsi::String::createFromUtf8(rt, filename);
    auto jsId = facebook::jsi::String::createFromUtf8(rt, id);

    // Handle optional bindings parameter
    facebook::jsi::Value jsBindings = facebook::jsi::Value::null();
    if (bindings_handle != 0) {
        auto* bindings_entry = get_handle(bindings_handle);
        if (bindings_entry) {
            jsBindings = facebook::jsi::Value(rt, *bindings_entry->value);
        }
    }

    auto result = s_compileTemplateFn->call(rt, jsSource, jsFilename, jsId, scoped, jsBindings);
    return allocate_handle(std::move(result));
}

/**
 * Gets the compiled template code (render function).
 */
extern "C" const char* vue_template_result_code(uint64_t handle) {
    auto* entry = get_handle(handle);
    if (!entry) {
        return "";
    }

    auto& rt = *s_hermes;
    auto obj = entry->value->getObject(rt);
    auto code = obj.getProperty(rt, "code");

    if (!code.isString()) {
        return "";
    }

    return cache_string(handle, code.getString(rt).utf8(rt));
}

/**
 * Gets the number of template compilation errors.
 */
extern "C" size_t vue_template_result_error_count(uint64_t handle) {
    auto* entry = get_handle(handle);
    if (!entry) {
        return 0;
    }

    auto& rt = *s_hermes;
    auto obj = entry->value->getObject(rt);
    auto errors = obj.getProperty(rt, "errors");

    if (!errors.isObject()) {
        return 0;
    }

    return errors.getObject(rt).getArray(rt).size(rt);
}

// ============================================================================
// FFI Functions - Style Compilation
// ============================================================================

/**
 * Compiles a CSS style block.
 *
 * @param source Null-terminated CSS source string.
 * @param filename Null-terminated filename string.
 * @param id Null-terminated scope ID string.
 * @param scoped Whether to add scoped attribute selectors.
 * @return Handle to the compilation result.
 */
extern "C" uint64_t vue_compile_style(
    const char* source,
    const char* filename,
    const char* id,
    bool scoped
) {
    init_runtime();
    auto& rt = *s_hermes;

    auto jsSource = facebook::jsi::String::createFromUtf8(rt, source);
    auto jsFilename = facebook::jsi::String::createFromUtf8(rt, filename);
    auto jsId = facebook::jsi::String::createFromUtf8(rt, id);

    auto result = s_compileStyleFn->call(rt, jsSource, jsFilename, jsId, scoped);
    return allocate_handle(std::move(result));
}

/**
 * Gets the compiled CSS code.
 */
extern "C" const char* vue_style_result_code(uint64_t handle) {
    auto* entry = get_handle(handle);
    if (!entry) {
        return "";
    }

    auto& rt = *s_hermes;
    auto obj = entry->value->getObject(rt);
    auto code = obj.getProperty(rt, "code");

    if (!code.isString()) {
        return "";
    }

    return cache_string(handle, code.getString(rt).utf8(rt));
}
