//! Vue Single File Component compiler for Rust.
//!
//! This crate provides Rust bindings to the Vue SFC compiler (`@vue/compiler-sfc`),
//! compiled to native code via Static Hermes.
//!
//! # Thread Safety
//!
//! Each `Compiler` instance must only be used from one thread at a time.
//! To compile in parallel, create multiple `Compiler` instances - each
//! owns its own Hermes runtime.
//!
//! # API Layers
//!
//! The crate is organized into two layers:
//!
//! - **`bindings`** (recommended): Safe Rust types with RAII and methods
//! - **`ffi`**: Raw FFI bindings (unsafe, for advanced use)
//!
//! # Example
//!
//! ```no_run
//! use libvue_compiler_sfc::Compiler;
//!
//! fn compile(source: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
//!     let compiler = Compiler::new()?;
//!     let parsed = compiler.parse(source, "App.vue")?;
//!     let desc = parsed.descriptor().ok_or("No descriptor")?;
//!
//!     // Compile script
//!     let script = desc.compile_script("scope-id", false)?;
//!
//!     // Compile template with bindings from script
//!     let template = compiler.compile_template(
//!         desc.template().unwrap().content(),
//!         "App.vue",
//!         "scope-id",
//!         desc.has_scoped_style(),
//!         Some(&script),
//!     )?;
//!
//!     // Compile styles
//!     let css: Vec<String> = desc.styles()
//!         .map(|s| compiler.compile_style(s.content(), "App.vue", "scope-id", s.is_scoped()))
//!         .collect::<Result<Vec<_>, _>>()?
//!         .into_iter()
//!         .map(|r| r.code().to_string())
//!         .collect();
//!
//!     Ok((
//!         format!("{}\n{}", script.content(), template.code()),
//!         css.join("\n"),
//!     ))
//! }
//! ```

// Layer 1: Raw FFI (unsafe, extern "C") - re-exported from sys crate
pub use lib_vue_compiler_sfc_sys as ffi;

// Layer 2: Safe Rust types and compiler
mod compiler;
pub(crate) mod types;
mod util;

// Tests
#[cfg(test)]
mod tests;

// Re-export public API
pub use compiler::Compiler;
pub use types::{
    AttrValue, CustomBlock, Descriptor, Error, ImportBinding, ParseOutput, Position, Result,
    ScriptBlock, ScriptOutput, SourceLocation, StyleBlock, StyleOutput, TemplateBlock,
    TemplateOutput,
};
