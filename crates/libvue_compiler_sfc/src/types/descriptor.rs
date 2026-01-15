//! SFC Descriptor type.

use std::os::raw::c_char;

use super::custom_block::CustomBlock;
use super::error::{Error, Result};
use super::handle::Handle;
use super::script_block::ScriptBlock;
use super::script_output::ScriptOutput;
use super::style_block::StyleBlock;
use super::template_block::TemplateBlock;
use crate::ffi;
use crate::util::ptr_to_str;

/// SFC Descriptor containing all parsed blocks.
pub struct Descriptor<'c>(Handle<'c>);

impl<'c> Descriptor<'c> {
    pub(crate) fn from_handle(handle: Handle<'c>) -> Self {
        Descriptor(handle)
    }

    /// Check if the SFC has a template block.
    pub fn has_template(&self) -> bool {
        unsafe { ffi::vue_descriptor_has_template(*self.0.runtime(), self.0.raw()) }
    }

    /// Check if the SFC has a regular script block.
    pub fn has_script(&self) -> bool {
        unsafe { ffi::vue_descriptor_has_script(*self.0.runtime(), self.0.raw()) }
    }

    /// Check if the SFC has a script setup block.
    pub fn has_script_setup(&self) -> bool {
        unsafe { ffi::vue_descriptor_has_script_setup(*self.0.runtime(), self.0.raw()) }
    }

    /// Get the number of style blocks.
    pub fn style_count(&self) -> usize {
        unsafe { ffi::vue_descriptor_style_count(*self.0.runtime(), self.0.raw()) }
    }

    /// Get the template block.
    pub fn template(&self) -> Option<TemplateBlock<'c>> {
        if !self.has_template() {
            return None;
        }
        let handle = unsafe { ffi::vue_descriptor_template(*self.0.runtime(), self.0.raw()) };
        Handle::new(handle, self.0.runtime()).map(TemplateBlock::from_handle)
    }

    /// Get the regular script block (not setup).
    pub fn script(&self) -> Option<ScriptBlock<'c>> {
        if !self.has_script() {
            return None;
        }
        let handle = unsafe { ffi::vue_descriptor_script(*self.0.runtime(), self.0.raw()) };
        Handle::new(handle, self.0.runtime()).map(ScriptBlock::from_handle)
    }

    /// Get the script setup block.
    pub fn script_setup(&self) -> Option<ScriptBlock<'c>> {
        if !self.has_script_setup() {
            return None;
        }
        let handle = unsafe { ffi::vue_descriptor_script_setup(*self.0.runtime(), self.0.raw()) };
        Handle::new(handle, self.0.runtime()).map(ScriptBlock::from_handle)
    }

    /// Iterate over all style blocks.
    pub fn styles(&self) -> impl Iterator<Item = StyleBlock<'c>> + '_ {
        let count = self.style_count();
        (0..count).filter_map(move |index| {
            let handle =
                unsafe { ffi::vue_descriptor_style_at(*self.0.runtime(), self.0.raw(), index) };
            Handle::new(handle, self.0.runtime()).map(StyleBlock::from_handle)
        })
    }

    /// Check if any style block is scoped.
    pub fn has_scoped_style(&self) -> bool {
        self.styles().any(|s| s.is_scoped())
    }

    /// Get the original source code of the SFC.
    pub fn source(&self) -> &str {
        unsafe { ptr_to_str(ffi::vue_descriptor_source(*self.0.runtime(), self.0.raw())) }
    }

    /// Get the filename of the SFC.
    pub fn filename(&self) -> &str {
        unsafe {
            ptr_to_str(ffi::vue_descriptor_filename(
                *self.0.runtime(),
                self.0.raw(),
            ))
        }
    }

    /// Get the number of CSS variables extracted from scoped styles.
    pub fn css_vars_count(&self) -> usize {
        unsafe { ffi::vue_descriptor_css_vars_count(*self.0.runtime(), self.0.raw()) }
    }

    /// Get the CSS variable names extracted from scoped styles.
    ///
    /// These are v-bind() expressions found in `<style>` blocks.
    pub fn css_vars(&self) -> Vec<String> {
        let count = self.css_vars_count();
        let mut vars = Vec::with_capacity(count);

        for i in 0..count {
            let var = unsafe {
                ptr_to_str(ffi::vue_descriptor_css_var_at(
                    *self.0.runtime(),
                    self.0.raw(),
                    i,
                ))
                .to_string()
            };
            vars.push(var);
        }

        vars
    }

    /// Check if the SFC uses :slotted() in scoped styles.
    pub fn slotted(&self) -> bool {
        unsafe { ffi::vue_descriptor_slotted(*self.0.runtime(), self.0.raw()) }
    }

    /// Get the number of custom blocks (e.g., `<i18n>`, `<docs>`).
    pub fn custom_blocks_count(&self) -> usize {
        unsafe { ffi::vue_descriptor_custom_blocks_count(*self.0.runtime(), self.0.raw()) }
    }

    /// Iterate over all custom blocks.
    pub fn custom_blocks(&self) -> impl Iterator<Item = CustomBlock<'c>> + '_ {
        let count = self.custom_blocks_count();
        (0..count).filter_map(move |index| {
            let handle = unsafe {
                ffi::vue_descriptor_custom_block_at(*self.0.runtime(), self.0.raw(), index)
            };
            Handle::new(handle, self.0.runtime()).map(CustomBlock::from_handle)
        })
    }

    /// Compile the script blocks from this descriptor.
    pub fn compile_script(&self, id: &str, is_prod: bool) -> Result<ScriptOutput<'c>> {
        let handle = unsafe {
            ffi::vue_compile_script(
                *self.0.runtime(),
                self.0.raw(),
                id.as_ptr() as *const c_char,
                id.len(),
                is_prod,
            )
        };
        Handle::new(handle, self.0.runtime())
            .map(ScriptOutput::from_handle)
            .ok_or_else(|| Error::new("compile_script returned invalid handle"))
    }
}
