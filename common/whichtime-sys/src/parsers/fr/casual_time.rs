//! French casual time parser
//!
//! Handles French casual time expressions like:
//! - "à midi", "a midi"
//! - "à minuit", "a minuit"

use crate::components::Component;
use crate::context::ParsingContext;
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use crate::scanner::TokenType;
use crate::types::Meridiem;
use chrono::{Datelike, Duration, Timelike};
use fancy_regex::Regex;
use std::sync::LazyLock;

static PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)(?:à\s+|a\s+)?(midi|minuit)(?=\W|$)").unwrap());

/// French casual time parser
pub struct FRCasualTimeParser;

impl FRCasualTimeParser {
    pub fn new() -> Self {
        Self
    }
}

impl Parser for FRCasualTimeParser {
    fn name(&self) -> &'static str {
        "FRCasualTimeParser"
    }

    fn should_apply(&self, context: &ParsingContext) -> bool {
        context.has_token_type(TokenType::CasualTime)
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();
        let ref_date = context.reference.instant;

        let mut start = 0;
        while start < context.text.len() {
            let search_text = &context.text[start..];
            let mat = match PATTERN.find(search_text) {
                Ok(Some(m)) => m,
                Ok(None) => break,
                Err(_) => break,
            };

            let matched_text = mat.as_str();
            let index = start + mat.start();

            let caps = match PATTERN.captures(matched_text) {
                Ok(Some(c)) => c,
                Ok(None) => {
                    start = index + 1;
                    continue;
                }
                Err(_) => {
                    start = index + 1;
                    continue;
                }
            };

            let time_word = caps
                .get(1)
                .map(|m| m.as_str().to_lowercase())
                .unwrap_or_default();

            let mut components = context.create_components();
            let mut target_date = ref_date;

            match time_word.as_str() {
                "midi" => {
                    components.assign(Component::Hour, 12);
                    components.assign(Component::Minute, 0);
                    components.assign(Component::Second, 0);
                    components.assign(Component::Meridiem, Meridiem::PM as i32);
                }
                "minuit" => {
                    // Midnight is at 00:00 of the next day if reference is in the afternoon
                    if ref_date.hour() >= 12 {
                        target_date = ref_date + Duration::days(1);
                    }
                    components.assign(Component::Hour, 0);
                    components.assign(Component::Minute, 0);
                    components.assign(Component::Second, 0);
                }
                _ => {
                    start = index + 1;
                    continue;
                }
            }

            // Set date components
            components.assign(Component::Year, target_date.year());
            components.assign(Component::Month, target_date.month() as i32);
            components.assign(Component::Day, target_date.day() as i32);

            results.push(context.create_result(
                index,
                index + matched_text.len(),
                components,
                None,
            ));

            start = index + matched_text.len();
        }

        Ok(results)
    }
}

impl Default for FRCasualTimeParser {
    fn default() -> Self {
        Self::new()
    }
}
