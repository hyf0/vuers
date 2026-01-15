//! Custom block type for SFC parsing.

use std::collections::HashMap;

use super::attr_value::AttrValue;
use super::handle::Handle;
use super::source_location::{Position, SourceLocation};
use crate::ffi::{self, HermesHandle, HermesRuntime};
use crate::util::ptr_to_str;

/// Custom block from an SFC (e.g., `<i18n>`, `<docs>`).
pub struct CustomBlock<'c>(pub(crate) Handle<'c>);

impl<'c> CustomBlock<'c> {
    pub(crate) fn from_handle(handle: Handle<'c>) -> Self {
        CustomBlock(handle)
    }
}

impl CustomBlock<'_> {
    /// Get the block type (e.g., "i18n", "docs").
    pub fn block_type(&self) -> &str {
        unsafe { ptr_to_str(ffi::vue_custom_block_type(*self.0.runtime(), self.0.raw())) }
    }

    /// Get the block content.
    pub fn content(&self) -> &str {
        unsafe { ptr_to_str(ffi::vue_block_content(*self.0.runtime(), self.0.raw())) }
    }

    /// Get the block language.
    pub fn lang(&self) -> &str {
        unsafe { ptr_to_str(ffi::vue_block_lang(*self.0.runtime(), self.0.raw())) }
    }

    /// Get the src attribute if present.
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

/// Helper function to get source location from a block handle.
pub(crate) fn get_block_loc(rt: HermesRuntime, handle: HermesHandle) -> SourceLocation {
    SourceLocation {
        start: Position {
            offset: unsafe { ffi::vue_block_loc_start_offset(rt, handle) },
            line: unsafe { ffi::vue_block_loc_start_line(rt, handle) },
            column: unsafe { ffi::vue_block_loc_start_column(rt, handle) },
        },
        end: Position {
            offset: unsafe { ffi::vue_block_loc_end_offset(rt, handle) },
            line: unsafe { ffi::vue_block_loc_end_line(rt, handle) },
            column: unsafe { ffi::vue_block_loc_end_column(rt, handle) },
        },
    }
}

/// Helper function to get attributes from a block handle.
pub(crate) fn get_block_attrs(
    rt: HermesRuntime,
    handle: HermesHandle,
) -> HashMap<String, AttrValue> {
    let count = unsafe { ffi::vue_block_attrs_count(rt, handle) };
    let mut attrs = HashMap::with_capacity(count);

    for i in 0..count {
        let key = unsafe { ptr_to_str(ffi::vue_block_attrs_key_at(rt, handle, i)).to_string() };
        let is_bool = unsafe { ffi::vue_block_attrs_is_bool_at(rt, handle, i) };

        let value = if is_bool {
            AttrValue::Bool(true)
        } else {
            let val =
                unsafe { ptr_to_str(ffi::vue_block_attrs_value_at(rt, handle, i)).to_string() };
            AttrValue::String(val)
        };

        attrs.insert(key, value);
    }

    attrs
}
