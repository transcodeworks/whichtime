//! Portuguese casual time parser
//!
//! Handles standalone Portuguese time expressions like:
//! - "agora"
//! - "meio-dia", "meia-noite"
//! - "ao meio-dia", "a meia-noite"

use crate::components::Component;
use crate::context::ParsingContext;
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use crate::types::Meridiem;
use chrono::{Datelike, Duration, Timelike};
use fancy_regex::Regex;
use std::sync::LazyLock;

static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?<![a-zA-Z])(?:(?:a|ao|à)\s+)?(meio-dia|meia-noite|agora)(?=\W|$)").unwrap()
});

const TIME_GROUP: usize = 1;

/// Portuguese casual time parser
pub struct PTCasualTimeParser;

impl PTCasualTimeParser {
    pub fn new() -> Self {
        Self
    }
}

impl Parser for PTCasualTimeParser {
    fn name(&self) -> &'static str {
        "PTCasualTimeParser"
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
            let mut target_date = ref_date;

            match time_keyword.as_str() {
                "agora" => {
                    components.assign(Component::Year, ref_date.year());
                    components.assign(Component::Month, ref_date.month() as i32);
                    components.assign(Component::Day, ref_date.day() as i32);
                    components.assign(Component::Hour, ref_date.hour() as i32);
                    components.assign(Component::Minute, ref_date.minute() as i32);
                    components.assign(Component::Second, ref_date.second() as i32);
                }
                "meio-dia" => {
                    components.assign(Component::Year, target_date.year());
                    components.assign(Component::Month, target_date.month() as i32);
                    components.assign(Component::Day, target_date.day() as i32);
                    components.imply(Component::Hour, 12);
                    components.imply(Component::Minute, 0);
                    components.assign(Component::Meridiem, Meridiem::AM as i32);
                }
                "meia-noite" => {
                    // Midnight logic:
                    // If implied, usually means coming midnight.
                    // Always move to the next midnight (00:00) relative to now.
                    // If it's 11:00, midnight is tomorrow 00:00.
                    // If it's 23:00, midnight is tomorrow 00:00.
                    // We assume standard logic: midnight is the start of the NEXT day.
                    target_date = ref_date + Duration::days(1);

                    components.assign(Component::Year, target_date.year());
                    components.assign(Component::Month, target_date.month() as i32);
                    components.assign(Component::Day, target_date.day() as i32);
                    components.imply(Component::Hour, 0);
                    components.imply(Component::Minute, 0);
                }
                _ => {
                    start = match_end;
                    continue;
                }
            }

            results.push(context.create_result(match_start, match_end, components, None));

            start = match_end;
        }

        Ok(results)
    }
}

impl Default for PTCasualTimeParser {
    fn default() -> Self {
        Self::new()
    }
}
