//! Safe Rust types for Vue SFC compilation.
//!
//! This module provides ergonomic Rust types that wrap opaque handles
//! to JavaScript objects. Handles are automatically freed on drop.

mod attr_value;
mod custom_block;
mod descriptor;
mod error;
mod handle;
mod import_binding;
mod parse_output;
mod script_block;
mod script_output;
mod source_location;
mod style_block;
mod style_output;
mod template_block;
mod template_output;

pub use attr_value::AttrValue;
pub use custom_block::CustomBlock;
pub use descriptor::Descriptor;
pub use error::{Error, Result};
pub use import_binding::ImportBinding;
pub use parse_output::ParseOutput;
pub use script_block::ScriptBlock;
pub use script_output::ScriptOutput;
pub use source_location::{Position, SourceLocation};
pub use style_block::StyleBlock;
pub use style_output::StyleOutput;
pub use template_block::TemplateBlock;
pub use template_output::TemplateOutput;
