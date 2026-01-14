//! Raw extern "C" function declarations for the Vue SFC compiler FFI.
//!
//! This module provides direct bindings to the C++ wrapper functions defined in
//! `ffi/wrapper.cpp`. All functions operate on opaque handles and require careful
//! attention to memory management.
//!
//! # Memory Model
//!
//! - **Handles**: Opaque 64-bit identifiers for JS objects. Must be freed with
//!   [`vue_handle_free`] when no longer needed.
//! - **Strings**: Returned `*const c_char` pointers are owned by their parent handle
//!   and remain valid until that handle is freed. Do not free these pointers directly.
//!
//! # Thread Safety
//!
//! None of these functions are thread-safe. The Hermes runtime uses global state
//! and must only be accessed from a single thread.

use std::os::raw::c_char;

// ============================================================================
// Handle Type
// ============================================================================

/// Opaque handle to a JavaScript object in the Hermes runtime.
///
/// Handles are 64-bit identifiers that reference entries in an internal handle table
/// managed by the C++ wrapper. They provide a way to pass JS objects across the FFI
/// boundary without exposing internal pointers.
///
/// # Representation
///
/// - `0` represents an invalid/null handle
/// - Non-zero values are 1-indexed into the handle table
///
/// # Lifetime
///
/// Handles must be explicitly freed with [`vue_handle_free`] when no longer needed.
/// Failing to free handles will leak memory. Strings returned by handle accessor
/// functions (e.g., [`vue_block_content`]) are owned by the handle and become invalid
/// when the handle is freed.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RawHandle(pub u64);

impl RawHandle {
    /// The invalid/null handle value.
    ///
    /// This is returned by functions when an operation fails or a value doesn't exist.
    pub const INVALID: Self = RawHandle(0);

    /// Returns `true` if this handle is valid (non-null).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let handle = unsafe { vue_parse(source, filename) };
    /// if handle.is_valid() {
    ///     // handle can be used
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn is_valid(self) -> bool {
        self.0 != 0
    }
}

impl Default for RawHandle {
    fn default() -> Self {
        Self::INVALID
    }
}

impl From<u64> for RawHandle {
    fn from(value: u64) -> Self {
        RawHandle(value)
    }
}

impl From<RawHandle> for u64 {
    fn from(handle: RawHandle) -> Self {
        handle.0
    }
}

// ============================================================================
// FFI Function Declarations
// ============================================================================

