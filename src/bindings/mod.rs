//! Safe Rust bindings for Vue SFC compilation.
//!
//! This module provides ergonomic Rust types that wrap opaque handles
//! to JavaScript objects. Handles are automatically freed on drop.
//!
//! # Example
//!
//! ```ignore
//! use libvue_compiler_sfc::{ParseResult, compile_template, compile_style};
//!
//! let parsed = ParseResult::parse(&source, "App.vue")?;
//! let desc = parsed.descriptor()?;
//! let script = desc.compile_script("scope-id", false)?;
//! let template = compile_template(
//!     desc.template()?.content(),
//!     "App.vue",
//!     "scope-id",
//!     true,
//!     script.bindings().as_ref(),
//! )?;
//! ```

mod handle;
mod error;
mod util;
mod parse;
mod blocks;
mod compile;

pub use handle::Handle;
pub use error::{Error, Result};
pub use parse::{ParseResult, Descriptor};
pub use blocks::{TemplateBlock, ScriptBlock, StyleBlock};
pub use compile::{
    ScriptResult, TemplateResult, StyleResult, Bindings,
    compile_template, compile_style,
};
