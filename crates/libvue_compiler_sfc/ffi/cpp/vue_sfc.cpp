/**
 * @file vue_sfc.cpp
 * @brief Vue SFC compiler FFI implementation.
 */

#include "vue_sfc.h"
#include "runtime_internal.h"

// ============================================================================
// Parsing
// ============================================================================

extern "C" HermesHandle vue_parse(
    HermesRuntime rt,
    const char* source, size_t source_len,
    const char* filename, size_t filename_len
) {
    if (!rt) {
        return 0;
    }

    auto& hermes = rt->runtime();
    auto jsSource = facebook::jsi::String::createFromUtf8(
        hermes, reinterpret_cast<const uint8_t*>(source), source_len);
    auto jsFilename = facebook::jsi::String::createFromUtf8(
        hermes, reinterpret_cast<const uint8_t*>(filename), filename_len);
    auto result = rt->parse_fn->call(hermes, jsSource, jsFilename);

    return rt->allocate_handle(std::move(result));
}

extern "C" HermesHandle vue_parse_result_descriptor(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return 0;
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return 0;
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto desc = obj.getProperty(hermes, "descriptor");

    if (desc.isNull() || desc.isUndefined()) {
        return 0;
    }

    return rt->allocate_handle(std::move(desc));
}

extern "C" size_t vue_parse_result_error_count(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return 0;
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return 0;
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto errors = obj.getProperty(hermes, "errors");

    if (!errors.isObject()) {
        return 0;
    }

    return errors.getObject(hermes).getArray(hermes).size(hermes);
}

