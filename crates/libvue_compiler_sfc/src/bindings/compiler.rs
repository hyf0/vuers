//! Vue SFC Compiler instance.
//!
//! Each `Compiler` owns its own Hermes runtime and can be used independently.
//! This enables thread-safe parallel compilation by creating one Compiler per thread.

use super::compile::{ScriptOutput, StyleOutput, TemplateOutput};
use super::error::{Error, Result};
use super::parse::ParseOutput;
use crate::ffi::{self, HermesHandle, HermesRuntime};

/// Vue SFC compiler instance.
///
/// Each `Compiler` owns its own Hermes runtime and handle table.
/// It is `Send` but NOT `Sync` - can be moved between threads,
/// but cannot be shared across threads simultaneously.
///
/// # Example
///
/// ```ignore
/// use libvue_compiler_sfc::Compiler;
///
/// let compiler = Compiler::new()?;
/// let parsed = compiler.parse(source, "App.vue")?;
/// let desc = parsed.descriptor()?;
/// ```
///
/// # Thread Safety
///
/// A single `Compiler` instance must only be used from one thread at a time.
/// To compile in parallel, create multiple `Compiler` instances.
pub struct Compiler {
    pub(crate) runtime: HermesRuntime,
}

impl Compiler {
    /// Creates a new compiler instance.
    ///
    /// This initializes a fresh Hermes runtime. The operation is relatively
    /// expensive (~100ms), so reuse compiler instances when possible.
    ///
    /// # Errors
    ///
    /// Returns an error if the Hermes runtime fails to initialize.
    pub fn new() -> Result<Self> {
        let runtime = unsafe { ffi::hermes_runtime_create() };
        if runtime.is_null() {
            return Err(Error::new("Failed to create compiler instance"));
        }
        Ok(Self { runtime })
    }

    /// Parses a Vue Single File Component source string.
    ///
    /// # Arguments
    ///
    /// * `source` - The SFC source code as a string.
    /// * `filename` - The filename (used for error messages and source maps).
    ///
    /// # Returns
    ///
    /// A `ParseOutput` that provides access to the parsed descriptor and any errors.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let compiler = Compiler::new()?;
    /// let result = compiler.parse("<template>Hello</template>", "App.vue")?;
    /// assert!(!result.has_errors());
    /// ```
    pub fn parse<'c>(&'c self, source: &str, filename: &str) -> Result<ParseOutput<'c>> {
        use std::os::raw::c_char;

        let handle = unsafe {
            ffi::vue_parse(
                self.runtime,
                source.as_ptr() as *const c_char,
                source.len(),
                filename.as_ptr() as *const c_char,
                filename.len(),
            )
        };

        if !handle.is_valid() {
            return Err(Error::new("Parse returned invalid handle"));
        }

        Ok(ParseOutput::from_raw(handle, self))
    }

    /// Compiles a Vue template to a render function.
    ///
    /// # Arguments
    ///
    /// * `source` - The template source code.
    /// * `filename` - The filename (for error messages).
    /// * `id` - A unique scope ID for scoped CSS (e.g., "data-v-abc123").
    /// * `scoped` - Whether to add scoped attribute selectors.
    /// * `bindings` - Optional bindings from script compilation for optimization.
    pub fn compile_template<'c>(
        &'c self,
        source: &str,
        filename: &str,
        id: &str,
        scoped: bool,
        bindings: Option<&ScriptOutput<'c>>,
    ) -> Result<TemplateOutput<'c>> {
        use std::os::raw::c_char;

        let bindings_handle = bindings
            .map(|b| b.bindings_handle())
            .unwrap_or(HermesHandle::INVALID);

        let handle = unsafe {
            ffi::vue_compile_template(
                self.runtime,
                source.as_ptr() as *const c_char,
                source.len(),
                filename.as_ptr() as *const c_char,
                filename.len(),
                id.as_ptr() as *const c_char,
                id.len(),
                scoped,
                bindings_handle,
            )
        };

        if !handle.is_valid() {
            return Err(Error::new("compile_template returned invalid handle"));
        }

        Ok(TemplateOutput::from_raw(handle, self))
    }

    /// Compiles a CSS style block.
    ///
    /// # Arguments
    ///
    /// * `source` - The CSS source code.
    /// * `filename` - The filename (for error messages).
    /// * `id` - A unique scope ID for scoped CSS.
    /// * `scoped` - Whether to add scoped attribute selectors.
    pub fn compile_style<'c>(
        &'c self,
        source: &str,
        filename: &str,
        id: &str,
        scoped: bool,
    ) -> Result<StyleOutput<'c>> {
        use std::os::raw::c_char;

        let handle = unsafe {
            ffi::vue_compile_style(
                self.runtime,
                source.as_ptr() as *const c_char,
                source.len(),
                filename.as_ptr() as *const c_char,
                filename.len(),
                id.as_ptr() as *const c_char,
                id.len(),
                scoped,
            )
        };

        if !handle.is_valid() {
            return Err(Error::new("compile_style returned invalid handle"));
        }

        Ok(StyleOutput::from_raw(handle, self))
    }
}

impl Drop for Compiler {
    fn drop(&mut self) {
        unsafe { ffi::hermes_runtime_destroy(self.runtime) };
    }
}

// Compiler is Send but NOT Sync
// It can be moved to another thread, but can't be shared between threads
unsafe impl Send for Compiler {}

// Explicitly NOT implementing Sync:
// impl !Sync for Compiler {}
// Note: Rust doesn't have negative trait impls yet, but since Compiler
// contains a raw handle (u64), it would normally be Sync. We rely on
// documentation and the design to prevent misuse.
