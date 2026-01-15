//! Import binding metadata type.

/// Import binding metadata from script setup analysis.
#[derive(Debug, Clone)]
pub struct ImportBinding {
    /// Whether this is a type-only import.
    pub is_type: bool,
    /// The imported name (e.g., "ref" from `import { ref } from 'vue'`).
    pub imported: String,
    /// The source module (e.g., "vue").
    pub source: String,
    /// Whether the import is from the setup function.
    pub is_from_setup: bool,
}
