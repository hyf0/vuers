//! Raw extern "C" function declarations for the Vue SFC compiler FFI.
//!
//! This module provides direct bindings to the C++ wrapper functions defined in
//! `ffi/cpp/runtime.cpp` and `ffi/cpp/vue_sfc.cpp`. All functions operate on
//! opaque handles and require careful attention to memory management.
//!
//! # Memory Model
//!
//! - **HermesRuntime**: Opaque pointer to the Hermes runtime. Must be destroyed
//!   with [`hermes_runtime_destroy`] when no longer needed.
//! - **HermesHandle**: Opaque 64-bit identifiers for JS objects. Must be freed
//!   with [`hermes_handle_free`] when no longer needed.
//! - **Strings**: Returned `*const c_char` pointers are owned by their parent
//!   handle and remain valid until that handle is freed.
//!
//! # Thread Safety
//!
//! - Each runtime instance must only be used from one thread at a time
//! - Multiple runtime instances can be used in parallel from different threads

use std::os::raw::c_char;

// ============================================================================
// Types
// ============================================================================

/// Opaque pointer to a Hermes runtime instance.
///
/// The runtime owns:
/// - Static Hermes runtime (SHRuntime*)
/// - JSI runtime
/// - Handle table for JS objects
/// - Cached Vue compiler function references
///
/// # Lifetime
///
/// Must be explicitly destroyed with [`hermes_runtime_destroy`] when no longer
/// needed. All handles created by this runtime become invalid after destruction.
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct HermesRuntime(*mut std::ffi::c_void);

impl HermesRuntime {
    /// Returns `true` if this runtime pointer is null.
    #[inline]
    pub fn is_null(self) -> bool {
        self.0.is_null()
    }
}

/// Opaque handle to a JavaScript object in the Hermes runtime.
///
/// Handles are 64-bit identifiers that reference entries in an internal handle
/// table managed by the runtime. They provide a way to pass JS objects across
/// the FFI boundary without exposing internal pointers.
///
/// # Representation
///
/// - `0` represents an invalid/null handle
/// - Non-zero values are 1-indexed into the handle table
///
/// # Lifetime
///
/// Handles must be explicitly freed with [`hermes_handle_free`] when no longer
/// needed. Failing to free handles will leak memory. Strings returned by handle
/// accessor functions are owned by the handle and become invalid when the handle
/// is freed.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HermesHandle(pub u64);

impl HermesHandle {
    /// The invalid/null handle value.
    pub const INVALID: Self = HermesHandle(0);

    /// Returns `true` if this handle is valid (non-null).
    #[inline]
    #[must_use]
    pub fn is_valid(self) -> bool {
        self.0 != 0
    }
}

impl Default for HermesHandle {
    fn default() -> Self {
        Self::INVALID
    }
}

impl From<u64> for HermesHandle {
    fn from(value: u64) -> Self {
        HermesHandle(value)
    }
}

impl From<HermesHandle> for u64 {
    fn from(handle: HermesHandle) -> Self {
        handle.0
    }
}

// ============================================================================
// FFI Function Declarations
// ============================================================================

