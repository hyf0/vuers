//! Parse output type for SFC parsing.

use super::descriptor::Descriptor;
use super::handle::Handle;
use crate::ffi::{self, HermesHandle, HermesRuntime};
use crate::util::ptr_to_str;

/// Output of parsing an SFC source file.
pub struct ParseOutput<'c>(Handle<'c>);

impl<'c> ParseOutput<'c> {
    /// Create from a raw handle.
    pub(crate) fn from_raw(handle: HermesHandle, runtime: &'c HermesRuntime) -> Self {
        ParseOutput(Handle::new(handle, runtime).expect("Invalid handle"))
    }

    /// Get the SFC descriptor containing all parsed blocks.
    pub fn descriptor(&self) -> Option<Descriptor<'c>> {
        let handle = unsafe { ffi::vue_parse_result_descriptor(*self.0.runtime(), self.0.raw()) };
        Handle::new(handle, self.0.runtime()).map(Descriptor::from_handle)
    }

    /// Get the number of parse errors.
    pub fn error_count(&self) -> usize {
        unsafe { ffi::vue_parse_result_error_count(*self.0.runtime(), self.0.raw()) }
    }

    /// Get an error message by index.
    pub fn error_message(&self, index: usize) -> &str {
        unsafe {
            ptr_to_str(ffi::vue_parse_result_error_message(
                *self.0.runtime(),
                self.0.raw(),
                index,
            ))
        }
    }

    /// Check if parsing produced errors.
    pub fn has_errors(&self) -> bool {
        self.error_count() > 0 || self.descriptor().is_none()
    }

    /// Iterate over all error messages.
    pub fn errors(&self) -> impl Iterator<Item = &str> {
        (0..self.error_count()).map(move |i| self.error_message(i))
    }
}
