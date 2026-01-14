//! SFC block types (template, script, style).

use crate::ffi;
use super::handle::Handle;
use super::util::ptr_to_str;

/// Template block from an SFC.
pub struct TemplateBlock(pub(crate) Handle);

impl TemplateBlock {
    /// Get the template content.
    pub fn content(&self) -> &str {
        unsafe { ptr_to_str(ffi::vue_block_content(self.0.raw())) }
    }

    /// Get the template language (e.g., "pug").
    pub fn lang(&self) -> &str {
        unsafe { ptr_to_str(ffi::vue_block_lang(self.0.raw())) }
    }
}

/// Script block from an SFC.
pub struct ScriptBlock(pub(crate) Handle);

impl ScriptBlock {
    /// Get the script content.
    pub fn content(&self) -> &str {
        unsafe { ptr_to_str(ffi::vue_block_content(self.0.raw())) }
    }

    /// Get the script language (e.g., "ts").
    pub fn lang(&self) -> &str {
        unsafe { ptr_to_str(ffi::vue_block_lang(self.0.raw())) }
    }
}

/// Style block from an SFC.
pub struct StyleBlock(pub(crate) Handle);

impl StyleBlock {
    /// Get the style content.
    pub fn content(&self) -> &str {
        unsafe { ptr_to_str(ffi::vue_block_content(self.0.raw())) }
    }

    /// Get the style language (e.g., "scss").
    pub fn lang(&self) -> &str {
        unsafe { ptr_to_str(ffi::vue_block_lang(self.0.raw())) }
    }

    /// Check if the style is scoped.
    pub fn is_scoped(&self) -> bool {
        unsafe { ffi::vue_style_is_scoped(self.0.raw()) }
    }
}
