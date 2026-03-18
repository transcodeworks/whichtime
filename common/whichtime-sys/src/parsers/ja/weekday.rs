//! Japanese weekday parser
//!
//! Handles Japanese weekday expressions like:
//! - "木曜日", "木曜", "木" (Thursday)
//! - "(木)", "（木）" (Thursday in parentheses)
//! - "前の水曜日" (last Wednesday)
//! - "来週の金曜日" (next Friday)

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::ja as dict;
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use crate::scanner::TokenType;
use chrono::{Datelike, Duration};
use fancy_regex::Regex;
use std::sync::LazyLock;

// Pattern for weekday with optional modifier
static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?:(?P<modifier>前|先|次|来|今|この|来週|先週|今週)(?:の)?)?(?P<weekday>日曜日|月曜日|火曜日|水曜日|木曜日|金曜日|土曜日|日曜|月曜|火曜|水曜|木曜|金曜|土曜)"
    ).unwrap()
});

// Pattern for weekday in parentheses
static PAREN_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[（(](?P<weekday>日|月|火|水|木|金|土)[）)]").unwrap());

/// Japanese weekday parser
pub struct JAWeekdayParser;

impl JAWeekdayParser {
    pub fn new() -> Self {
        Self
    }
}

impl Parser for JAWeekdayParser {
    fn name(&self) -> &'static str {
        "JAWeekdayParser"
    }

    fn should_apply(&self, context: &ParsingContext) -> bool {
        context.has_token_type(TokenType::Weekday)
            || context.text.contains('曜')
            || context.text.contains('(')
            || context.text.contains('（')
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();
        let ref_date = context.reference.instant;

        let mut start = 0;
        while start < context.text.len() {
            let search_text = &context.text[start..];

            // Try parentheses pattern first
            if let Ok(Some(caps)) = PAREN_PATTERN.captures(search_text) {
                let full_match = caps.get(0).unwrap();
                let match_start = start + full_match.start();
                let match_end = start + full_match.end();

                let weekday_str = caps.name("weekday").map(|m| m.as_str()).unwrap_or_default();

                if let Some(weekday) = dict::get_weekday(weekday_str) {
                    // For parenthesized weekday, find the nearest occurrence
                    let ref_weekday = ref_date.weekday().num_days_from_monday() as i32;
                    let target_weekday = match weekday as i32 {
                        0 => 6, // Sunday
                        n => n - 1,
                    };
                    let diff = target_weekday - ref_weekday;
                    let target_date = ref_date + Duration::days(diff as i64);

                    let mut components = context.create_components();
                    components.assign(Component::Year, target_date.year());
                    components.assign(Component::Month, target_date.month() as i32);
                    components.assign(Component::Day, target_date.day() as i32);
                    components.assign(Component::Weekday, weekday as i32);

                    results.push(context.create_result(match_start, match_end, components, None));
                    start = match_end;
                    continue;
                }
            }

            // Try main pattern
            if let Ok(Some(caps)) = PATTERN.captures(search_text) {
                let full_match = caps.get(0).unwrap();
                let match_start = start + full_match.start();
                let match_end = start + full_match.end();

                let weekday_str = caps.name("weekday").map(|m| m.as_str()).unwrap_or_default();
                let modifier = caps.name("modifier").map(|m| m.as_str());

                if let Some(weekday) = dict::get_weekday(weekday_str) {
                    let ref_weekday = ref_date.weekday().num_days_from_monday() as i32;
                    let target_weekday = match weekday as i32 {
                        0 => 6, // Sunday
                        n => n - 1,
                    };

                    let days_diff = match modifier {
                        Some("次") | Some("来") | Some("来週") => {
                            // Next occurrence
                            let diff = target_weekday - ref_weekday;
                            if diff <= 0 { diff + 7 } else { diff }
                        }
                        Some("前") | Some("先") | Some("先週") => {
                            // Previous occurrence
                            let diff = target_weekday - ref_weekday;
                            if diff >= 0 { diff - 7 } else { diff }
                        }
                        Some("今") | Some("この") | Some("今週") | None => {
                            // Current week
                            target_weekday - ref_weekday
                        }
                        _ => target_weekday - ref_weekday,
                    };

                    let target_date = ref_date + Duration::days(days_diff as i64);

                    let mut components = context.create_components();
                    components.assign(Component::Year, target_date.year());
                    components.assign(Component::Month, target_date.month() as i32);
                    components.assign(Component::Day, target_date.day() as i32);
                    components.assign(Component::Weekday, weekday as i32);

                    results.push(context.create_result(match_start, match_end, components, None));
                    start = match_end;
                    continue;
                }
            }

            // No match - advance
            if let Some(c) = search_text.chars().next() {
                start += c.len_utf8();
            } else {
                break;
            }
        }

        Ok(results)
    }
}

impl Default for JAWeekdayParser {
    fn default() -> Self {
        Self::new()
    }
}
