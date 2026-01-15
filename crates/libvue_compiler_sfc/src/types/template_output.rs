//! Template compilation output type.

use super::handle::Handle;
use crate::ffi::{self, HermesHandle, HermesRuntime};
use crate::util::ptr_to_str;

/// Output of compiling a template.
pub struct TemplateOutput<'c>(Handle<'c>);

impl<'c> TemplateOutput<'c> {
    pub(crate) fn from_raw(handle: HermesHandle, runtime: &'c HermesRuntime) -> Self {
        TemplateOutput(Handle::new(handle, runtime).expect("Invalid handle"))
    }

    /// Get the compiled render function code.
    pub fn code(&self) -> &str {
        unsafe {
            ptr_to_str(ffi::vue_template_result_code(
                *self.0.runtime(),
                self.0.raw(),
            ))
        }
    }

    /// Get the number of compilation errors.
    pub fn error_count(&self) -> usize {
        unsafe { ffi::vue_template_result_error_count(*self.0.runtime(), self.0.raw()) }
    }

    /// Check if compilation produced errors.
    pub fn has_errors(&self) -> bool {
        self.error_count() > 0
    }
}
