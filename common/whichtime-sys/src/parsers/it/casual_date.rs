//! Italian casual date parser
//!
//! Handles Italian casual date expressions like:
//! - "ora", "oggi", "domani", "ieri"
//! - "stamattina", "stasera"
//! - Combined: "domani alle 15:00"

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
        r"(?i)(?<![a-zA-ZàèéìòùÀÈÉÌÒÙ])(adesso|ora|oggi|domani|ieri|dopodomani|altroieri|l'altro\s+ieri|stamattina|stasera|stanotte)(?:\s+(mattina|pomeriggio|sera|notte))?(?:\s+(?:alle?\s+)?(\d{1,2})(?:[:\.](\d{2}))?)?(?=\W|$)"
    ).unwrap()
});

const DATE_GROUP: usize = 1;
const TIME_PERIOD_GROUP: usize = 2;
const HOUR_GROUP: usize = 3;
const MINUTE_GROUP: usize = 4;

/// Italian casual date parser
pub struct ITCasualDateParser;

impl ITCasualDateParser {
    pub fn new() -> Self {
        Self
    }
}

impl Parser for ITCasualDateParser {
    fn name(&self) -> &'static str {
        "ITCasualDateParser"
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

            let date_keyword = captures
                .get(DATE_GROUP)
                .map(|m| m.as_str().to_lowercase())
                .unwrap_or_default();

            let time_period = captures
                .get(TIME_PERIOD_GROUP)
                .map(|m| m.as_str().to_lowercase());

            let mut components = context.create_components();
            let target_date;

            // Process date keyword
            match date_keyword.as_str() {
                "adesso" | "ora" => {
                    components.assign(Component::Year, ref_date.year());
                    components.assign(Component::Month, ref_date.month() as i32);
                    components.assign(Component::Day, ref_date.day() as i32);
                    components.assign(Component::Hour, ref_date.hour() as i32);
                    components.assign(Component::Minute, ref_date.minute() as i32);
                    components.assign(Component::Second, ref_date.second() as i32);
                }
                "oggi" => {
                    components.assign(Component::Year, ref_date.year());
                    components.assign(Component::Month, ref_date.month() as i32);
                    components.assign(Component::Day, ref_date.day() as i32);
                }
                "domani" => {
                    target_date = ref_date + Duration::days(1);
                    components.assign(Component::Year, target_date.year());
                    components.assign(Component::Month, target_date.month() as i32);
                    components.assign(Component::Day, target_date.day() as i32);
                }
                "ieri" => {
                    target_date = ref_date - Duration::days(1);
                    components.assign(Component::Year, target_date.year());
                    components.assign(Component::Month, target_date.month() as i32);
                    components.assign(Component::Day, target_date.day() as i32);
                }
                "dopodomani" => {
                    target_date = ref_date + Duration::days(2);
                    components.assign(Component::Year, target_date.year());
                    components.assign(Component::Month, target_date.month() as i32);
                    components.assign(Component::Day, target_date.day() as i32);
                }
                "altroieri" | "l'altro ieri" => {
                    target_date = ref_date - Duration::days(2);
                    components.assign(Component::Year, target_date.year());
                    components.assign(Component::Month, target_date.month() as i32);
                    components.assign(Component::Day, target_date.day() as i32);
                }
                "stamattina" => {
                    components.assign(Component::Year, ref_date.year());
                    components.assign(Component::Month, ref_date.month() as i32);
                    components.assign(Component::Day, ref_date.day() as i32);
                    components.imply(Component::Hour, 6);
                    components.assign(Component::Meridiem, Meridiem::AM as i32);
                }
                "stasera" | "stanotte" => {
                    components.assign(Component::Year, ref_date.year());
                    components.assign(Component::Month, ref_date.month() as i32);
                    components.assign(Component::Day, ref_date.day() as i32);
                    components.imply(Component::Hour, 22);
                    components.assign(Component::Meridiem, Meridiem::PM as i32);
                }
                _ => {
                    start = match_end;
                    continue;
                }
            }

            // Apply time period modifier if present
            if let Some(ref period) = time_period {
                match period.as_str() {
                    "mattina" => {
                        components.imply(Component::Hour, 8);
                        components.assign(Component::Meridiem, Meridiem::AM as i32);
                    }
                    "pomeriggio" => {
                        components.imply(Component::Hour, 14);
                        components.assign(Component::Meridiem, Meridiem::PM as i32);
                    }
                    "sera" => {
                        components.imply(Component::Hour, 18);
                        components.assign(Component::Meridiem, Meridiem::PM as i32);
                    }
                    "notte" => {
                        components.imply(Component::Hour, 22);
                        components.assign(Component::Meridiem, Meridiem::PM as i32);
                    }
                    _ => {}
                }
            }

            // Handle explicit time: "alle 15:00"
            if let Some(hour_match) = captures.get(HOUR_GROUP) {
                let hour: i32 = hour_match.as_str().parse().unwrap_or(0);
                let minute: i32 = captures
                    .get(MINUTE_GROUP)
                    .map(|m| m.as_str().parse().unwrap_or(0))
                    .unwrap_or(0);

                components.assign(Component::Hour, hour);
                components.assign(Component::Minute, minute);

                if hour >= 12 {
                    components.assign(Component::Meridiem, Meridiem::PM as i32);
                } else {
                    components.assign(Component::Meridiem, Meridiem::AM as i32);
                }
            }

            results.push(context.create_result(match_start, match_end, components, None));

            start = match_end;
        }

        Ok(results)
    }
}

impl Default for ITCasualDateParser {
    fn default() -> Self {
        Self::new()
    }
}
