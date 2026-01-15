//! Attribute value type for SFC blocks.

/// Attribute value that can be either a string or a boolean flag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttrValue {
    /// String value (e.g., `lang="ts"`).
    String(String),
    /// Boolean flag (key-only attribute, e.g., `scoped`).
    Bool(bool),
}
