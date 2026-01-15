/**
 * @file vue_sfc.h
 * @brief Vue SFC compiler FFI API.
 *
 * Provides functions for parsing and compiling Vue Single File Components
 * using a HermesRuntime instance.
 *
 * All functions take HermesRuntime as the first parameter.
 */

#ifndef VUE_SFC_H
#define VUE_SFC_H

#include "runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Parsing
// ============================================================================

/**
 * Parses a Vue SFC source string.
 *
 * @param rt The Hermes runtime.
 * @param source UTF-8 source string (not null-terminated).
 * @param source_len Length of source in bytes.
 * @param filename UTF-8 filename (not null-terminated).
 * @param filename_len Length of filename in bytes.
 * @return Handle to parse result, or 0 on failure.
 */
HermesHandle vue_parse(
    HermesRuntime rt,
    const char* source, size_t source_len,
    const char* filename, size_t filename_len
);

/**
 * Gets the descriptor handle from a parse result.
 */
HermesHandle vue_parse_result_descriptor(HermesRuntime rt, HermesHandle handle);

/**
 * Gets the number of parse errors.
 */
size_t vue_parse_result_error_count(HermesRuntime rt, HermesHandle handle);

/**
 * Gets an error message at the specified index.
 */
const char* vue_parse_result_error_message(HermesRuntime rt, HermesHandle handle, size_t index);

// ============================================================================
// Descriptor Accessors
// ============================================================================

/**
 * Checks if the descriptor has a template block.
 */
bool vue_descriptor_has_template(HermesRuntime rt, HermesHandle handle);

/**
 * Checks if the descriptor has a script block (not setup).
 */
bool vue_descriptor_has_script(HermesRuntime rt, HermesHandle handle);

/**
 * Checks if the descriptor has a script setup block.
 */
bool vue_descriptor_has_script_setup(HermesRuntime rt, HermesHandle handle);

/**
 * Gets the number of style blocks.
 */
size_t vue_descriptor_style_count(HermesRuntime rt, HermesHandle handle);

/**
 * Gets the template block handle.
 */
HermesHandle vue_descriptor_template(HermesRuntime rt, HermesHandle handle);

/**
 * Gets the script block handle (not setup).
 */
HermesHandle vue_descriptor_script(HermesRuntime rt, HermesHandle handle);

/**
 * Gets the script setup block handle.
 */
HermesHandle vue_descriptor_script_setup(HermesRuntime rt, HermesHandle handle);

/**
 * Gets a style block handle by index.
 */
HermesHandle vue_descriptor_style_at(HermesRuntime rt, HermesHandle handle, size_t index);

/**
 * Gets the number of custom blocks.
 */
size_t vue_descriptor_custom_blocks_count(HermesRuntime rt, HermesHandle handle);

/**
 * Gets a custom block handle by index.
 */
HermesHandle vue_descriptor_custom_block_at(HermesRuntime rt, HermesHandle handle, size_t index);

/**
 * Gets the number of CSS variables.
 */
size_t vue_descriptor_css_vars_count(HermesRuntime rt, HermesHandle handle);

/**
 * Gets a CSS variable by index.
 */
const char* vue_descriptor_css_var_at(HermesRuntime rt, HermesHandle handle, size_t index);

/**
 * Checks if the descriptor uses :slotted().
 */
bool vue_descriptor_slotted(HermesRuntime rt, HermesHandle handle);

/**
 * Gets the source property of a descriptor.
 */
const char* vue_descriptor_source(HermesRuntime rt, HermesHandle handle);

/**
 * Gets the filename property of a descriptor.
 */
const char* vue_descriptor_filename(HermesRuntime rt, HermesHandle handle);

// ============================================================================
// Block Accessors
// ============================================================================

/**
 * Gets the content of a block.
 */
const char* vue_block_content(HermesRuntime rt, HermesHandle handle);

/**
 * Gets the lang attribute of a block.
 */
const char* vue_block_lang(HermesRuntime rt, HermesHandle handle);

/**
 * Gets the src attribute of a block.
 */
const char* vue_block_src(HermesRuntime rt, HermesHandle handle);

/**
 * Gets the type of a custom block.
 */
const char* vue_custom_block_type(HermesRuntime rt, HermesHandle handle);

// ============================================================================
// Block Location Accessors
// ============================================================================

