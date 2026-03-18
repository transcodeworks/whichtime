//! Russian casual time parser
//!
//! Handles standalone Russian time expressions like:
//! - "полдень", "полночь"
//! - "утро", "вечер" (implying today)

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
    Regex::new(
        r"(?i)(?<![a-zA-Zа-яА-Я])(?:(?:в|во|к)\s+)?(полдень|полудень|полночь|утро|вечер|ночь)(?=\W|$)"
    ).unwrap()
});

const TIME_GROUP: usize = 1;

/// Russian casual time parser
pub struct RUCasualTimeParser;

impl RUCasualTimeParser {
    pub fn new() -> Self {
        Self
    }
}

impl Parser for RUCasualTimeParser {
    fn name(&self) -> &'static str {
        "RUCasualTimeParser"
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
                "утро" => {
                    components.imply(Component::Hour, 6);
                    components.imply(Component::Minute, 0);
                    components.assign(Component::Meridiem, Meridiem::AM as i32);
                }
                "полдень" | "полудень" => {
                    components.imply(Component::Hour, 12);
                    components.imply(Component::Minute, 0);
                    components.assign(Component::Meridiem, Meridiem::PM as i32);
                }
                "вечер" => {
                    components.imply(Component::Hour, 20);
                    components.imply(Component::Minute, 0);
                    components.assign(Component::Meridiem, Meridiem::PM as i32);
                }
                "ночь" => {
                    components.imply(Component::Hour, 23); // or 23?
                    components.imply(Component::Minute, 0);
                    components.assign(Component::Meridiem, Meridiem::PM as i32);
                }
                "полночь" => {
                    // Midnight logic: usually 00:00 of next day
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

impl Default for RUCasualTimeParser {
    fn default() -> Self {
        Self::new()
    }
}