extern "C" {
    // ------------------------------------------------------------------------
    // Handle Management
    // ------------------------------------------------------------------------

    /// Frees a handle and releases its associated resources.
    ///
    /// After calling this function, the handle becomes invalid and must not be used.
    /// Any strings that were returned by accessor functions for this handle also
    /// become invalid.
    ///
    /// # Safety
    ///
    /// - `handle` should be a valid handle previously returned by an FFI function,
    ///   or `RawHandle::INVALID` (which is a no-op).
    /// - The handle must not be used after this call.
    /// - This function is idempotent for invalid handles.
    /// - Must be called from the same thread that created the handle.
    pub fn vue_handle_free(handle: RawHandle);

    // ------------------------------------------------------------------------
    // Parsing
    // ------------------------------------------------------------------------

    /// Parses a Vue Single File Component source string.
    ///
    /// Returns a handle to a `ParseResult` object containing the parsed descriptor
    /// and any parse errors.
    ///
    /// # Safety
    ///
    /// - `source` must be a valid, null-terminated UTF-8 string.
    /// - `filename` must be a valid, null-terminated UTF-8 string.
    /// - Both pointers must remain valid for the duration of the call.
    /// - Must be called from the main thread.
    ///
    /// # Returns
    ///
    /// A valid handle on success. The handle must be freed with [`vue_handle_free`].
    /// Even if parsing fails, a valid handle is returned containing error information.
    #[must_use]
    pub fn vue_parse(source: *const c_char, filename: *const c_char) -> RawHandle;

    /// Gets the descriptor handle from a parse result.
    ///
    /// The descriptor contains information about the SFC's template, script, and style blocks.
    ///
    /// # Safety
    ///
    /// - `handle` must be a valid `ParseResult` handle from [`vue_parse`].
    /// - Must be called from the main thread.
    ///
    /// # Returns
    ///
    /// A valid handle to the descriptor, or `RawHandle::INVALID` if the parse failed.
    /// The returned handle must be freed separately from the parse result handle.
    #[must_use]
    pub fn vue_parse_result_descriptor(handle: RawHandle) -> RawHandle;

    /// Gets the number of parse errors.
    ///
    /// # Safety
    ///
    /// - `handle` must be a valid `ParseResult` handle from [`vue_parse`].
    /// - Must be called from the main thread.
    #[must_use]
    pub fn vue_parse_result_error_count(handle: RawHandle) -> usize;

    /// Gets an error message at the specified index.
    ///
    /// # Safety
    ///
    /// - `handle` must be a valid `ParseResult` handle from [`vue_parse`].
    /// - `index` must be less than the value returned by [`vue_parse_result_error_count`].
    /// - Must be called from the main thread.
    ///
    /// # Returns
    ///
    /// A pointer to a null-terminated UTF-8 string owned by the handle.
    /// The string remains valid until the handle is freed.
    /// Returns an empty string if the handle or index is invalid.
    #[must_use]
    pub fn vue_parse_result_error_message(handle: RawHandle, index: usize) -> *const c_char;

    // ------------------------------------------------------------------------
    // Descriptor Accessors
    // ------------------------------------------------------------------------

    /// Checks if the descriptor has a `<template>` block.
    ///
    /// # Safety
    ///
    /// - `handle` must be a valid descriptor handle from [`vue_parse_result_descriptor`].
    /// - Must be called from the main thread.
    #[must_use]
    pub fn vue_descriptor_has_template(handle: RawHandle) -> bool;

    /// Checks if the descriptor has a `<script>` block (not setup).
    ///
    /// # Safety
    ///
    /// - `handle` must be a valid descriptor handle from [`vue_parse_result_descriptor`].
    /// - Must be called from the main thread.
    #[must_use]
    pub fn vue_descriptor_has_script(handle: RawHandle) -> bool;

    /// Checks if the descriptor has a `<script setup>` block.
    ///
    /// # Safety
    ///
    /// - `handle` must be a valid descriptor handle from [`vue_parse_result_descriptor`].
    /// - Must be called from the main thread.
    #[must_use]
    pub fn vue_descriptor_has_script_setup(handle: RawHandle) -> bool;

    /// Gets the number of `<style>` blocks in the component.
    ///
    /// # Safety
    ///
    /// - `handle` must be a valid descriptor handle from [`vue_parse_result_descriptor`].
    /// - Must be called from the main thread.
    #[must_use]
    pub fn vue_descriptor_style_count(handle: RawHandle) -> usize;

    /// Gets the template block handle.
    ///
    /// # Safety
    ///
    /// - `handle` must be a valid descriptor handle from [`vue_parse_result_descriptor`].
    /// - Must be called from the main thread.
    ///
    /// # Returns
    ///
    /// A handle to the template block, or `RawHandle::INVALID` if no template exists.
    /// The returned handle must be freed separately.
    #[must_use]
    pub fn vue_descriptor_template(handle: RawHandle) -> RawHandle;

    /// Gets the script setup block handle.
    ///
    /// # Safety
    ///
    /// - `handle` must be a valid descriptor handle from [`vue_parse_result_descriptor`].
    /// - Must be called from the main thread.
    ///
    /// # Returns
    ///
    /// A handle to the script setup block, or `RawHandle::INVALID` if none exists.
    /// The returned handle must be freed separately.
    #[must_use]
    pub fn vue_descriptor_script_setup(handle: RawHandle) -> RawHandle;

    /// Gets a style block handle by index.
    ///
    /// # Safety
    ///
    /// - `handle` must be a valid descriptor handle from [`vue_parse_result_descriptor`].
    /// - `index` must be less than the value returned by [`vue_descriptor_style_count`].
    /// - Must be called from the main thread.
    ///
    /// # Returns
    ///
    /// A handle to the style block, or `RawHandle::INVALID` if the index is out of bounds.
    /// The returned handle must be freed separately.
    #[must_use]
    pub fn vue_descriptor_style_at(handle: RawHandle, index: usize) -> RawHandle;

    // ------------------------------------------------------------------------
    // Block Accessors
    // ------------------------------------------------------------------------

    /// Gets the content of a block (template, script, or style).
    ///
    /// # Safety
    ///
    /// - `handle` must be a valid block handle (template, script, or style).
    /// - Must be called from the main thread.
    ///
    /// # Returns
    ///
    /// A pointer to the block's content as a null-terminated UTF-8 string.
    /// The string is owned by the handle and remains valid until the handle is freed.
    /// Returns an empty string if the handle is invalid.
    #[must_use]
    pub fn vue_block_content(handle: RawHandle) -> *const c_char;

    /// Gets the `lang` attribute of a block.
    ///
    /// # Safety
    ///
    /// - `handle` must be a valid block handle (template, script, or style).
    /// - Must be called from the main thread.
    ///
    /// # Returns
    ///
    /// A pointer to the lang attribute as a null-terminated UTF-8 string.
    /// Returns an empty string if no lang attribute is specified or handle is invalid.
    /// The string is owned by the handle and remains valid until the handle is freed.
    #[must_use]
    pub fn vue_block_lang(handle: RawHandle) -> *const c_char;

    /// Checks if a style block has the `scoped` attribute.
    ///
    /// # Safety
    ///
    /// - `handle` must be a valid style block handle from [`vue_descriptor_style_at`].
    /// - Must be called from the main thread.
    #[must_use]
    pub fn vue_style_is_scoped(handle: RawHandle) -> bool;

    // ------------------------------------------------------------------------
    // Script Compilation
    // ------------------------------------------------------------------------

    /// Compiles the script blocks of an SFC descriptor.
    ///
    /// This processes both `<script>` and `<script setup>` blocks, combining them
    /// into a single compiled output with binding metadata.
    ///
    /// # Safety
    ///
    /// - `descriptor` must be a valid descriptor handle from [`vue_parse_result_descriptor`].
    /// - `id` must be a valid, null-terminated UTF-8 string (scope ID for the component).
    /// - Both pointers must remain valid for the duration of the call.
    /// - Must be called from the main thread.
    ///
    /// # Returns
    ///
    /// A handle to the script compilation result. Must be freed with [`vue_handle_free`].
    #[must_use]
    pub fn vue_compile_script(
        descriptor: RawHandle,
        id: *const c_char,
        is_prod: bool,
    ) -> RawHandle;

    /// Gets the compiled script content.
    ///
    /// # Safety
    ///
    /// - `handle` must be a valid script result handle from [`vue_compile_script`].
    /// - Must be called from the main thread.
    ///
    /// # Returns
    ///
    /// A pointer to the compiled JavaScript code as a null-terminated UTF-8 string.
    /// The string is owned by the handle and remains valid until the handle is freed.
    #[must_use]
    pub fn vue_script_result_content(handle: RawHandle) -> *const c_char;

    /// Gets the bindings handle from a script compilation result.
    ///
    /// Bindings contain metadata about variables defined in the script, which is
    /// used by the template compiler for optimization.
    ///
    /// # Safety
    ///
    /// - `handle` must be a valid script result handle from [`vue_compile_script`].
    /// - Must be called from the main thread.
    ///
    /// # Returns
    ///
    /// A handle to the bindings object, or `RawHandle::INVALID` if no bindings exist.
    /// The returned handle must be freed separately.
    #[must_use]
    pub fn vue_script_result_bindings(handle: RawHandle) -> RawHandle;

    // ------------------------------------------------------------------------
    // Template Compilation
    // ------------------------------------------------------------------------

    /// Compiles a Vue template to a render function.
    ///
    /// # Safety
    ///
    /// - `source` must be a valid, null-terminated UTF-8 string containing the template.
    /// - `filename` must be a valid, null-terminated UTF-8 string.
    /// - `id` must be a valid, null-terminated UTF-8 string (scope ID).
    /// - `bindings` may be `RawHandle::INVALID` if no bindings are available.
    /// - All string pointers must remain valid for the duration of the call.
    /// - Must be called from the main thread.
    ///
    /// # Returns
    ///
    /// A handle to the template compilation result. Must be freed with [`vue_handle_free`].
    #[must_use]
    pub fn vue_compile_template(
        source: *const c_char,
        filename: *const c_char,
        id: *const c_char,
        scoped: bool,
        bindings: RawHandle,
    ) -> RawHandle;

    /// Gets the compiled template code (render function).
    ///
    /// # Safety
    ///
    /// - `handle` must be a valid template result handle from [`vue_compile_template`].
    /// - Must be called from the main thread.
    ///
    /// # Returns
    ///
    /// A pointer to the compiled render function code as a null-terminated UTF-8 string.
    /// The string is owned by the handle and remains valid until the handle is freed.
    #[must_use]
    pub fn vue_template_result_code(handle: RawHandle) -> *const c_char;

    /// Gets the number of template compilation errors.
    ///
    /// # Safety
    ///
    /// - `handle` must be a valid template result handle from [`vue_compile_template`].
    /// - Must be called from the main thread.
    #[must_use]
    pub fn vue_template_result_error_count(handle: RawHandle) -> usize;

    // ------------------------------------------------------------------------
    // Style Compilation
    // ------------------------------------------------------------------------

    /// Compiles a CSS style block, optionally adding scoped attribute selectors.
    ///
    /// # Safety
    ///
    /// - `source` must be a valid, null-terminated UTF-8 string containing CSS.
    /// - `filename` must be a valid, null-terminated UTF-8 string.
    /// - `id` must be a valid, null-terminated UTF-8 string (scope ID).
    /// - All string pointers must remain valid for the duration of the call.
    /// - Must be called from the main thread.
    ///
    /// # Returns
    ///
    /// A handle to the style compilation result. Must be freed with [`vue_handle_free`].
    #[must_use]
    pub fn vue_compile_style(
        source: *const c_char,
        filename: *const c_char,
        id: *const c_char,
        scoped: bool,
    ) -> RawHandle;

    /// Gets the compiled CSS code.
    ///
    /// # Safety
    ///
    /// - `handle` must be a valid style result handle from [`vue_compile_style`].
    /// - Must be called from the main thread.
    ///
    /// # Returns
    ///
    /// A pointer to the compiled CSS as a null-terminated UTF-8 string.
    /// The string is owned by the handle and remains valid until the handle is freed.
    #[must_use]
    pub fn vue_style_result_code(handle: RawHandle) -> *const c_char;
}
