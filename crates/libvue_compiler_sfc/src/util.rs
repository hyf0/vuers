//! Internal utility functions.

use std::ffi::CStr;
use std::os::raw::c_char;

/// Convert a C string pointer to a Rust &str.
///
/// # Safety
/// The pointer must be valid and point to a null-terminated UTF-8 string.
/// The string must remain valid for the lifetime 'a.
pub(crate) unsafe fn ptr_to_str<'a>(ptr: *const c_char) -> &'a str {
    if ptr.is_null() {
        ""
    } else {
        CStr::from_ptr(ptr).to_str().unwrap_or("")
    }
}
