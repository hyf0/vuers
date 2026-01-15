//! Opaque handle type for JavaScript objects.

use crate::ffi::{self, HermesHandle, HermesRuntime};
use std::num::NonZeroU64;

/// Opaque handle to a JavaScript object.
///
/// Handles are automatically freed when dropped. The handle ensures
/// that the underlying JS object stays alive and that any strings
/// extracted from it remain valid.
///
/// The lifetime `'c` ties this handle to its parent runtime,
/// ensuring the handle cannot outlive the runtime that created it.
pub struct Handle<'c> {
    raw: NonZeroU64,
    runtime: &'c HermesRuntime,
}

impl<'c> Handle<'c> {
    /// Create a handle from a raw FFI handle.
    /// Returns None if the handle is invalid (0).
    pub(crate) fn new(raw: HermesHandle, runtime: &'c HermesRuntime) -> Option<Self> {
        NonZeroU64::new(raw.0).map(|raw| Handle { raw, runtime })
    }

    /// Get the raw handle value for FFI calls.
    pub(crate) fn raw(&self) -> HermesHandle {
        HermesHandle(self.raw.get())
    }

    /// Get a reference to the runtime.
    pub(crate) fn runtime(&self) -> &'c HermesRuntime {
        self.runtime
    }
}

impl Drop for Handle<'_> {
    fn drop(&mut self) {
        unsafe { ffi::hermes_handle_free(*self.runtime, HermesHandle(self.raw.get())) }
    }
}
