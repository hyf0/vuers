//! Vue Single File Component compiler for Rust.
//!
//! This crate provides Rust bindings to the Vue SFC compiler (`@vue/compiler-sfc`),
//! compiled to native code via Static Hermes.
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
//! use libvue_compiler_sfc::{ParseResult, compile_template, compile_style};
//!
//! fn compile(source: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
//!     let parsed = ParseResult::parse(source, "App.vue")?;
//!     let desc = parsed.descriptor().ok_or("No descriptor")?;
//!
//!     // Compile script
//!     let script = desc.compile_script("scope-id", false)?;
//!
//!     // Compile template with bindings from script
//!     let template = compile_template(
//!         desc.template().unwrap().content(),
//!         "App.vue",
//!         "scope-id",
//!         desc.has_scoped_style(),
//!         script.bindings().as_ref(),
//!     )?;
//!
//!     // Compile styles
//!     let css: Vec<String> = desc.styles()
//!         .map(|s| compile_style(s.content(), "App.vue", "scope-id", s.is_scoped()))
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

// Layer 1: Raw FFI (unsafe, extern "C")
pub mod ffi;

// Layer 2: Safe Rust bindings (recommended)
pub mod bindings;

// Re-export bindings API as the primary API
pub use bindings::{
    // Core types
    Handle, Error, Result,
    // Parse
    ParseResult, Descriptor,
    // Blocks
    TemplateBlock, ScriptBlock, StyleBlock,
    // Compile results
    ScriptResult, TemplateResult, StyleResult, Bindings,
    // Compile functions
    compile_template, compile_style,
};