extern "C" {
    // ------------------------------------------------------------------------
    // Runtime Lifecycle
    // ------------------------------------------------------------------------

    /// Creates a new Hermes runtime instance.
    ///
    /// This function:
    /// 1. Initializes a fresh Static Hermes runtime
    /// 2. Loads the Vue compiler unit
    /// 3. Caches references to Vue compiler functions
    ///
    /// # Safety
    ///
    /// - The returned runtime must be destroyed with [`hermes_runtime_destroy`].
    ///
    /// # Returns
    ///
    /// A valid runtime pointer on success, or null on failure.
    #[must_use]
    pub fn hermes_runtime_create() -> HermesRuntime;

    /// Destroys a runtime instance and releases all its resources.
    ///
    /// All handles created by this runtime become invalid after this call.
    ///
    /// # Safety
    ///
    /// - `rt` must be a valid runtime from [`hermes_runtime_create`],
    ///   or null (which is a no-op).
    /// - The runtime must not be used after this call.
    /// - All handles created by this runtime become invalid.
    pub fn hermes_runtime_destroy(rt: HermesRuntime);

    // ------------------------------------------------------------------------
    // Handle Management
    // ------------------------------------------------------------------------

    /// Frees a handle and releases its associated resources.
    ///
    /// After calling this function, the handle becomes invalid and must not be
    /// used. Any strings that were returned by accessor functions for this
    /// handle also become invalid.
    ///
    /// # Safety
    ///
    /// - `rt` must be a valid runtime.
    /// - `handle` should be a valid handle previously returned by an FFI
    ///   function, or `HermesHandle::INVALID` (which is a no-op).
    /// - The handle must not be used after this call.
    pub fn hermes_handle_free(rt: HermesRuntime, handle: HermesHandle);

    // ------------------------------------------------------------------------
    // Parsing
    // ------------------------------------------------------------------------

    /// Parses a Vue Single File Component source string.
    ///
    /// Returns a handle to a `ParseResult` object containing the parsed
    /// descriptor and any parse errors.
    ///
    /// # Safety
    ///
    /// - `rt` must be a valid runtime.
    /// - `source` must be a valid UTF-8 byte slice of length `source_len`.
    /// - `filename` must be a valid UTF-8 byte slice of length `filename_len`.
    /// - Both pointers must remain valid for the duration of the call.
    ///
    /// # Returns
    ///
    /// A valid handle on success. The handle must be freed with
    /// [`hermes_handle_free`]. Even if parsing fails, a valid handle is
    /// returned containing error information.
    #[must_use]
    pub fn vue_parse(
        rt: HermesRuntime,
        source: *const c_char,
        source_len: usize,
        filename: *const c_char,
        filename_len: usize,
    ) -> HermesHandle;

    /// Gets the descriptor handle from a parse result.
    #[must_use]
    pub fn vue_parse_result_descriptor(rt: HermesRuntime, handle: HermesHandle) -> HermesHandle;

    /// Gets the number of parse errors.
    #[must_use]
    pub fn vue_parse_result_error_count(rt: HermesRuntime, handle: HermesHandle) -> usize;

    /// Gets an error message at the specified index.
    #[must_use]
    pub fn vue_parse_result_error_message(
        rt: HermesRuntime,
        handle: HermesHandle,
        index: usize,
    ) -> *const c_char;

    // ------------------------------------------------------------------------
    // Descriptor Accessors
    // ------------------------------------------------------------------------

    #[must_use]
    pub fn vue_descriptor_has_template(rt: HermesRuntime, handle: HermesHandle) -> bool;

    #[must_use]
    pub fn vue_descriptor_has_script(rt: HermesRuntime, handle: HermesHandle) -> bool;

    #[must_use]
    pub fn vue_descriptor_has_script_setup(rt: HermesRuntime, handle: HermesHandle) -> bool;

    #[must_use]
    pub fn vue_descriptor_style_count(rt: HermesRuntime, handle: HermesHandle) -> usize;

    #[must_use]
    pub fn vue_descriptor_template(rt: HermesRuntime, handle: HermesHandle) -> HermesHandle;

    #[must_use]
    pub fn vue_descriptor_script_setup(rt: HermesRuntime, handle: HermesHandle) -> HermesHandle;

    #[must_use]
    pub fn vue_descriptor_style_at(
        rt: HermesRuntime,
        handle: HermesHandle,
        index: usize,
    ) -> HermesHandle;

    #[must_use]
    pub fn vue_descriptor_script(rt: HermesRuntime, handle: HermesHandle) -> HermesHandle;

    #[must_use]
    pub fn vue_descriptor_custom_blocks_count(rt: HermesRuntime, handle: HermesHandle) -> usize;

    #[must_use]
    pub fn vue_descriptor_custom_block_at(
        rt: HermesRuntime,
        handle: HermesHandle,
        index: usize,
    ) -> HermesHandle;

    #[must_use]
    pub fn vue_descriptor_css_vars_count(rt: HermesRuntime, handle: HermesHandle) -> usize;

    #[must_use]
    pub fn vue_descriptor_css_var_at(
        rt: HermesRuntime,
        handle: HermesHandle,
        index: usize,
    ) -> *const c_char;

    #[must_use]
    pub fn vue_descriptor_slotted(rt: HermesRuntime, handle: HermesHandle) -> bool;

    #[must_use]
    pub fn vue_descriptor_source(rt: HermesRuntime, handle: HermesHandle) -> *const c_char;

    #[must_use]
    pub fn vue_descriptor_filename(rt: HermesRuntime, handle: HermesHandle) -> *const c_char;

    // ------------------------------------------------------------------------
    // Block Accessors
    // ------------------------------------------------------------------------

    #[must_use]
    pub fn vue_block_content(rt: HermesRuntime, handle: HermesHandle) -> *const c_char;

    #[must_use]
    pub fn vue_block_lang(rt: HermesRuntime, handle: HermesHandle) -> *const c_char;

    #[must_use]
    pub fn vue_style_is_scoped(rt: HermesRuntime, handle: HermesHandle) -> bool;

    #[must_use]
    pub fn vue_custom_block_type(rt: HermesRuntime, handle: HermesHandle) -> *const c_char;

    // ------------------------------------------------------------------------
    // Block Location Accessors
    // ------------------------------------------------------------------------

    #[must_use]
    pub fn vue_block_loc_start_offset(rt: HermesRuntime, handle: HermesHandle) -> usize;

    #[must_use]
    pub fn vue_block_loc_start_line(rt: HermesRuntime, handle: HermesHandle) -> usize;

    #[must_use]
    pub fn vue_block_loc_start_column(rt: HermesRuntime, handle: HermesHandle) -> usize;

    #[must_use]
    pub fn vue_block_loc_end_offset(rt: HermesRuntime, handle: HermesHandle) -> usize;

    #[must_use]
    pub fn vue_block_loc_end_line(rt: HermesRuntime, handle: HermesHandle) -> usize;

    #[must_use]
    pub fn vue_block_loc_end_column(rt: HermesRuntime, handle: HermesHandle) -> usize;

    // ------------------------------------------------------------------------
    // Block Attribute Accessors
    // ------------------------------------------------------------------------

    #[must_use]
    pub fn vue_block_src(rt: HermesRuntime, handle: HermesHandle) -> *const c_char;

    #[must_use]
    pub fn vue_block_attrs_count(rt: HermesRuntime, handle: HermesHandle) -> usize;

    #[must_use]
    pub fn vue_block_attrs_key_at(
        rt: HermesRuntime,
        handle: HermesHandle,
        index: usize,
    ) -> *const c_char;

    #[must_use]
    pub fn vue_block_attrs_value_at(
        rt: HermesRuntime,
        handle: HermesHandle,
        index: usize,
    ) -> *const c_char;

    #[must_use]
    pub fn vue_block_attrs_is_bool_at(
        rt: HermesRuntime,
        handle: HermesHandle,
        index: usize,
    ) -> bool;

    // ------------------------------------------------------------------------
    // Script Block Specific Accessors
    // ------------------------------------------------------------------------

    #[must_use]
    pub fn vue_script_has_setup(rt: HermesRuntime, handle: HermesHandle) -> bool;

    #[must_use]
    pub fn vue_script_setup_value(rt: HermesRuntime, handle: HermesHandle) -> *const c_char;

    #[must_use]
    pub fn vue_script_bindings_count(rt: HermesRuntime, handle: HermesHandle) -> usize;

    #[must_use]
    pub fn vue_script_bindings_key_at(
        rt: HermesRuntime,
        handle: HermesHandle,
        index: usize,
    ) -> *const c_char;

    #[must_use]
    pub fn vue_script_bindings_value_at(
        rt: HermesRuntime,
        handle: HermesHandle,
        index: usize,
    ) -> *const c_char;

    #[must_use]
    pub fn vue_script_imports_count(rt: HermesRuntime, handle: HermesHandle) -> usize;

    #[must_use]
    pub fn vue_script_imports_key_at(
        rt: HermesRuntime,
        handle: HermesHandle,
        index: usize,
    ) -> *const c_char;

    #[must_use]
    pub fn vue_script_imports_value_at(
        rt: HermesRuntime,
        handle: HermesHandle,
        index: usize,
    ) -> HermesHandle;

    #[must_use]
    pub fn vue_import_binding_is_type(rt: HermesRuntime, handle: HermesHandle) -> bool;

    #[must_use]
    pub fn vue_import_binding_imported(rt: HermesRuntime, handle: HermesHandle) -> *const c_char;

    #[must_use]
    pub fn vue_import_binding_source(rt: HermesRuntime, handle: HermesHandle) -> *const c_char;

    #[must_use]
    pub fn vue_import_binding_is_from_setup(rt: HermesRuntime, handle: HermesHandle) -> bool;

    #[must_use]
    pub fn vue_script_warnings_count(rt: HermesRuntime, handle: HermesHandle) -> usize;

    #[must_use]
    pub fn vue_script_warning_at(
        rt: HermesRuntime,
        handle: HermesHandle,
        index: usize,
    ) -> *const c_char;

    #[must_use]
    pub fn vue_script_deps_count(rt: HermesRuntime, handle: HermesHandle) -> usize;

    #[must_use]
    pub fn vue_script_dep_at(
        rt: HermesRuntime,
        handle: HermesHandle,
        index: usize,
    ) -> *const c_char;

    // ------------------------------------------------------------------------
    // Style Block Specific Accessors
    // ------------------------------------------------------------------------

    #[must_use]
    pub fn vue_style_has_module(rt: HermesRuntime, handle: HermesHandle) -> bool;

    #[must_use]
    pub fn vue_style_module_value(rt: HermesRuntime, handle: HermesHandle) -> *const c_char;

    // ------------------------------------------------------------------------
    // Script Compilation
    // ------------------------------------------------------------------------

    #[must_use]
    pub fn vue_compile_script(
        rt: HermesRuntime,
        descriptor: HermesHandle,
        id: *const c_char,
        id_len: usize,
        is_prod: bool,
    ) -> HermesHandle;

    #[must_use]
    pub fn vue_script_result_content(rt: HermesRuntime, handle: HermesHandle) -> *const c_char;

    #[must_use]
    pub fn vue_script_result_bindings(rt: HermesRuntime, handle: HermesHandle) -> HermesHandle;

    // ------------------------------------------------------------------------
    // Template Compilation
    // ------------------------------------------------------------------------

    #[must_use]
    pub fn vue_compile_template(
        rt: HermesRuntime,
        source: *const c_char,
        source_len: usize,
        filename: *const c_char,
        filename_len: usize,
        id: *const c_char,
        id_len: usize,
        scoped: bool,
        bindings: HermesHandle,
    ) -> HermesHandle;

    #[must_use]
    pub fn vue_template_result_code(rt: HermesRuntime, handle: HermesHandle) -> *const c_char;

    #[must_use]
    pub fn vue_template_result_error_count(rt: HermesRuntime, handle: HermesHandle) -> usize;

    // ------------------------------------------------------------------------
    // Style Compilation
    // ------------------------------------------------------------------------

    #[must_use]
    pub fn vue_compile_style(
        rt: HermesRuntime,
        source: *const c_char,
        source_len: usize,
        filename: *const c_char,
        filename_len: usize,
        id: *const c_char,
        id_len: usize,
        scoped: bool,
    ) -> HermesHandle;

    #[must_use]
    pub fn vue_style_result_code(rt: HermesRuntime, handle: HermesHandle) -> *const c_char;
}
