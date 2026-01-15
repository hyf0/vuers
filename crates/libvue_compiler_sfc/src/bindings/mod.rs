//! Safe Rust bindings for Vue SFC compilation.
//!
//! This module provides ergonomic Rust types that wrap opaque handles
//! to JavaScript objects. Handles are automatically freed on drop.
//!
//! # Example
//!
//! ```ignore
//! use libvue_compiler_sfc::Compiler;
//!
//! let compiler = Compiler::new()?;
//! let parsed = compiler.parse(&source, "App.vue")?;
//! let desc = parsed.descriptor()?;
//! let script = desc.compile_script("scope-id", false)?;
//! let template = compiler.compile_template(
//!     desc.template()?.content(),
//!     "App.vue",
//!     "scope-id",
//!     true,
//!     Some(&script),
//! )?;
//! ```
//!
//! # Thread Safety
//!
//! Each `Compiler` instance must only be used from one thread at a time.
//! To compile in parallel, create multiple `Compiler` instances - each
//! owns its own Hermes runtime.

mod compiler;
mod handle;
mod error;
mod util;
mod types;
mod parse;
mod blocks;
mod compile;

pub use compiler::Compiler;
pub use error::{Error, Result};
pub use parse::{ParseOutput, Descriptor};
pub use blocks::{TemplateBlock, ScriptBlock, StyleBlock};
pub use types::{SourceLocation, Position, AttrValue, ImportBinding, CustomBlock};
pub use compile::{ScriptOutput, TemplateOutput, StyleOutput};
