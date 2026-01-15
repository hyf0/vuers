//! Parse result and descriptor types.

use std::os::raw::c_char;

use crate::ffi;
use super::handle::Handle;
use super::error::{Error, Result};
use super::util::ptr_to_str;
use super::blocks::{TemplateBlock, ScriptBlock, StyleBlock};
use super::compile::ScriptOutput;

/// Parse an SFC source string.
pub fn parse(source: &str, filename: &str) -> Result<ParseOutput> {
    let handle = unsafe {
        ffi::vue_parse(
            source.as_ptr() as *const c_char,
            source.len(),
            filename.as_ptr() as *const c_char,
            filename.len(),
        )
    };

    Handle::new(handle)
        .map(ParseOutput)
        .ok_or_else(|| Error::new("Parse returned invalid handle"))
}

/// Compile the script blocks from a descriptor.
pub fn compile_script(descriptor: &Descriptor, id: &str, is_prod: bool) -> Result<ScriptOutput> {
    let handle = unsafe {
        ffi::vue_compile_script(
            descriptor.0.raw(),
            id.as_ptr() as *const c_char,
            id.len(),
            is_prod,
        )
    };
    Handle::new(handle)
        .map(ScriptOutput::from_handle)
        .ok_or_else(|| Error::new("compile_script returned invalid handle"))
}

/// Output of parsing an SFC source file.
pub struct ParseOutput(Handle);

impl ParseOutput {
    /// Get the SFC descriptor containing all parsed blocks.
    pub fn descriptor(&self) -> Option<Descriptor> {
        let handle = unsafe { ffi::vue_parse_result_descriptor(self.0.raw()) };
        Handle::new(handle).map(Descriptor)
    }

    /// Get the number of parse errors.
    pub fn error_count(&self) -> usize {
        unsafe { ffi::vue_parse_result_error_count(self.0.raw()) }
    }

    /// Get an error message by index.
    pub fn error_message(&self, index: usize) -> &str {
        unsafe { ptr_to_str(ffi::vue_parse_result_error_message(self.0.raw(), index)) }
    }

    /// Check if parsing produced errors.
    pub fn has_errors(&self) -> bool {
        self.error_count() > 0 || self.descriptor().is_none()
    }

    /// Iterate over all error messages.
    pub fn errors(&self) -> impl Iterator<Item = &str> {
        (0..self.error_count()).map(move |i| self.error_message(i))
    }
}

/// SFC Descriptor containing all parsed blocks.
pub struct Descriptor(Handle);

impl Descriptor {
    /// Check if the SFC has a template block.
    pub fn has_template(&self) -> bool {
        unsafe { ffi::vue_descriptor_has_template(self.0.raw()) }
    }

    /// Check if the SFC has a regular script block.
    pub fn has_script(&self) -> bool {
        unsafe { ffi::vue_descriptor_has_script(self.0.raw()) }
    }

    /// Check if the SFC has a script setup block.
    pub fn has_script_setup(&self) -> bool {
        unsafe { ffi::vue_descriptor_has_script_setup(self.0.raw()) }
    }

    /// Get the number of style blocks.
    pub fn style_count(&self) -> usize {
        unsafe { ffi::vue_descriptor_style_count(self.0.raw()) }
    }

    /// Get the template block.
    pub fn template(&self) -> Option<TemplateBlock> {
        if !self.has_template() {
            return None;
        }
        let handle = unsafe { ffi::vue_descriptor_template(self.0.raw()) };
        Handle::new(handle).map(TemplateBlock)
    }

    /// Get the script setup block.
    pub fn script_setup(&self) -> Option<ScriptBlock> {
        if !self.has_script_setup() {
            return None;
        }
        let handle = unsafe { ffi::vue_descriptor_script_setup(self.0.raw()) };
        Handle::new(handle).map(ScriptBlock)
    }

    /// Iterate over all style blocks.
    pub fn styles(&self) -> impl Iterator<Item = StyleBlock> + '_ {
        let count = self.style_count();
        (0..count).filter_map(move |index| {
            let handle = unsafe { ffi::vue_descriptor_style_at(self.0.raw(), index) };
            Handle::new(handle).map(StyleBlock)
        })
    }

    /// Check if any style block is scoped.
    pub fn has_scoped_style(&self) -> bool {
        self.styles().any(|s| s.is_scoped())
    }
}
