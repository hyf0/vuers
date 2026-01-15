//! Opaque handle type for JavaScript objects.

use std::num::NonZeroU64;
use crate::ffi::{self, HermesHandle};
use super::compiler::Compiler;

/// Opaque handle to a JavaScript object.
///
/// Handles are automatically freed when dropped. The handle ensures
/// that the underlying JS object stays alive and that any strings
/// extracted from it remain valid.
///
/// The lifetime `'c` ties this handle to its parent `Compiler`,
/// ensuring the handle cannot outlive the compiler that created it.
pub struct Handle<'c> {
    raw: NonZeroU64,
    compiler: &'c Compiler,
}

impl<'c> Handle<'c> {
    /// Create a handle from a raw FFI handle.
    /// Returns None if the handle is invalid (0).
    pub(crate) fn new(raw: HermesHandle, compiler: &'c Compiler) -> Option<Self> {
        NonZeroU64::new(raw.0).map(|raw| Handle { raw, compiler })
    }

    /// Get the raw handle value for FFI calls.
    pub(crate) fn raw(&self) -> HermesHandle {
        HermesHandle(self.raw.get())
    }

    /// Get a reference to the compiler.
    pub(crate) fn compiler(&self) -> &'c Compiler {
        self.compiler
    }
}

impl Drop for Handle<'_> {
    fn drop(&mut self) {
        unsafe {
            ffi::hermes_handle_free(self.compiler.runtime, HermesHandle(self.raw.get()))
        }
    }
}
