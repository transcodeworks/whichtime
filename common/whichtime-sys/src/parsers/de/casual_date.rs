//! German casual date parser
//!
//! Handles German casual date expressions like:
//! - "jetzt" (now), "heute" (today), "morgen" (tomorrow)
//! - "heute Morgen" (this morning), "gestern Abend" (yesterday evening)
//! - "letzte Nacht" (last night)

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
        r"(?i)(?<![a-zA-ZäöüÄÖÜß])(jetzt|heute|morgen|übermorgen|uebermorgen|gestern|vorgestern|letzte\s*nacht)(?:\s*(morgen|vormittag|mittags?|nachmittag|abend|nacht|mitternacht))?(?:\s+um\s+(\d{1,2})(?:\s*uhr)?)?(?=\W|$)"
    ).unwrap()
});

const DATE_GROUP: usize = 1;
const TIME_GROUP: usize = 2;
const HOUR_GROUP: usize = 3;

/// German casual date parser
pub struct DECasualDateParser;

impl DECasualDateParser {
    pub fn new() -> Self {
        Self
    }

    /// Extract time components from a time keyword
    fn extract_time_components(
        components: &mut crate::components::FastComponents,
        time_keyword: &str,
    ) {
        match time_keyword {
            "morgen" => {
                components.imply(Component::Hour, 6);
                components.imply(Component::Minute, 0);
                components.imply(Component::Second, 0);
                components.assign(Component::Meridiem, Meridiem::AM as i32);
            }
            "vormittag" => {
                components.imply(Component::Hour, 9);
                components.imply(Component::Minute, 0);
                components.imply(Component::Second, 0);
                components.assign(Component::Meridiem, Meridiem::AM as i32);
            }
            "mittag" | "mittags" => {
                components.imply(Component::Hour, 12);
                components.imply(Component::Minute, 0);
                components.imply(Component::Second, 0);
                components.assign(Component::Meridiem, Meridiem::PM as i32);
            }
            "nachmittag" => {
                components.imply(Component::Hour, 15);
                components.imply(Component::Minute, 0);
                components.imply(Component::Second, 0);
                components.assign(Component::Meridiem, Meridiem::PM as i32);
            }
            "abend" => {
                components.imply(Component::Hour, 18);
                components.imply(Component::Minute, 0);
                components.imply(Component::Second, 0);
                components.assign(Component::Meridiem, Meridiem::PM as i32);
            }
            "nacht" => {
                components.imply(Component::Hour, 22);
                components.imply(Component::Minute, 0);
                components.imply(Component::Second, 0);
                components.assign(Component::Meridiem, Meridiem::PM as i32);
            }
            "mitternacht" => {
                // Midnight is typically the next day if not specified
                components.imply(Component::Hour, 0);
                components.imply(Component::Minute, 0);
                components.imply(Component::Second, 0);
            }
            _ => {}
        }
    }
}

impl Parser for DECasualDateParser {
    fn name(&self) -> &'static str {
        "DECasualDateParser"
    }

    fn should_apply(&self, _context: &ParsingContext) -> bool {
        // This parser can always be applied for German locale
        true
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();
        let ref_date = context.reference.instant;

        // Use fancy_regex find_iter equivalent
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
            let _matched_text = full_match.as_str();

            let date_keyword = captures
                .get(DATE_GROUP)
                .map(|m| m.as_str().to_lowercase())
                .unwrap_or_default();

            let time_keyword = captures.get(TIME_GROUP).map(|m| m.as_str().to_lowercase());

            let explicit_hour: Option<i32> = captures
                .get(HOUR_GROUP)
                .and_then(|m| m.as_str().parse().ok());

            let mut components = context.create_components();
            let mut target_date = ref_date;

            match date_keyword.as_str() {
                "jetzt" => {
                    components.assign(Component::Year, ref_date.year());
                    components.assign(Component::Month, ref_date.month() as i32);
                    components.assign(Component::Day, ref_date.day() as i32);
                    components.assign(Component::Hour, ref_date.hour() as i32);
                    components.assign(Component::Minute, ref_date.minute() as i32);
                    components.assign(Component::Second, ref_date.second() as i32);
                }
                "heute" => {
                    components.assign(Component::Year, ref_date.year());
                    components.assign(Component::Month, ref_date.month() as i32);
                    components.assign(Component::Day, ref_date.day() as i32);
                }
                "morgen" => {
                    target_date = ref_date + Duration::days(1);
                    components.assign(Component::Year, target_date.year());
                    components.assign(Component::Month, target_date.month() as i32);
                    components.assign(Component::Day, target_date.day() as i32);
                }
                "übermorgen" | "uebermorgen" => {
                    target_date = ref_date + Duration::days(2);
                    components.assign(Component::Year, target_date.year());
                    components.assign(Component::Month, target_date.month() as i32);
                    components.assign(Component::Day, target_date.day() as i32);
                }
                "gestern" => {
                    target_date = ref_date - Duration::days(1);
                    components.assign(Component::Year, target_date.year());
                    components.assign(Component::Month, target_date.month() as i32);
                    components.assign(Component::Day, target_date.day() as i32);
                }
                "vorgestern" => {
                    target_date = ref_date - Duration::days(2);
                    components.assign(Component::Year, target_date.year());
                    components.assign(Component::Month, target_date.month() as i32);
                    components.assign(Component::Day, target_date.day() as i32);
                }
                _ if date_keyword.contains("letzte") && date_keyword.contains("nacht") => {
                    // "letzte Nacht" - last night
                    if ref_date.hour() > 6 {
                        target_date = ref_date - Duration::days(1);
                    }
                    components.assign(Component::Year, target_date.year());
                    components.assign(Component::Month, target_date.month() as i32);
                    components.assign(Component::Day, target_date.day() as i32);
                    components.imply(Component::Hour, 0);
                }
                _ => {}
            }

            // Apply time component if present
            if let Some(ref time_kw) = time_keyword {
                Self::extract_time_components(&mut components, time_kw);
            }

            // If explicit hour is given (e.g., "heute Abend um 8"), override the implied hour
            if let Some(hour) = explicit_hour {
                // Adjust hour based on time-of-day context
                let adjusted_hour = if let Some(ref time_kw) = time_keyword {
                    match time_kw.as_str() {
                        "abend" | "nacht" | "nachmittag" => {
                            // PM context: if hour < 12, add 12
                            if hour < 12 { hour + 12 } else { hour }
                        }
                        "morgen" | "vormittag" => {
                            // AM context: keep as is
                            if hour == 12 { 0 } else { hour }
                        }
                        _ => hour,
                    }
                } else {
                    hour
                };
                components.assign(Component::Hour, adjusted_hour);
                components.assign(Component::Minute, 0);
            }

            results.push(context.create_result(match_start, match_end, components, None));

            start = match_end;
        }

        Ok(results)
    }
}

impl Default for DECasualDateParser {
    fn default() -> Self {
        Self::new()
    }
}
