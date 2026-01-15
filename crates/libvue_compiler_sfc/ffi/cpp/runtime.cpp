/**
 * @file runtime.cpp
 * @brief Implementation of HermesRuntime abstraction.
 */

#include "runtime_internal.h"

// External declaration for the compiled Vue compiler unit
extern "C" SHUnit sh_export_vue_compiler;

// ============================================================================
// Runtime Lifecycle
// ============================================================================

extern "C" HermesRuntime hermes_runtime_create(void) {
    auto* rt = new HermesRuntimeImpl();
    rt->sh_runtime = nullptr;
    rt->jsi_runtime = nullptr;

    // Initialize the Static Hermes runtime
    rt->sh_runtime = _sh_init(0, nullptr);
    if (!rt->sh_runtime) {
        delete rt;
        return nullptr;
    }

    rt->jsi_runtime = _sh_get_hermes_runtime(rt->sh_runtime);
    if (!rt->jsi_runtime) {
        delete rt;
        return nullptr;
    }

    // Load the compiled Vue compiler unit
    if (!_sh_initialize_units(rt->sh_runtime, 1, &sh_export_vue_compiler)) {
        delete rt;
        return nullptr;
    }

    // Cache references to the JavaScript functions
    auto& hermes = rt->runtime();
    auto global = hermes.global();
    rt->parse_fn = std::make_unique<facebook::jsi::Function>(
        global.getPropertyAsFunction(hermes, "parse"));
    rt->compile_script_fn = std::make_unique<facebook::jsi::Function>(
        global.getPropertyAsFunction(hermes, "compileScript"));
    rt->compile_template_fn = std::make_unique<facebook::jsi::Function>(
        global.getPropertyAsFunction(hermes, "compileTemplate"));
    rt->compile_style_fn = std::make_unique<facebook::jsi::Function>(
        global.getPropertyAsFunction(hermes, "compileStyle"));

    return rt;
}

extern "C" void hermes_runtime_destroy(HermesRuntime rt) {
    if (!rt) {
        return;
    }

    // Clear function references before destroying runtime
    rt->parse_fn.reset();
    rt->compile_script_fn.reset();
    rt->compile_template_fn.reset();
    rt->compile_style_fn.reset();

    // Clear handle table
    rt->handles.clear();
    rt->free_list.clear();

    // Note: We don't explicitly destroy the Hermes runtime here.
    // The _sh_done() function should be called, but Static Hermes
    // documentation suggests letting it leak to avoid destruction order issues.
    // If needed, uncomment: _sh_done(rt->sh_runtime);

    delete rt;
}

// ============================================================================
// Handle Management
// ============================================================================

extern "C" void hermes_handle_free(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return;
    }
    rt->free_handle(handle);
}
