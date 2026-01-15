//! Error types for the bindings API.

/// Error type for binding operations.
#[derive(Debug)]
pub struct Error(String);

impl Error {
    /// Create a new error with the given message.
    pub(crate) fn new(message: impl Into<String>) -> Self {
        Error(message.into())
    }

    /// Get the error message.
    pub fn message(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {}

/// Result type alias for binding operations.
pub type Result<T> = std::result::Result<T, Error>;