extern "C" const char* vue_parse_result_error_message(HermesRuntime rt, HermesHandle handle, size_t index) {
    if (!rt) {
        return "";
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return "";
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto errors = obj.getProperty(hermes, "errors").getObject(hermes).getArray(hermes);

    if (index >= errors.size(hermes)) {
        return "";
    }

    auto err = errors.getValueAtIndex(hermes, index).getObject(hermes);
    auto msg = err.getProperty(hermes, "message").getString(hermes).utf8(hermes);
    return rt->cache_string(handle, msg);
}

// ============================================================================
// Descriptor Accessors
// ============================================================================

extern "C" bool vue_descriptor_has_template(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return false;
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return false;
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto tmpl = obj.getProperty(hermes, "template");
    return !tmpl.isNull() && !tmpl.isUndefined();
}

extern "C" bool vue_descriptor_has_script(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return false;
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return false;
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto script = obj.getProperty(hermes, "script");
    return !script.isNull() && !script.isUndefined();
}

extern "C" bool vue_descriptor_has_script_setup(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return false;
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return false;
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto script = obj.getProperty(hermes, "scriptSetup");
    return !script.isNull() && !script.isUndefined();
}

extern "C" size_t vue_descriptor_style_count(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return 0;
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return 0;
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto styles = obj.getProperty(hermes, "styles");

    if (!styles.isObject()) {
        return 0;
    }

    return styles.getObject(hermes).getArray(hermes).size(hermes);
}

extern "C" HermesHandle vue_descriptor_template(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return 0;
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return 0;
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto tmpl = obj.getProperty(hermes, "template");

    if (tmpl.isNull() || tmpl.isUndefined()) {
        return 0;
    }

    return rt->allocate_handle(std::move(tmpl));
}

extern "C" HermesHandle vue_descriptor_script(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return 0;
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return 0;
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto script = obj.getProperty(hermes, "script");

    if (script.isNull() || script.isUndefined()) {
        return 0;
    }

    return rt->allocate_handle(std::move(script));
}

extern "C" HermesHandle vue_descriptor_script_setup(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return 0;
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return 0;
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto script = obj.getProperty(hermes, "scriptSetup");

    if (script.isNull() || script.isUndefined()) {
        return 0;
    }

    return rt->allocate_handle(std::move(script));
}

extern "C" HermesHandle vue_descriptor_style_at(HermesRuntime rt, HermesHandle handle, size_t index) {
    if (!rt) {
        return 0;
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return 0;
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto styles = obj.getProperty(hermes, "styles").getObject(hermes).getArray(hermes);

    if (index >= styles.size(hermes)) {
        return 0;
    }

    auto style = styles.getValueAtIndex(hermes, index);
    return rt->allocate_handle(std::move(style));
}

extern "C" size_t vue_descriptor_custom_blocks_count(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return 0;
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return 0;
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto customBlocks = obj.getProperty(hermes, "customBlocks");

    if (!customBlocks.isObject()) {
        return 0;
    }

    return customBlocks.getObject(hermes).getArray(hermes).size(hermes);
}

extern "C" HermesHandle vue_descriptor_custom_block_at(HermesRuntime rt, HermesHandle handle, size_t index) {
    if (!rt) {
        return 0;
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return 0;
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto customBlocks = obj.getProperty(hermes, "customBlocks").getObject(hermes).getArray(hermes);

    if (index >= customBlocks.size(hermes)) {
        return 0;
    }

    auto block = customBlocks.getValueAtIndex(hermes, index);
    return rt->allocate_handle(std::move(block));
}

extern "C" size_t vue_descriptor_css_vars_count(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return 0;
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return 0;
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto cssVars = obj.getProperty(hermes, "cssVars");

    if (!cssVars.isObject()) {
        return 0;
    }

    return cssVars.getObject(hermes).getArray(hermes).size(hermes);
}

extern "C" const char* vue_descriptor_css_var_at(HermesRuntime rt, HermesHandle handle, size_t index) {
    if (!rt) {
        return "";
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return "";
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto cssVars = obj.getProperty(hermes, "cssVars");

    if (!cssVars.isObject()) {
        return "";
    }

    auto cssVarsArr = cssVars.getObject(hermes).getArray(hermes);
    if (index >= cssVarsArr.size(hermes)) {
        return "";
    }

    auto cssVar = cssVarsArr.getValueAtIndex(hermes, index);
    if (cssVar.isString()) {
        return rt->cache_string(handle, cssVar.getString(hermes).utf8(hermes));
    }

    return "";
}

extern "C" bool vue_descriptor_slotted(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return false;
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return false;
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto slotted = obj.getProperty(hermes, "slotted");
    return slotted.isBool() && slotted.getBool();
}

extern "C" const char* vue_descriptor_source(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return "";
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return "";
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto source = obj.getProperty(hermes, "source");

    if (!source.isString()) {
        return "";
    }

    return rt->cache_string(handle, source.getString(hermes).utf8(hermes));
}

extern "C" const char* vue_descriptor_filename(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return "";
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return "";
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto filename = obj.getProperty(hermes, "filename");

    if (!filename.isString()) {
        return "";
    }

    return rt->cache_string(handle, filename.getString(hermes).utf8(hermes));
}

// ============================================================================
// Block Accessors
// ============================================================================

extern "C" const char* vue_block_content(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return "";
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return "";
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto content = obj.getProperty(hermes, "content");

    if (!content.isString()) {
        return "";
    }

    return rt->cache_string(handle, content.getString(hermes).utf8(hermes));
}

extern "C" const char* vue_block_lang(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return "";
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return "";
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto lang = obj.getProperty(hermes, "lang");

    if (!lang.isString()) {
        return "";
    }

    return rt->cache_string(handle, lang.getString(hermes).utf8(hermes));
}

extern "C" const char* vue_block_src(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return "";
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return "";
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto src = obj.getProperty(hermes, "src");

    if (!src.isString()) {
        return "";
    }

    return rt->cache_string(handle, src.getString(hermes).utf8(hermes));
}

extern "C" const char* vue_custom_block_type(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return "";
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return "";
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto type = obj.getProperty(hermes, "type");

    if (!type.isString()) {
        return "";
    }

    return rt->cache_string(handle, type.getString(hermes).utf8(hermes));
}

// ============================================================================
// Block Location Accessors
// ============================================================================

extern "C" size_t vue_block_loc_start_offset(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return 0;
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return 0;
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto loc = obj.getProperty(hermes, "loc");
    if (!loc.isObject()) {
        return 0;
    }

    auto start = loc.getObject(hermes).getProperty(hermes, "start");
    if (!start.isObject()) {
        return 0;
    }

    auto offset = start.getObject(hermes).getProperty(hermes, "offset");
    if (!offset.isNumber()) {
        return 0;
    }

    return static_cast<size_t>(offset.getNumber());
}

extern "C" size_t vue_block_loc_start_line(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return 0;
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return 0;
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto loc = obj.getProperty(hermes, "loc");
    if (!loc.isObject()) {
        return 0;
    }

    auto start = loc.getObject(hermes).getProperty(hermes, "start");
    if (!start.isObject()) {
        return 0;
    }

    auto line = start.getObject(hermes).getProperty(hermes, "line");
    if (!line.isNumber()) {
        return 0;
    }

    return static_cast<size_t>(line.getNumber());
}

extern "C" size_t vue_block_loc_start_column(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return 0;
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return 0;
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto loc = obj.getProperty(hermes, "loc");
    if (!loc.isObject()) {
        return 0;
    }

    auto start = loc.getObject(hermes).getProperty(hermes, "start");
    if (!start.isObject()) {
        return 0;
    }

    auto column = start.getObject(hermes).getProperty(hermes, "column");
    if (!column.isNumber()) {
        return 0;
    }

    return static_cast<size_t>(column.getNumber());
}

extern "C" size_t vue_block_loc_end_offset(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return 0;
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return 0;
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto loc = obj.getProperty(hermes, "loc");
    if (!loc.isObject()) {
        return 0;
    }

    auto end = loc.getObject(hermes).getProperty(hermes, "end");
    if (!end.isObject()) {
        return 0;
    }

    auto offset = end.getObject(hermes).getProperty(hermes, "offset");
    if (!offset.isNumber()) {
        return 0;
    }

    return static_cast<size_t>(offset.getNumber());
}

extern "C" size_t vue_block_loc_end_line(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return 0;
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return 0;
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto loc = obj.getProperty(hermes, "loc");
    if (!loc.isObject()) {
        return 0;
    }

    auto end = loc.getObject(hermes).getProperty(hermes, "end");
    if (!end.isObject()) {
        return 0;
    }

    auto line = end.getObject(hermes).getProperty(hermes, "line");
    if (!line.isNumber()) {
        return 0;
    }

    return static_cast<size_t>(line.getNumber());
}

extern "C" size_t vue_block_loc_end_column(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return 0;
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return 0;
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto loc = obj.getProperty(hermes, "loc");
    if (!loc.isObject()) {
        return 0;
    }

    auto end = loc.getObject(hermes).getProperty(hermes, "end");
    if (!end.isObject()) {
        return 0;
    }

    auto column = end.getObject(hermes).getProperty(hermes, "column");
    if (!column.isNumber()) {
        return 0;
    }

    return static_cast<size_t>(column.getNumber());
}

// ============================================================================
// Block Attribute Accessors
// ============================================================================

extern "C" size_t vue_block_attrs_count(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return 0;
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return 0;
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto attrs = obj.getProperty(hermes, "attrs");

    if (!attrs.isObject()) {
        return 0;
    }

    auto attrsObj = attrs.getObject(hermes);
    auto names = attrsObj.getPropertyNames(hermes);
    return names.size(hermes);
}

extern "C" const char* vue_block_attrs_key_at(HermesRuntime rt, HermesHandle handle, size_t index) {
    if (!rt) {
        return "";
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return "";
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto attrs = obj.getProperty(hermes, "attrs");

    if (!attrs.isObject()) {
        return "";
    }

    auto attrsObj = attrs.getObject(hermes);
    auto names = attrsObj.getPropertyNames(hermes);
    if (index >= names.size(hermes)) {
        return "";
    }

    auto key = names.getValueAtIndex(hermes, index).getString(hermes).utf8(hermes);
    return rt->cache_string(handle, key);
}

extern "C" const char* vue_block_attrs_value_at(HermesRuntime rt, HermesHandle handle, size_t index) {
    if (!rt) {
        return "";
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return "";
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto attrs = obj.getProperty(hermes, "attrs");

    if (!attrs.isObject()) {
        return "";
    }

    auto attrsObj = attrs.getObject(hermes);
    auto names = attrsObj.getPropertyNames(hermes);
    if (index >= names.size(hermes)) {
        return "";
    }

    auto key = names.getValueAtIndex(hermes, index).getString(hermes);
    auto value = attrsObj.getProperty(hermes, key);

    if (value.isString()) {
        return rt->cache_string(handle, value.getString(hermes).utf8(hermes));
    }

    return "";
}

extern "C" bool vue_block_attrs_is_bool_at(HermesRuntime rt, HermesHandle handle, size_t index) {
    if (!rt) {
        return false;
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return false;
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto attrs = obj.getProperty(hermes, "attrs");

    if (!attrs.isObject()) {
        return false;
    }

    auto attrsObj = attrs.getObject(hermes);
    auto names = attrsObj.getPropertyNames(hermes);
    if (index >= names.size(hermes)) {
        return false;
    }

    auto key = names.getValueAtIndex(hermes, index).getString(hermes);
    auto value = attrsObj.getProperty(hermes, key);

    return value.isBool() && value.getBool();
}

// ============================================================================
// Style Block Accessors
// ============================================================================

extern "C" bool vue_style_is_scoped(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return false;
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return false;
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto scoped = obj.getProperty(hermes, "scoped");
    return scoped.isBool() && scoped.getBool();
}

extern "C" bool vue_style_has_module(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return false;
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return false;
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto module = obj.getProperty(hermes, "module");
    return !module.isNull() && !module.isUndefined();
}

extern "C" const char* vue_style_module_value(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return "";
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return "";
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto module = obj.getProperty(hermes, "module");

    if (module.isString()) {
        return rt->cache_string(handle, module.getString(hermes).utf8(hermes));
    }

    return "";
}

// ============================================================================
// Script Block Accessors
// ============================================================================

extern "C" bool vue_script_has_setup(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return false;
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return false;
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto setup = obj.getProperty(hermes, "setup");
    return !setup.isNull() && !setup.isUndefined();
}

extern "C" const char* vue_script_setup_value(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return "";
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return "";
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto setup = obj.getProperty(hermes, "setup");

    if (setup.isString()) {
        return rt->cache_string(handle, setup.getString(hermes).utf8(hermes));
    }

    return "";
}

extern "C" size_t vue_script_bindings_count(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return 0;
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return 0;
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto bindings = obj.getProperty(hermes, "bindings");

    if (!bindings.isObject()) {
        return 0;
    }

    auto bindingsObj = bindings.getObject(hermes);
    auto names = bindingsObj.getPropertyNames(hermes);
    return names.size(hermes);
}

extern "C" const char* vue_script_bindings_key_at(HermesRuntime rt, HermesHandle handle, size_t index) {
    if (!rt) {
        return "";
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return "";
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto bindings = obj.getProperty(hermes, "bindings");

    if (!bindings.isObject()) {
        return "";
    }

    auto bindingsObj = bindings.getObject(hermes);
    auto names = bindingsObj.getPropertyNames(hermes);
    if (index >= names.size(hermes)) {
        return "";
    }

    auto key = names.getValueAtIndex(hermes, index).getString(hermes).utf8(hermes);
    return rt->cache_string(handle, key);
}

extern "C" const char* vue_script_bindings_value_at(HermesRuntime rt, HermesHandle handle, size_t index) {
    if (!rt) {
        return "";
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return "";
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto bindings = obj.getProperty(hermes, "bindings");

    if (!bindings.isObject()) {
        return "";
    }

    auto bindingsObj = bindings.getObject(hermes);
    auto names = bindingsObj.getPropertyNames(hermes);
    if (index >= names.size(hermes)) {
        return "";
    }

    auto key = names.getValueAtIndex(hermes, index).getString(hermes);
    auto value = bindingsObj.getProperty(hermes, key);

    if (value.isString()) {
        return rt->cache_string(handle, value.getString(hermes).utf8(hermes));
    }

    return "";
}

extern "C" size_t vue_script_imports_count(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return 0;
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return 0;
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto imports = obj.getProperty(hermes, "imports");

    if (!imports.isObject()) {
        return 0;
    }

    auto importsObj = imports.getObject(hermes);
    auto names = importsObj.getPropertyNames(hermes);
    return names.size(hermes);
}

extern "C" const char* vue_script_imports_key_at(HermesRuntime rt, HermesHandle handle, size_t index) {
    if (!rt) {
        return "";
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return "";
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto imports = obj.getProperty(hermes, "imports");

    if (!imports.isObject()) {
        return "";
    }

    auto importsObj = imports.getObject(hermes);
    auto names = importsObj.getPropertyNames(hermes);
    if (index >= names.size(hermes)) {
        return "";
    }

    auto key = names.getValueAtIndex(hermes, index).getString(hermes).utf8(hermes);
    return rt->cache_string(handle, key);
}

extern "C" HermesHandle vue_script_imports_value_at(HermesRuntime rt, HermesHandle handle, size_t index) {
    if (!rt) {
        return 0;
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return 0;
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto imports = obj.getProperty(hermes, "imports");

    if (!imports.isObject()) {
        return 0;
    }

    auto importsObj = imports.getObject(hermes);
    auto names = importsObj.getPropertyNames(hermes);
    if (index >= names.size(hermes)) {
        return 0;
    }

    auto key = names.getValueAtIndex(hermes, index).getString(hermes);
    auto value = importsObj.getProperty(hermes, key);

    if (!value.isObject()) {
        return 0;
    }

    return rt->allocate_handle(std::move(value));
}

extern "C" bool vue_import_binding_is_type(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return false;
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return false;
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto isType = obj.getProperty(hermes, "isType");
    return isType.isBool() && isType.getBool();
}

extern "C" const char* vue_import_binding_imported(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return "";
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return "";
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto imported = obj.getProperty(hermes, "imported");

    if (!imported.isString()) {
        return "";
    }

    return rt->cache_string(handle, imported.getString(hermes).utf8(hermes));
}

extern "C" const char* vue_import_binding_source(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return "";
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return "";
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto source = obj.getProperty(hermes, "source");

    if (!source.isString()) {
        return "";
    }

    return rt->cache_string(handle, source.getString(hermes).utf8(hermes));
}

extern "C" bool vue_import_binding_is_from_setup(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return false;
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return false;
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto isFromSetup = obj.getProperty(hermes, "isFromSetup");
    return isFromSetup.isBool() && isFromSetup.getBool();
}

extern "C" size_t vue_script_warnings_count(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return 0;
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return 0;
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto warnings = obj.getProperty(hermes, "warnings");

    if (!warnings.isObject()) {
        return 0;
    }

    return warnings.getObject(hermes).getArray(hermes).size(hermes);
}

extern "C" const char* vue_script_warning_at(HermesRuntime rt, HermesHandle handle, size_t index) {
    if (!rt) {
        return "";
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return "";
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto warnings = obj.getProperty(hermes, "warnings");

    if (!warnings.isObject()) {
        return "";
    }

    auto warningsArr = warnings.getObject(hermes).getArray(hermes);
    if (index >= warningsArr.size(hermes)) {
        return "";
    }

    auto warning = warningsArr.getValueAtIndex(hermes, index);
    if (warning.isString()) {
        return rt->cache_string(handle, warning.getString(hermes).utf8(hermes));
    }

    return "";
}

extern "C" size_t vue_script_deps_count(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return 0;
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return 0;
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto deps = obj.getProperty(hermes, "deps");

    if (!deps.isObject()) {
        return 0;
    }

    return deps.getObject(hermes).getArray(hermes).size(hermes);
}

extern "C" const char* vue_script_dep_at(HermesRuntime rt, HermesHandle handle, size_t index) {
    if (!rt) {
        return "";
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return "";
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto deps = obj.getProperty(hermes, "deps");

    if (!deps.isObject()) {
        return "";
    }

    auto depsArr = deps.getObject(hermes).getArray(hermes);
    if (index >= depsArr.size(hermes)) {
        return "";
    }

    auto dep = depsArr.getValueAtIndex(hermes, index);
    if (dep.isString()) {
        return rt->cache_string(handle, dep.getString(hermes).utf8(hermes));
    }

    return "";
}

// ============================================================================
// Script Compilation
// ============================================================================

extern "C" HermesHandle vue_compile_script(
    HermesRuntime rt,
    HermesHandle desc_handle,
    const char* id, size_t id_len,
    bool is_prod
) {
    if (!rt) {
        return 0;
    }

    auto* entry = rt->get_handle(desc_handle);
    if (!entry) {
        return 0;
    }

    auto& hermes = rt->runtime();
    auto jsId = facebook::jsi::String::createFromUtf8(
        hermes, reinterpret_cast<const uint8_t*>(id), id_len);
    auto result = rt->compile_script_fn->call(hermes, *entry->value, jsId, is_prod);

    return rt->allocate_handle(std::move(result));
}

extern "C" const char* vue_script_result_content(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return "";
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return "";
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto content = obj.getProperty(hermes, "content");

    if (!content.isString()) {
        return "";
    }

    return rt->cache_string(handle, content.getString(hermes).utf8(hermes));
}

extern "C" HermesHandle vue_script_result_bindings(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return 0;
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return 0;
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto bindings = obj.getProperty(hermes, "bindings");

    if (bindings.isNull() || bindings.isUndefined()) {
        return 0;
    }

    return rt->allocate_handle(std::move(bindings));
}

// ============================================================================
// Template Compilation
// ============================================================================

extern "C" HermesHandle vue_compile_template(
    HermesRuntime rt,
    const char* source, size_t source_len,
    const char* filename, size_t filename_len,
    const char* id, size_t id_len,
    bool scoped,
    HermesHandle bindings_handle
) {
    if (!rt) {
        return 0;
    }

    auto& hermes = rt->runtime();

    auto jsSource = facebook::jsi::String::createFromUtf8(
        hermes, reinterpret_cast<const uint8_t*>(source), source_len);
    auto jsFilename = facebook::jsi::String::createFromUtf8(
        hermes, reinterpret_cast<const uint8_t*>(filename), filename_len);
    auto jsId = facebook::jsi::String::createFromUtf8(
        hermes, reinterpret_cast<const uint8_t*>(id), id_len);

    // Handle optional bindings parameter
    facebook::jsi::Value jsBindings = facebook::jsi::Value::null();
    if (bindings_handle != 0) {
        auto* bindings_entry = rt->get_handle(bindings_handle);
        if (bindings_entry) {
            jsBindings = facebook::jsi::Value(hermes, *bindings_entry->value);
        }
    }

    auto result = rt->compile_template_fn->call(hermes, jsSource, jsFilename, jsId, scoped, jsBindings);
    return rt->allocate_handle(std::move(result));
}

extern "C" const char* vue_template_result_code(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return "";
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return "";
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto code = obj.getProperty(hermes, "code");

    if (!code.isString()) {
        return "";
    }

    return rt->cache_string(handle, code.getString(hermes).utf8(hermes));
}

extern "C" size_t vue_template_result_error_count(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return 0;
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return 0;
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto errors = obj.getProperty(hermes, "errors");

    if (!errors.isObject()) {
        return 0;
    }

    return errors.getObject(hermes).getArray(hermes).size(hermes);
}

// ============================================================================
// Style Compilation
// ============================================================================

extern "C" HermesHandle vue_compile_style(
    HermesRuntime rt,
    const char* source, size_t source_len,
    const char* filename, size_t filename_len,
    const char* id, size_t id_len,
    bool scoped
) {
    if (!rt) {
        return 0;
    }

    auto& hermes = rt->runtime();

    auto jsSource = facebook::jsi::String::createFromUtf8(
        hermes, reinterpret_cast<const uint8_t*>(source), source_len);
    auto jsFilename = facebook::jsi::String::createFromUtf8(
        hermes, reinterpret_cast<const uint8_t*>(filename), filename_len);
    auto jsId = facebook::jsi::String::createFromUtf8(
        hermes, reinterpret_cast<const uint8_t*>(id), id_len);

    auto result = rt->compile_style_fn->call(hermes, jsSource, jsFilename, jsId, scoped);
    return rt->allocate_handle(std::move(result));
}

extern "C" const char* vue_style_result_code(HermesRuntime rt, HermesHandle handle) {
    if (!rt) {
        return "";
    }

    auto* entry = rt->get_handle(handle);
    if (!entry) {
        return "";
    }

    auto& hermes = rt->runtime();
    auto obj = entry->value->getObject(hermes);
    auto code = obj.getProperty(hermes, "code");

    if (!code.isString()) {
        return "";
    }

    return rt->cache_string(handle, code.getString(hermes).utf8(hermes));
}
