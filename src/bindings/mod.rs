//! Safe Rust bindings for Vue SFC compilation.
//!
//! This module provides ergonomic Rust types that wrap opaque handles
//! to JavaScript objects. Handles are automatically freed on drop.
//!
//! # Example
//!
//! ```ignore
//! use libvue_compiler_sfc::{parse, compile_script, compile_template, compile_style};
//!
//! let parsed = parse(&source, "App.vue")?;
//! let desc = parsed.descriptor()?;
//! let script = compile_script(&desc, "scope-id", false)?;
//! let template = compile_template(
//!     desc.template()?.content(),
//!     "App.vue",
//!     "scope-id",
//!     true,
//!     Some(&script),
//! )?;
//! ```

mod handle;
mod error;
mod util;
mod parse;
mod blocks;
mod compile;

pub use error::{Error, Result};
pub use parse::{parse, compile_script, ParseOutput, Descriptor};
pub use blocks::{TemplateBlock, ScriptBlock, StyleBlock};
pub use compile::{
    ScriptOutput, TemplateOutput, StyleOutput,
    compile_template, compile_style,
};
