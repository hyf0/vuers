//! Source location types for SFC parsing.

/// Source position in the SFC file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    /// Byte offset from the start of the file.
    pub offset: usize,
    /// Line number (1-indexed).
    pub line: usize,
    /// Column number (1-indexed).
    pub column: usize,
}

/// Source location spanning a range in the SFC file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceLocation {
    /// Start position.
    pub start: Position,
    /// End position.
    pub end: Position,
}
