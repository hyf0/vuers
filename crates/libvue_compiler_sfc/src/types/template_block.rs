//! Template block type for SFC parsing.

use std::collections::HashMap;

use super::attr_value::AttrValue;
use super::custom_block::{get_block_attrs, get_block_loc};
use super::handle::Handle;
use super::source_location::SourceLocation;
use crate::ffi;
use crate::util::ptr_to_str;

/// Template block from an SFC.
pub struct TemplateBlock<'c>(pub(crate) Handle<'c>);

impl<'c> TemplateBlock<'c> {
    pub(crate) fn from_handle(handle: Handle<'c>) -> Self {
        TemplateBlock(handle)
    }
}

impl TemplateBlock<'_> {
    /// Get the template content.
    pub fn content(&self) -> &str {
        unsafe { ptr_to_str(ffi::vue_block_content(*self.0.runtime(), self.0.raw())) }
    }

    /// Get the template language (e.g., "pug").
    pub fn lang(&self) -> &str {
        unsafe { ptr_to_str(ffi::vue_block_lang(*self.0.runtime(), self.0.raw())) }
    }

    /// Get the src attribute if present (external file reference).
    pub fn src(&self) -> Option<&str> {
        let s = unsafe { ptr_to_str(ffi::vue_block_src(*self.0.runtime(), self.0.raw())) };
        if s.is_empty() {
            None
        } else {
            Some(s)
        }
    }

    /// Get the source location of the block.
    pub fn loc(&self) -> SourceLocation {
        get_block_loc(*self.0.runtime(), self.0.raw())
    }

    /// Get all attributes on the block.
    pub fn attrs(&self) -> HashMap<String, AttrValue> {
        get_block_attrs(*self.0.runtime(), self.0.raw())
    }
}
