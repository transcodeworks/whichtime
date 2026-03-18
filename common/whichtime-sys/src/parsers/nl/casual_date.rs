//! Dutch casual date parser
//!
//! Handles Dutch casual date expressions like:
//! - "vandaag", "morgen", "gisteren"
//! - "vanavond", "vanochtend"
//! - "morgenavond", "gisterenmiddag"
//! - "deze avond"

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
        r"(?i)(?<![a-zA-Z])(nu|vandaag|overmorgen|eergisteren|van(?:ochtend|middag|avond|nacht)|morgen(?:ochtend|middag|avond|nacht)|gisteren(?:ochtend|middag|avond|nacht)|deze\s+(?:ochtend|middag|namiddag|avond|nacht)|morgen|gisteren)(?:\s+(?:om\s+)?(\d{1,2})(?::(\d{1,2}))?(?:\s*uhr|\s*uur)?)?(?=\W|$)"
    ).unwrap()
});

const DATE_GROUP: usize = 1;
const HOUR_GROUP: usize = 2;
const MINUTE_GROUP: usize = 3;

/// Dutch casual date parser
pub struct NLCasualDateParser;

impl NLCasualDateParser {
    pub fn new() -> Self {
        Self
    }

    fn assign_time(components: &mut crate::components::FastComponents, time_part: &str) {
        match time_part {
            "ochtend" | "vanochtend" => {
                components.imply(Component::Hour, 6);
                components.imply(Component::Minute, 0);
                components.imply(Component::Second, 0);
                components.assign(Component::Meridiem, Meridiem::AM as i32);
            }
            "middag" | "vanmiddag" => {
                components.imply(Component::Hour, 12); // Noon
                components.imply(Component::Minute, 0);
                components.imply(Component::Second, 0);
                components.assign(Component::Meridiem, Meridiem::PM as i32);
            }
            "namiddag" => {
                components.imply(Component::Hour, 15); // Afternoon
                components.imply(Component::Minute, 0);
                components.imply(Component::Second, 0);
                components.assign(Component::Meridiem, Meridiem::PM as i32);
            }
            "avond" | "vanavond" => {
                components.imply(Component::Hour, 20);
                components.imply(Component::Minute, 0);
                components.imply(Component::Second, 0);
                components.assign(Component::Meridiem, Meridiem::PM as i32);
            }
            "nacht" | "vannacht" => {
                components.imply(Component::Hour, 22);
                components.imply(Component::Minute, 0);
                components.imply(Component::Second, 0);
                components.assign(Component::Meridiem, Meridiem::PM as i32);
            }
            _ => {}
        }
    }
}

impl Parser for NLCasualDateParser {
    fn name(&self) -> &'static str {
        "NLCasualDateParser"
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

            let explicit_hour: Option<i32> = captures
                .get(HOUR_GROUP)
                .and_then(|m| m.as_str().parse().ok());

            let explicit_minute: Option<i32> = captures
                .get(MINUTE_GROUP)
                .and_then(|m| m.as_str().parse().ok());

            let mut components = context.create_components();
            let mut target_date = ref_date;

            // Logic to determine target date and time part
            let (day_offset, time_part) = if date_keyword == "nu" || date_keyword == "vandaag" {
                (0, None)
            } else if date_keyword == "morgen" {
                (1, None)
            } else if date_keyword == "overmorgen" {
                (2, None)
            } else if date_keyword == "gisteren" {
                (-1, None)
            } else if date_keyword == "eergisteren" {
                (-2, None)
            } else if let Some(time_part) = date_keyword.strip_prefix("van") {
                // vanochtend, vanmiddag, vanavond, vannacht -> Today
                let part = if date_keyword == "vannacht" {
                    "nacht"
                } else {
                    time_part
                };
                (0, Some(part.to_string()))
            } else if let Some(time_part) = date_keyword.strip_prefix("morgen")
                && date_keyword.len() > 6
            {
                // morgenochtend etc.
                (1, Some(time_part.to_string()))
            } else if let Some(time_part) = date_keyword.strip_prefix("gisteren")
                && date_keyword.len() > 8
            {
                // gisterenochtend etc.
                (-1, Some(time_part.to_string()))
            } else if let Some(time_part) = date_keyword.strip_prefix("deze ") {
                // deze avond
                (0, Some(time_part.to_string()))
            } else {
                (0, None)
            };

            // Adjust day
            if day_offset != 0 {
                target_date = ref_date + Duration::days(day_offset);
            }

            components.assign(Component::Year, target_date.year());
            components.assign(Component::Month, target_date.month() as i32);
            components.assign(Component::Day, target_date.day() as i32);

            if date_keyword == "nu" {
                components.assign(Component::Hour, ref_date.hour() as i32);
                components.assign(Component::Minute, ref_date.minute() as i32);
                components.assign(Component::Second, ref_date.second() as i32);
            }

            // Apply implied time
            if let Some(ref tp) = time_part {
                Self::assign_time(&mut components, tp);
            }

            // Handle explicit hour override
            if let Some(hour) = explicit_hour {
                // If time part implies PM (avond, etc) and hour < 12, make it PM
                let adjusted_hour =
                    if let Some(ref tp) = time_part {
                        match tp.as_str() {
                            "avond" | "nacht" | "namiddag" | "vanavond" | "vannacht" => {
                                if hour < 12 { hour + 12 } else { hour }
                            }
                            "ochtend" | "vanochtend" => {
                                if hour == 12 {
                                    0
                                } else {
                                    hour
                                }
                            }
                            _ => hour,
                        }
                    } else {
                        hour
                    };
                components.assign(Component::Hour, adjusted_hour);
                components.assign(Component::Minute, explicit_minute.unwrap_or(0));
            }

            results.push(context.create_result(match_start, match_end, components, None));

            start = match_end;
        }

        Ok(results)
    }
}

impl Default for NLCasualDateParser {
    fn default() -> Self {
        Self::new()
    }
}
