//! Parsing context

use crate::components::FastComponents;
use crate::dictionaries::Locale;
use crate::results::{ParsedResult, ReferenceWithTimezone};
use crate::scanner::TokenScanner;

/// Shared state for a single parse request.
///
/// Parsers and refiners receive a `ParsingContext` so they can reuse the input
/// text, reference time, locale, and precomputed token scan.
pub struct ParsingContext<'a> {
    /// Original input text.
    pub text: &'a str,
    /// Lowercased input cached for parser prefilters.
    lower_text: String,
    /// Reference date/time used to resolve relative expressions.
    pub reference: &'a ReferenceWithTimezone,
    /// Tokens discovered during the scanner pre-pass.
    pub tokens: Vec<crate::scanner::Token>,
    /// Active locale for the parse request.
    pub locale: Locale,
}

impl<'a> ParsingContext<'a> {
    /// Create a new English parsing context.
    pub fn new(text: &'a str, reference: &'a ReferenceWithTimezone) -> Self {
        Self::with_locale(text, reference, Locale::En)
    }

    /// Create a parsing context for a specific locale.
    pub fn with_locale(
        text: &'a str,
        reference: &'a ReferenceWithTimezone,
        locale: Locale,
    ) -> Self {
        let lower_text = text.to_lowercase();
        let tokens = TokenScanner::scan_locale(&lower_text, locale);

        Self {
            text,
            lower_text,
            reference,
            tokens,
            locale,
        }
    }

    /// Return the cached lowercase representation of the input.
    #[inline]
    pub fn lower_text(&self) -> &str {
        &self.lower_text
    }

    /// Create a fresh component set seeded from the reference date.
    #[inline]
    pub fn create_components(&self) -> FastComponents {
        FastComponents::with_defaults(self.reference)
    }

    /// Build a [`ParsedResult`] for a matched span.
    pub fn create_result(
        &self,
        index: usize,
        end_index: usize,
        start: FastComponents,
        end: Option<FastComponents>,
    ) -> ParsedResult {
        let text = &self.text[index..end_index];
        ParsedResult::new(self.reference, index, text, start, end)
    }

    /// Return `true` when the scanner found a token of the given type.
    pub fn has_token_type(&self, token_type: crate::scanner::TokenType) -> bool {
        self.tokens.iter().any(|t| t.token_type == token_type)
    }
}
