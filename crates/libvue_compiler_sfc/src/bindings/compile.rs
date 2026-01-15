//! Compilation result types.

use super::compiler::Compiler;
use super::handle::Handle;
use super::util::ptr_to_str;
use crate::ffi::{self, HermesHandle};

/// Output of compiling script blocks.
pub struct ScriptOutput<'c>(Handle<'c>);

impl<'c> ScriptOutput<'c> {
    pub(crate) fn from_handle(handle: Handle<'c>) -> Self {
        ScriptOutput(handle)
    }

    /// Get the compiled script content.
    pub fn content(&self) -> &str {
        unsafe {
            ptr_to_str(ffi::vue_script_result_content(
                self.0.compiler().runtime,
                self.0.raw(),
            ))
        }
    }

    /// Get the internal bindings handle for template compilation.
    pub(crate) fn bindings_handle(&self) -> HermesHandle {
        unsafe { ffi::vue_script_result_bindings(self.0.compiler().runtime, self.0.raw()) }
    }
}

/// Output of compiling a template.
pub struct TemplateOutput<'c>(Handle<'c>);

impl<'c> TemplateOutput<'c> {
    pub(crate) fn from_raw(handle: HermesHandle, compiler: &'c Compiler) -> Self {
        TemplateOutput(Handle::new(handle, compiler).expect("Invalid handle"))
    }

    /// Get the compiled render function code.
    pub fn code(&self) -> &str {
        unsafe {
            ptr_to_str(ffi::vue_template_result_code(
                self.0.compiler().runtime,
                self.0.raw(),
            ))
        }
    }

    /// Get the number of compilation errors.
    pub fn error_count(&self) -> usize {
        unsafe { ffi::vue_template_result_error_count(self.0.compiler().runtime, self.0.raw()) }
    }

    /// Check if compilation produced errors.
    pub fn has_errors(&self) -> bool {
        self.error_count() > 0
    }
}

/// Output of compiling a style block.
pub struct StyleOutput<'c>(Handle<'c>);

impl<'c> StyleOutput<'c> {
    pub(crate) fn from_raw(handle: HermesHandle, compiler: &'c Compiler) -> Self {
        StyleOutput(Handle::new(handle, compiler).expect("Invalid handle"))
    }

    /// Get the compiled CSS.
    pub fn code(&self) -> &str {
        unsafe {
            ptr_to_str(ffi::vue_style_result_code(
                self.0.compiler().runtime,
                self.0.raw(),
            ))
        }
    }
}
