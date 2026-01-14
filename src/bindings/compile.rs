//! Compilation result types and functions.

use std::ffi::CString;

use crate::ffi;
use super::handle::Handle;
use super::error::{Error, Result};
use super::util::ptr_to_str;

/// Binding metadata from script compilation.
///
/// Used to optimize template compilation by providing type information
/// about variables defined in the script.
pub struct Bindings(Handle);

impl Bindings {
    pub(crate) fn raw(&self) -> ffi::RawHandle {
        self.0.raw()
    }
}

/// Result of compiling script blocks.
pub struct ScriptResult(Handle);

impl ScriptResult {
    pub(crate) fn from_handle(handle: Handle) -> Self {
        ScriptResult(handle)
    }

    /// Get the compiled script content.
    pub fn content(&self) -> &str {
        unsafe { ptr_to_str(ffi::vue_script_result_content(self.0.raw())) }
    }

    /// Get the binding metadata for template optimization.
    pub fn bindings(&self) -> Option<Bindings> {
        let handle = unsafe { ffi::vue_script_result_bindings(self.0.raw()) };
        Handle::new(handle).map(Bindings)
    }
}

/// Result of compiling a template.
pub struct TemplateResult(Handle);

impl TemplateResult {
    /// Get the compiled render function code.
    pub fn code(&self) -> &str {
        unsafe { ptr_to_str(ffi::vue_template_result_code(self.0.raw())) }
    }

    /// Get the number of compilation errors.
    pub fn error_count(&self) -> usize {
        unsafe { ffi::vue_template_result_error_count(self.0.raw()) }
    }

    /// Check if compilation succeeded.
    pub fn is_ok(&self) -> bool {
        self.error_count() == 0
    }
}

/// Result of compiling a style block.
pub struct StyleResult(Handle);

impl StyleResult {
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
/// * `bindings` - Optional binding metadata from script compilation
pub fn compile_template(
    source: &str,
    filename: &str,
    id: &str,
    scoped: bool,
    bindings: Option<&Bindings>,
) -> Result<TemplateResult> {
    let source_c = CString::new(source)?;
    let filename_c = CString::new(filename)?;
    let id_c = CString::new(id)?;
    let bindings_handle = bindings.map(|b| b.raw()).unwrap_or(ffi::RawHandle::INVALID);

    let handle = unsafe {
        ffi::vue_compile_template(
            source_c.as_ptr(),
            filename_c.as_ptr(),
            id_c.as_ptr(),
            scoped,
            bindings_handle,
        )
    };

    Handle::new(handle)
        .map(TemplateResult)
        .ok_or_else(|| Error("compile_template returned invalid handle".into()))
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
) -> Result<StyleResult> {
    let source_c = CString::new(source)?;
    let filename_c = CString::new(filename)?;
    let id_c = CString::new(id)?;

    let handle = unsafe {
        ffi::vue_compile_style(
            source_c.as_ptr(),
            filename_c.as_ptr(),
            id_c.as_ptr(),
            scoped,
        )
    };

    Handle::new(handle)
        .map(StyleResult)
        .ok_or_else(|| Error("compile_style returned invalid handle".into()))
}
