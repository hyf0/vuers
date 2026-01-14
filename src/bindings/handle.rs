//! Opaque handle type for JavaScript objects.

use std::num::NonZeroU64;
use crate::ffi::RawHandle;

/// Opaque handle to a JavaScript object.
///
/// Handles are automatically freed when dropped. The handle ensures
/// that the underlying JS object stays alive and that any strings
/// extracted from it remain valid.
#[repr(transparent)]
pub struct Handle(NonZeroU64);

impl Handle {
    /// Create a handle from a raw FFI handle.
    /// Returns None if the handle is invalid (0).
    pub(crate) fn new(raw: RawHandle) -> Option<Self> {
        NonZeroU64::new(raw.0).map(Handle)
    }

    /// Get the raw handle value for FFI calls.
    pub(crate) fn raw(&self) -> RawHandle {
        RawHandle(self.0.get())
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe { crate::ffi::vue_handle_free(RawHandle(self.0.get())) }
    }
}
