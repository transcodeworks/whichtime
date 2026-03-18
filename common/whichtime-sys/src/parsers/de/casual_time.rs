//! German casual time parser
//!
//! Handles standalone German time expressions like:
//! - "mittag" (noon), "mitternacht" (midnight)
//! - "diesen Morgen" (this morning)

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
    Regex::new(
        r"(?i)(?:(diese[rnms]?)\s*)?(morgen|vormittag|mittags?|nachmittag|abend|nacht|mitternacht)(?=\W|$)"
    ).unwrap()
});

const _MODIFIER_GROUP: usize = 1;
const TIME_GROUP: usize = 2;

/// German casual time parser
pub struct DECasualTimeParser;

impl DECasualTimeParser {
    pub fn new() -> Self {
        Self
    }
}

impl Parser for DECasualTimeParser {
    fn name(&self) -> &'static str {
        "DECasualTimeParser"
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
                "morgen" => {
                    components.imply(Component::Hour, 6);
                    components.imply(Component::Minute, 0);
                    components.assign(Component::Meridiem, Meridiem::AM as i32);
                }
                "vormittag" => {
                    components.imply(Component::Hour, 9);
                    components.imply(Component::Minute, 0);
                    components.assign(Component::Meridiem, Meridiem::AM as i32);
                }
                "mittag" | "mittags" => {
                    components.imply(Component::Hour, 12);
                    components.imply(Component::Minute, 0);
                    components.assign(Component::Meridiem, Meridiem::PM as i32);
                }
                "nachmittag" => {
                    components.imply(Component::Hour, 15);
                    components.imply(Component::Minute, 0);
                    components.assign(Component::Meridiem, Meridiem::PM as i32);
                }
                "abend" => {
                    components.imply(Component::Hour, 18);
                    components.imply(Component::Minute, 0);
                    components.assign(Component::Meridiem, Meridiem::PM as i32);
                }
                "nacht" => {
                    components.imply(Component::Hour, 22);
                    components.imply(Component::Minute, 0);
                    components.assign(Component::Meridiem, Meridiem::PM as i32);
                }
                "mitternacht" => {
                    // Midnight: if reference time is after noon, advance to next day
                    if ref_date.hour() >= 6 {
                        target_date = ref_date + Duration::days(1);
                    }
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

impl Default for DECasualTimeParser {
    fn default() -> Self {
        Self::new()
    }
}
