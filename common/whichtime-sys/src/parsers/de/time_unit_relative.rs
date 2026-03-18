//! German relative time unit parser
//!
//! Handles expressions like:
//! - "kommende Woche" (next week)
//! - "letzten Monat" (last month)
//! - "letztes Quartal" (last quarter)
//! - "kommendes Jahr" (next year)

use crate::components::Component;
use crate::context::ParsingContext;
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use crate::types::{Duration, add_duration};
use chrono::{Datelike, Timelike};
use fancy_regex::Regex;
use std::sync::LazyLock;

static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)(?<![a-zA-ZäöüÄÖÜß])(kommend(?:e[rnms]?)?|nächst(?:e[rnms]?)?|naechst(?:e[rnms]?)?|letzt(?:e[rnms]?)?|vorig(?:e[rnms]?)?|vergangen(?:e[rnms]?)?)\s+(woche|monat(?:s)?|quartal(?:s)?|jahr(?:es)?)\b"
    ).unwrap()
});

const MODIFIER_GROUP: usize = 1;
const UNIT_GROUP: usize = 2;

/// German relative time unit parser
pub struct DETimeUnitRelativeParser;

impl DETimeUnitRelativeParser {
    pub fn new() -> Self {
        Self
    }
}

impl Parser for DETimeUnitRelativeParser {
    fn name(&self) -> &'static str {
        "DETimeUnitRelativeParser"
    }

    fn should_apply(&self, _context: &ParsingContext) -> bool {
        true
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();
        let ref_date = context.reference.instant;

        let mut start = 0;
        while start < context.text.len() {
            let search_text = &context.text[start..];
            let captures = match PATTERN.captures(search_text) {
                Ok(Some(caps)) => caps,
                Ok(None) => break,
                Err(_) => break,
            };

            let full_match = match captures.get(0) {
                Some(m) => m,
                None => break,
            };

            let match_start = start + full_match.start();
            let match_end = start + full_match.end();

            let modifier = captures
                .get(MODIFIER_GROUP)
                .map(|m| m.as_str().to_lowercase())
                .unwrap_or_default();

            let unit = captures
                .get(UNIT_GROUP)
                .map(|m| m.as_str().to_lowercase())
                .unwrap_or_default();

            // Determine direction: next (+1) or last (-1)
            let multiplier: f64 = if modifier.starts_with("kommend")
                || modifier.starts_with("nächst")
                || modifier.starts_with("naechst")
            {
                1.0
            } else {
                -1.0
            };

            let mut duration = Duration::new();

            if unit.starts_with("woche") {
                duration.week = Some(multiplier);
            } else if unit.starts_with("monat") {
                duration.month = Some(multiplier);
            } else if unit.starts_with("quartal") {
                duration.quarter = Some(multiplier);
            } else if unit.starts_with("jahr") {
                duration.year = Some(multiplier);
            }

            let target_date = add_duration(ref_date, &duration);

            let mut components = context.create_components();
            components.assign(Component::Year, target_date.year());
            components.assign(Component::Month, target_date.month() as i32);
            components.assign(Component::Day, target_date.day() as i32);
            // Imply time from reference for relative date expressions
            components.imply(Component::Hour, target_date.hour() as i32);
            components.imply(Component::Minute, target_date.minute() as i32);
            components.imply(Component::Second, target_date.second() as i32);

            results.push(context.create_result(match_start, match_end, components, None));

            start = match_end;
        }

        Ok(results)
    }
}

impl Default for DETimeUnitRelativeParser {
    fn default() -> Self {
        Self::new()
    }
}
