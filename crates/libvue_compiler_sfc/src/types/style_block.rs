//! Style block type for SFC parsing.

use std::collections::HashMap;

use super::attr_value::AttrValue;
use super::custom_block::{get_block_attrs, get_block_loc};
use super::handle::Handle;
use super::source_location::SourceLocation;
use crate::ffi;
use crate::util::ptr_to_str;

/// Style block from an SFC.
pub struct StyleBlock<'c>(pub(crate) Handle<'c>);

impl<'c> StyleBlock<'c> {
    pub(crate) fn from_handle(handle: Handle<'c>) -> Self {
        StyleBlock(handle)
    }
}

impl StyleBlock<'_> {
    /// Get the style content.
    pub fn content(&self) -> &str {
        unsafe { ptr_to_str(ffi::vue_block_content(*self.0.runtime(), self.0.raw())) }
    }

    /// Get the style language (e.g., "scss").
    pub fn lang(&self) -> &str {
        unsafe { ptr_to_str(ffi::vue_block_lang(*self.0.runtime(), self.0.raw())) }
    }

    /// Check if the style is scoped.
    pub fn is_scoped(&self) -> bool {
        unsafe { ffi::vue_style_is_scoped(*self.0.runtime(), self.0.raw()) }
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

    /// Check if the style uses CSS modules.
    pub fn has_module(&self) -> bool {
        unsafe { ffi::vue_style_has_module(*self.0.runtime(), self.0.raw()) }
    }

    /// Get the module attribute value if it's a string (custom module name).
    /// Returns None if module is boolean true or not present.
    pub fn module_name(&self) -> Option<&str> {
        if !self.has_module() {
            return None;
        }
        let s = unsafe { ptr_to_str(ffi::vue_style_module_value(*self.0.runtime(), self.0.raw())) };
        if s.is_empty() {
            None
        } else {
            Some(s)
        }
    }
}
