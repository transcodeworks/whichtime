//! Parser infrastructure and locale-specific implementations.
//!
//! Parsers are responsible for identifying date expressions in text and
//! turning them into [`ParsedResult`] values before refiners run.

pub mod common;
pub mod de;
pub mod en;
pub mod es;
pub mod fr;
pub mod it;
pub mod ja;
pub mod nl;
pub mod pt;
pub mod ru;
pub mod sv;
pub mod uk;
pub mod zh;

use crate::context::ParsingContext;
use crate::error::Result;
use crate::results::ParsedResult;

/// Trait implemented by individual date/time parsers.
pub trait Parser: Send + Sync {
    /// Get a name for this parser (for debugging)
    fn name(&self) -> &'static str;

    /// Check if this parser should run on the given context
    /// This is a fast pre-filter to skip parsers that won't match
    fn should_apply(&self, context: &ParsingContext) -> bool;

    /// Parse the text and return all matches
    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>>;
}
