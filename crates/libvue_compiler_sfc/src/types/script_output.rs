//! Script compilation output type.

use super::handle::Handle;
use crate::ffi::{self, HermesHandle};
use crate::util::ptr_to_str;

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
                *self.0.runtime(),
                self.0.raw(),
            ))
        }
    }

    /// Get the internal bindings handle for template compilation.
    pub(crate) fn bindings_handle(&self) -> HermesHandle {
        unsafe { ffi::vue_script_result_bindings(*self.0.runtime(), self.0.raw()) }
    }
}