size_t vue_block_loc_start_offset(HermesRuntime rt, HermesHandle handle);
size_t vue_block_loc_start_line(HermesRuntime rt, HermesHandle handle);
size_t vue_block_loc_start_column(HermesRuntime rt, HermesHandle handle);
size_t vue_block_loc_end_offset(HermesRuntime rt, HermesHandle handle);
size_t vue_block_loc_end_line(HermesRuntime rt, HermesHandle handle);
size_t vue_block_loc_end_column(HermesRuntime rt, HermesHandle handle);

// ============================================================================
// Block Attribute Accessors
// ============================================================================

size_t vue_block_attrs_count(HermesRuntime rt, HermesHandle handle);
const char* vue_block_attrs_key_at(HermesRuntime rt, HermesHandle handle, size_t index);
const char* vue_block_attrs_value_at(HermesRuntime rt, HermesHandle handle, size_t index);
bool vue_block_attrs_is_bool_at(HermesRuntime rt, HermesHandle handle, size_t index);

// ============================================================================
// Style Block Accessors
// ============================================================================

/**
 * Checks if a style block has the scoped attribute.
 */
bool vue_style_is_scoped(HermesRuntime rt, HermesHandle handle);

/**
 * Checks if a style block has the module attribute.
 */
bool vue_style_has_module(HermesRuntime rt, HermesHandle handle);

/**
 * Gets the module attribute value as string.
 */
const char* vue_style_module_value(HermesRuntime rt, HermesHandle handle);

// ============================================================================
// Script Block Accessors
// ============================================================================

bool vue_script_has_setup(HermesRuntime rt, HermesHandle handle);
const char* vue_script_setup_value(HermesRuntime rt, HermesHandle handle);
size_t vue_script_bindings_count(HermesRuntime rt, HermesHandle handle);
const char* vue_script_bindings_key_at(HermesRuntime rt, HermesHandle handle, size_t index);
const char* vue_script_bindings_value_at(HermesRuntime rt, HermesHandle handle, size_t index);
size_t vue_script_imports_count(HermesRuntime rt, HermesHandle handle);
const char* vue_script_imports_key_at(HermesRuntime rt, HermesHandle handle, size_t index);
HermesHandle vue_script_imports_value_at(HermesRuntime rt, HermesHandle handle, size_t index);
bool vue_import_binding_is_type(HermesRuntime rt, HermesHandle handle);
const char* vue_import_binding_imported(HermesRuntime rt, HermesHandle handle);
const char* vue_import_binding_source(HermesRuntime rt, HermesHandle handle);
bool vue_import_binding_is_from_setup(HermesRuntime rt, HermesHandle handle);
size_t vue_script_warnings_count(HermesRuntime rt, HermesHandle handle);
const char* vue_script_warning_at(HermesRuntime rt, HermesHandle handle, size_t index);
size_t vue_script_deps_count(HermesRuntime rt, HermesHandle handle);
const char* vue_script_dep_at(HermesRuntime rt, HermesHandle handle, size_t index);

// ============================================================================
// Script Compilation
// ============================================================================

/**
 * Compiles the script blocks of an SFC descriptor.
 */
HermesHandle vue_compile_script(
    HermesRuntime rt,
    HermesHandle descriptor,
    const char* id, size_t id_len,
    bool is_prod
);

/**
 * Gets the compiled script content.
 */
const char* vue_script_result_content(HermesRuntime rt, HermesHandle handle);

/**
 * Gets the bindings handle from a script compilation result.
 */
HermesHandle vue_script_result_bindings(HermesRuntime rt, HermesHandle handle);

// ============================================================================
// Template Compilation
// ============================================================================

/**
 * Compiles a Vue template to a render function.
 */
HermesHandle vue_compile_template(
    HermesRuntime rt,
    const char* source, size_t source_len,
    const char* filename, size_t filename_len,
    const char* id, size_t id_len,
    bool scoped,
    HermesHandle bindings
);

/**
 * Gets the compiled template code.
 */
const char* vue_template_result_code(HermesRuntime rt, HermesHandle handle);

/**
 * Gets the number of template compilation errors.
 */
size_t vue_template_result_error_count(HermesRuntime rt, HermesHandle handle);

// ============================================================================
// Style Compilation
// ============================================================================

/**
 * Compiles a CSS style block.
 */
HermesHandle vue_compile_style(
    HermesRuntime rt,
    const char* source, size_t source_len,
    const char* filename, size_t filename_len,
    const char* id, size_t id_len,
    bool scoped
);

/**
 * Gets the compiled CSS code.
 */
const char* vue_style_result_code(HermesRuntime rt, HermesHandle handle);

#ifdef __cplusplus
}
#endif

#endif /* VUE_SFC_H */
