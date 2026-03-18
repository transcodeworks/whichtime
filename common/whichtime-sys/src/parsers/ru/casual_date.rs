//! Russian casual date parser
//!
//! Handles Russian casual date expressions like:
//! - "сегодня", "завтра", "вчера"
//! - "сегодня вечером", "завтра утром"
//! - "вчера в 18:00"

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
        r"(?i)(?<![a-zA-Zа-яА-Я])(?:(сейчас|сегодня|завтра|послезавтра|вчера|позавчера)(?:\s+(утром|днем|днём|вечером|ночью))?|(?P<time_only>утром|днем|днём|вечером|ночью))(?:\s+(?:в|к)\s+)?(?:\s*(?P<noon>полдень|полудень|полночь))?(?:\s+(\d{1,2})(?::(\d{1,2}))?(?:\s*ч(?:\.|асов)?)?)?(?=\W|$)"
    ).unwrap()
});

const DATE_GROUP: usize = 1;
const TIME_PART_GROUP: usize = 2;
const TIME_ONLY_GROUP: usize = 3;
const NOON_GROUP: usize = 4;
const HOUR_GROUP: usize = 5;
const MINUTE_GROUP: usize = 6;

/// Russian casual date parser
pub struct RUCasualDateParser;

impl RUCasualDateParser {
    pub fn new() -> Self {
        Self
    }

    fn assign_time_part(components: &mut crate::components::FastComponents, time_part: &str) {
        match time_part {
            "утром" => {
                components.imply(Component::Hour, 6);
                components.imply(Component::Minute, 0);
                components.imply(Component::Second, 0);
                components.assign(Component::Meridiem, Meridiem::AM as i32);
            }
            "днем" | "днём" => {
                components.imply(Component::Hour, 14);
                components.imply(Component::Minute, 0);
                components.imply(Component::Second, 0);
                components.assign(Component::Meridiem, Meridiem::PM as i32);
            }
            "вечером" => {
                components.imply(Component::Hour, 20);
                components.imply(Component::Minute, 0);
                components.imply(Component::Second, 0);
                components.assign(Component::Meridiem, Meridiem::PM as i32);
            }
            "ночью" => {
                components.imply(Component::Hour, 23);
                components.imply(Component::Minute, 0);
                components.imply(Component::Second, 0);
                components.assign(Component::Meridiem, Meridiem::PM as i32);
            }
            _ => {}
        }
    }
}

impl Parser for RUCasualDateParser {
    fn name(&self) -> &'static str {
        "RUCasualDateParser"
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

            let date_keyword = captures.get(DATE_GROUP).map(|m| m.as_str().to_lowercase());

            let time_part_opt = captures
                .get(TIME_PART_GROUP)
                .map(|m| m.as_str().to_lowercase());

            let time_only = captures
                .get(TIME_ONLY_GROUP)
                .map(|m| m.as_str().to_lowercase());

            let time_part = time_part_opt.or(time_only);

            let noon_part = captures.get(NOON_GROUP).map(|m| m.as_str().to_lowercase());

            let explicit_hour: Option<i32> = captures
                .get(HOUR_GROUP)
                .and_then(|m| m.as_str().parse().ok());

            let explicit_minute: Option<i32> = captures
                .get(MINUTE_GROUP)
                .and_then(|m| m.as_str().parse().ok());

            let mut components = context.create_components();
            let mut target_date = ref_date;

            if let Some(kw) = date_keyword {
                match kw.as_str() {
                    "сегодня" => {}
                    "завтра" => {
                        target_date = ref_date + Duration::days(1);
                    }
                    "послезавтра" => {
                        target_date = ref_date + Duration::days(2);
                    }
                    "вчера" => {
                        target_date = ref_date - Duration::days(1);
                    }
                    "позавчера" => {
                        target_date = ref_date - Duration::days(2);
                    }
                    "сейчас" => {
                        components.assign(Component::Hour, ref_date.hour() as i32);
                        components.assign(Component::Minute, ref_date.minute() as i32);
                        components.assign(Component::Second, ref_date.second() as i32);
                    }
                    _ => {}
                }
            }

            components.assign(Component::Year, target_date.year());
            components.assign(Component::Month, target_date.month() as i32);
            components.assign(Component::Day, target_date.day() as i32);

            // Apply time part implications
            if let Some(ref tp) = time_part {
                Self::assign_time_part(&mut components, tp);
            }

            // Explicit hour overrides
            if let Some(hour) = explicit_hour {
                let mut adjusted_hour = hour;

                // Context-based adjustment
                if let Some(ref tp) = time_part {
                    match tp.as_str() {
                        "вечером" | "днем" | "днём" => {
                            // For 12-hour numbers, convert to PM (1 -> 13)
                            // But if number is e.g. 18, keep it.
                            if adjusted_hour < 12 {
                                adjusted_hour += 12;
                            }
                        }
                        "ночью" => {
                            if adjusted_hour < 6 {
                                // "ночью в 2" usually means 2am
                            } else if adjusted_hour < 12 {
                                // "ночью в 11" -> 23:00
                                adjusted_hour += 12;
                            }
                        }
                        _ => {}
                    }
                } else if adjusted_hour < 12 {
                    // No explicit time part context.
                }

                components.assign(Component::Hour, adjusted_hour);
                components.assign(Component::Minute, explicit_minute.unwrap_or(0));
            }

            // Check for "полдень"/"полночь"
            if let Some(noon) = noon_part {
                if noon.contains("полдень") || noon.contains("полудень") {
                    components.assign(Component::Hour, 12);
                    components.assign(Component::Minute, 0);
                    components.assign(Component::Meridiem, Meridiem::PM as i32);
                } else if noon.contains("полночь") {
                    // Midnight usually implies start of next day if mentioned as a target time "at midnight"
                    // But if just "midnight", can be ambiguous.
                    // Test `test_casual_v_polden` ("в полдень") expects noon.
                    // If no date keyword, imply today? Yes.
                    // If combined with "завтра в полдень", date is tomorrow.

                    components.assign(Component::Hour, 0);
                    components.assign(Component::Minute, 0);
                    components.assign(Component::Meridiem, Meridiem::AM as i32);
                }
            }

            results.push(context.create_result(match_start, match_end, components, None));

            start = match_end;
        }

        Ok(results)
    }
}

impl Default for RUCasualDateParser {
    fn default() -> Self {
        Self::new()
    }
}
