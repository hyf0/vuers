//! Compilation result types and functions.

use std::os::raw::c_char;

use crate::ffi;
use super::handle::Handle;
use super::error::{Error, Result};
use super::util::ptr_to_str;

/// Output of compiling script blocks.
pub struct ScriptOutput(Handle);

impl ScriptOutput {
    pub(crate) fn from_handle(handle: Handle) -> Self {
        ScriptOutput(handle)
    }

    /// Get the compiled script content.
    pub fn content(&self) -> &str {
        unsafe { ptr_to_str(ffi::vue_script_result_content(self.0.raw())) }
    }

    /// Get the internal bindings handle for template compilation.
    pub(crate) fn bindings_handle(&self) -> ffi::RawHandle {
        unsafe { ffi::vue_script_result_bindings(self.0.raw()) }
    }
}

/// Output of compiling a template.
pub struct TemplateOutput(Handle);

impl TemplateOutput {
    /// Get the compiled render function code.
    pub fn code(&self) -> &str {
        unsafe { ptr_to_str(ffi::vue_template_result_code(self.0.raw())) }
    }

    /// Get the number of compilation errors.
    pub fn error_count(&self) -> usize {
        unsafe { ffi::vue_template_result_error_count(self.0.raw()) }
    }

    /// Check if compilation produced errors.
    pub fn has_errors(&self) -> bool {
        self.error_count() > 0
    }
}

/// Output of compiling a style block.
pub struct StyleOutput(Handle);

impl StyleOutput {
    /// Get the compiled CSS.
    pub fn code(&self) -> &str {
        unsafe { ptr_to_str(ffi::vue_style_result_code(self.0.raw())) }
    }
}

/// Compile a template to a render function.
///
/// # Arguments
/// * `source` - Template source code
/// * `filename` - Source filename for error messages
/// * `id` - Scope ID for scoped styles
/// * `scoped` - Whether to add scoped style attributes
/// * `script` - Optional script output for binding metadata
pub fn compile_template(
    source: &str,
    filename: &str,
    id: &str,
    scoped: bool,
    script: Option<&ScriptOutput>,
) -> Result<TemplateOutput> {
    let bindings_handle = script.map(|s| s.bindings_handle()).unwrap_or(ffi::RawHandle::INVALID);

    let handle = unsafe {
        ffi::vue_compile_template(
            source.as_ptr() as *const c_char,
            source.len(),
            filename.as_ptr() as *const c_char,
            filename.len(),
            id.as_ptr() as *const c_char,
            id.len(),
            scoped,
            bindings_handle,
        )
    };

    Handle::new(handle)
        .map(TemplateOutput)
        .ok_or_else(|| Error::new("compile_template returned invalid handle"))
}

/// Compile a style block.
///
/// # Arguments
/// * `source` - Style source code
/// * `filename` - Source filename for error messages
/// * `id` - Scope ID for scoped styles
/// * `scoped` - Whether to scope the styles
pub fn compile_style(
    source: &str,
    filename: &str,
    id: &str,
    scoped: bool,
) -> Result<StyleOutput> {
    let handle = unsafe {
        ffi::vue_compile_style(
            source.as_ptr() as *const c_char,
            source.len(),
            filename.as_ptr() as *const c_char,
            filename.len(),
            id.as_ptr() as *const c_char,
            id.len(),
            scoped,
        )
    };

    Handle::new(handle)
        .map(StyleOutput)
        .ok_or_else(|| Error::new("compile_style returned invalid handle"))
}
