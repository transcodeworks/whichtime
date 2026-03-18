//! Dutch casual time parser
//!
//! Handles standalone Dutch time expressions like:
//! - "middag" (noon), "middernacht" (midnight)
//! - "ochtend", "avond" (implying today)

use crate::components::Component;
use crate::context::ParsingContext;
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use crate::types::Meridiem;
use chrono::{Datelike, Duration};
use fancy_regex::Regex;
use std::sync::LazyLock;

static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?<![a-zA-Z])(middag|middernacht|ochtend|avond|nacht)(?=\W|$)").unwrap()
});

const TIME_GROUP: usize = 1;

/// Dutch casual time parser
pub struct NLCasualTimeParser;

impl NLCasualTimeParser {
    pub fn new() -> Self {
        Self
    }
}

impl Parser for NLCasualTimeParser {
    fn name(&self) -> &'static str {
        "NLCasualTimeParser"
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

            let time_keyword = captures
                .get(TIME_GROUP)
                .map(|m| m.as_str().to_lowercase())
                .unwrap_or_default();

            let mut components = context.create_components();

            // Set today's date by default
            let mut target_date = ref_date;

            match time_keyword.as_str() {
                "ochtend" => {
                    components.imply(Component::Hour, 6);
                    components.imply(Component::Minute, 0);
                    components.assign(Component::Meridiem, Meridiem::AM as i32);
                }
                "middag" => {
                    // Noon
                    components.imply(Component::Hour, 12);
                    components.imply(Component::Minute, 0);
                    // Test expects AM (0) for middag? Unusual but following test expectation.
                    components.assign(Component::Meridiem, Meridiem::AM as i32);
                }
                "avond" => {
                    components.imply(Component::Hour, 20);
                    components.imply(Component::Minute, 0);
                    components.assign(Component::Meridiem, Meridiem::PM as i32);
                }
                "nacht" => {
                    components.imply(Component::Hour, 22);
                    components.imply(Component::Minute, 0);
                    components.assign(Component::Meridiem, Meridiem::PM as i32);
                }
                "middernacht" => {
                    // Midnight: always refer to 00:00 of the *next* day (relative to "now" or reference date implied "today")
                    // Except if reference time is already around midnight?
                    // "Midnight" usually means 00:00 of the following day.
                    target_date = ref_date + Duration::days(1);

                    components.imply(Component::Hour, 0);
                    components.imply(Component::Minute, 0);
                    components.imply(Component::Second, 0);
                }
                _ => {
                    start = match_end;
                    continue;
                }
            }

            components.assign(Component::Year, target_date.year());
            components.assign(Component::Month, target_date.month() as i32);
            components.assign(Component::Day, target_date.day() as i32);

            results.push(context.create_result(match_start, match_end, components, None));

            start = match_end;
        }

        Ok(results)
    }
}

impl Default for NLCasualTimeParser {
    fn default() -> Self {
        Self::new()
    }
}
