//! Error types for whichtime-sys

use thiserror::Error;

/// Convenience result type used throughout `whichtime-sys`.
pub type Result<T> = std::result::Result<T, Error>;

/// Error values returned by parsing and locale setup code.
#[derive(Error, Debug)]
pub enum Error {
    /// The requested locale code or name is not supported.
    #[error("Locale not found: {0}")]
    LocaleNotFound(String),
    /// A regex used by the parsing pipeline failed to compile or execute.
    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),
    /// Parsed components could not be turned into a valid date/time.
    #[error("Invalid date/time: {0}")]
    InvalidDateTime(String),
    /// Generic parser failure.
    #[error("Parse error: {0}")]
    Parse(String),
}
