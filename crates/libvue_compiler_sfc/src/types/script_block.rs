//! Script block type for SFC parsing.

use std::collections::HashMap;

use super::attr_value::AttrValue;
use super::custom_block::{get_block_attrs, get_block_loc};
use super::handle::Handle;
use super::import_binding::ImportBinding;
use super::source_location::SourceLocation;
use crate::ffi;
use crate::util::ptr_to_str;

/// Script block from an SFC.
pub struct ScriptBlock<'c>(pub(crate) Handle<'c>);

impl<'c> ScriptBlock<'c> {
    pub(crate) fn from_handle(handle: Handle<'c>) -> Self {
        ScriptBlock(handle)
    }
}

impl ScriptBlock<'_> {
    /// Get the script content.
    pub fn content(&self) -> &str {
        unsafe { ptr_to_str(ffi::vue_block_content(*self.0.runtime(), self.0.raw())) }
    }

    /// Get the script language (e.g., "ts").
    pub fn lang(&self) -> &str {
        unsafe { ptr_to_str(ffi::vue_block_lang(*self.0.runtime(), self.0.raw())) }
    }

    /// Get the src attribute if present (external file reference).
    pub fn src(&self) -> Option<&str> {
        let s = unsafe { ptr_to_str(ffi::vue_block_src(*self.0.runtime(), self.0.raw())) };
        if s.is_empty() {
            None
        } else {
            Some(s)
        }
    }

    /// Get the source location of the block.
    pub fn loc(&self) -> SourceLocation {
        get_block_loc(*self.0.runtime(), self.0.raw())
    }

    /// Get all attributes on the block.
    pub fn attrs(&self) -> HashMap<String, AttrValue> {
        get_block_attrs(*self.0.runtime(), self.0.raw())
    }

    /// Check if this is a setup script block.
    pub fn is_setup(&self) -> bool {
        unsafe { ffi::vue_script_has_setup(*self.0.runtime(), self.0.raw()) }
    }

    /// Get the setup attribute value if it's a string.
    /// Returns None if setup is boolean true or not present.
    pub fn setup_value(&self) -> Option<&str> {
        if !self.is_setup() {
            return None;
        }
        let s = unsafe { ptr_to_str(ffi::vue_script_setup_value(*self.0.runtime(), self.0.raw())) };
        if s.is_empty() {
            None
        } else {
            Some(s)
        }
    }

    /// Get the number of bindings in the script block.
    pub fn bindings_count(&self) -> usize {
        unsafe { ffi::vue_script_bindings_count(*self.0.runtime(), self.0.raw()) }
    }

    /// Get the variable bindings as a map of variable name to binding type.
    ///
    /// Binding types are strings like "setup-const", "setup-ref", "setup-maybe-ref",
    /// "setup-reactive-const", "props", "props-aliased", "data", "options", etc.
    pub fn bindings(&self) -> HashMap<String, String> {
        let count = self.bindings_count();
        let mut bindings = HashMap::with_capacity(count);

        for i in 0..count {
            let key = unsafe {
                ptr_to_str(ffi::vue_script_bindings_key_at(
                    *self.0.runtime(),
                    self.0.raw(),
                    i,
                ))
                .to_string()
            };
            let value = unsafe {
                ptr_to_str(ffi::vue_script_bindings_value_at(
                    *self.0.runtime(),
                    self.0.raw(),
                    i,
                ))
                .to_string()
            };
            bindings.insert(key, value);
        }

        bindings
    }

    /// Get the number of imports in the script block.
    pub fn imports_count(&self) -> usize {
        unsafe { ffi::vue_script_imports_count(*self.0.runtime(), self.0.raw()) }
    }

    /// Get the import bindings as a map of local name to import metadata.
    pub fn imports(&self) -> HashMap<String, ImportBinding> {
        let count = self.imports_count();
        let mut imports = HashMap::with_capacity(count);

        let rt = *self.0.runtime();

        for i in 0..count {
            let key = unsafe {
                ptr_to_str(ffi::vue_script_imports_key_at(rt, self.0.raw(), i)).to_string()
            };
            let handle = unsafe { ffi::vue_script_imports_value_at(rt, self.0.raw(), i) };

            if handle.is_valid() {
                // Extract data directly from the handle
                let is_type = unsafe { ffi::vue_import_binding_is_type(rt, handle) };
                let imported =
                    unsafe { ptr_to_str(ffi::vue_import_binding_imported(rt, handle)).to_string() };
                let source =
                    unsafe { ptr_to_str(ffi::vue_import_binding_source(rt, handle)).to_string() };
                let is_from_setup = unsafe { ffi::vue_import_binding_is_from_setup(rt, handle) };

                let binding = ImportBinding {
                    is_type,
                    imported,
                    source,
                    is_from_setup,
                };
                imports.insert(key, binding);

                // Free the handle
                unsafe { ffi::hermes_handle_free(rt, handle) };
            }
        }

        imports
    }

    /// Get the number of warnings in the script block.
    pub fn warnings_count(&self) -> usize {
        unsafe { ffi::vue_script_warnings_count(*self.0.runtime(), self.0.raw()) }
    }

    /// Get the warnings from the script block.
    pub fn warnings(&self) -> Vec<String> {
        let count = self.warnings_count();
        let mut warnings = Vec::with_capacity(count);

        for i in 0..count {
            let warning = unsafe {
                ptr_to_str(ffi::vue_script_warning_at(
                    *self.0.runtime(),
                    self.0.raw(),
                    i,
                ))
                .to_string()
            };
            warnings.push(warning);
        }

        warnings
    }

    /// Get the number of dependencies in the script block.
    pub fn deps_count(&self) -> usize {
        unsafe { ffi::vue_script_deps_count(*self.0.runtime(), self.0.raw()) }
    }

    /// Get the dependencies (imported modules) from the script block.
    pub fn deps(&self) -> Vec<String> {
        let count = self.deps_count();
        let mut deps = Vec::with_capacity(count);

        for i in 0..count {
            let dep = unsafe {
                ptr_to_str(ffi::vue_script_dep_at(*self.0.runtime(), self.0.raw(), i)).to_string()
            };
            deps.push(dep);
        }

        deps
    }
}
