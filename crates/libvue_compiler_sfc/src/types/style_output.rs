//! Style compilation output type.

use super::handle::Handle;
use crate::ffi::{self, HermesHandle, HermesRuntime};
use crate::util::ptr_to_str;

/// Output of compiling a style block.
pub struct StyleOutput<'c>(Handle<'c>);

impl<'c> StyleOutput<'c> {
    pub(crate) fn from_raw(handle: HermesHandle, runtime: &'c HermesRuntime) -> Self {
        StyleOutput(Handle::new(handle, runtime).expect("Invalid handle"))
    }

    /// Get the compiled CSS.
    pub fn code(&self) -> &str {
        unsafe { ptr_to_str(ffi::vue_style_result_code(*self.0.runtime(), self.0.raw())) }
    }
}
