//! Italian month name parser
//!
//! Handles Italian date formats with month names like:
//! - Little endian: "10 Agosto 2012", "il 10 Agosto"
//! - Middle endian: "Agosto 10, 2012", "Agosto 2017"
//! - Date ranges: "10 - 22 Agosto 2012"
//! - Combined with time: "12 Luglio alle 19:00"

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::it as dict;
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use crate::scanner::TokenType;
use chrono::Datelike;
use fancy_regex::Regex;
use std::sync::LazyLock;

// Little endian pattern: "10 Agosto 2012", "il 10 Agosto", "10 - 22 Agosto 2012"
static LITTLE_ENDIAN_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)(?:il\s+)?(?P<day>\d{1,2})(?:\s*(?:-|–|a)\s*(?P<end_day>\d{1,2}))?\s+(?P<month>gennaio|febbraio|febraio|marzo|aprile|maggio|giugno|luglio|agosto|settembre|ottobre|novembre|dicembre|gen|feb|febb|mar|apr|mag|giu|lug|lugl|ago|set|sett|ott|nov|dic|dice)\.?(?:\s+(?P<year>\d{1,4})(?![\d:]))?(?:\s+(?:alle?\s+)?(?P<hour>\d{1,2})(?:[:\.](?P<minute>\d{2}))?)?"
    ).unwrap()
});

// Middle endian pattern: "Agosto 10, 2012", "Agosto 2017", "Agosto 10"
// Note: "Agosto 10" (day without year) needs to be distinguished from year-only "Agosto 2017"
static MIDDLE_ENDIAN_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)(?P<month>gennaio|febbraio|febraio|marzo|aprile|maggio|giugno|luglio|agosto|settembre|ottobre|novembre|dicembre|gen|feb|febb|mar|apr|mag|giu|lug|lugl|ago|set|sett|ott|nov|dic|dice)\.?(?:\s+(?P<day>\d{1,2})(?:,\s*(?P<year>\d{4}))?|\s+(?P<year_only>\d{4}))(?![\d])"
    ).unwrap()
});

/// Italian month name parser
pub struct ITMonthNameParser;

impl ITMonthNameParser {
    pub fn new() -> Self {
        Self
    }

    fn is_valid_date(year: i32, month: u32, day: u32) -> bool {
        if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
            return false;
        }
        let days_in_month = match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                    29
                } else {
                    28
                }
            }
            _ => return false,
        };
        day <= days_in_month
    }

    fn parse_year(year_str: &str) -> i32 {
        let year: i32 = year_str.parse().unwrap_or(0);
        if year < 100 {
            if year > 50 { 1900 + year } else { 2000 + year }
        } else {
            year
        }
    }
}

impl Parser for ITMonthNameParser {
    fn name(&self) -> &'static str {
        "ITMonthNameParser"
    }

    fn should_apply(&self, context: &ParsingContext) -> bool {
        context.has_token_type(TokenType::Month) || context.text.bytes().any(|b| b.is_ascii_digit())
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();
        let ref_date = context.reference.instant;

        let mut start = 0;
        while start < context.text.len() {
            let search_text = &context.text[start..];

            // Try little endian pattern first (more specific)
            if let Ok(Some(caps)) = LITTLE_ENDIAN_PATTERN.captures(search_text)
                && let (Some(d), Some(m)) = (caps.name("day"), caps.name("month"))
            {
                let day: u32 = d.as_str().parse().unwrap_or(0);
                let month = dict::get_month(&m.as_str().to_lowercase()).unwrap_or(0);

                let full_match = caps.get(0).unwrap();
                let matched_text = full_match.as_str();

                // Skip "il " prefix for the result text
                let prefix_offset = if matched_text.to_lowercase().starts_with("il ") {
                    3
                } else {
                    0
                };

                let match_start = start + full_match.start() + prefix_offset;
                let match_end = start + full_match.end();

                let year = caps.name("year").map(|y| Self::parse_year(y.as_str()));

                let actual_year = year.unwrap_or(ref_date.year());

                // Validate the date
                if day == 0 || month == 0 || !Self::is_valid_date(actual_year, month, day) {
                    start = match_end;
                    continue;
                }

                // Check if time is present
                let has_time = caps.name("hour").is_some();

                let mut components = context.create_components();
                if let Some(y) = year {
                    components.assign(Component::Year, y);
                } else if has_time {
                    // When time is specified, assign year to prevent ForwardDateRefiner from moving it
                    components.assign(Component::Year, ref_date.year());
                } else {
                    components.imply(Component::Year, ref_date.year());
                }
                components.assign(Component::Month, month as i32);
                components.assign(Component::Day, day as i32);

                // Handle time if present
                if let Some(hour_match) = caps.name("hour") {
                    let hour: i32 = hour_match.as_str().parse().unwrap_or(0);
                    let minute: i32 = caps
                        .name("minute")
                        .and_then(|m| m.as_str().parse().ok())
                        .unwrap_or(0);
                    components.assign(Component::Hour, hour);
                    components.assign(Component::Minute, minute);
                }

                // Handle date range within same month
                let end_comp = if let Some(end_day_match) = caps.name("end_day") {
                    let end_day: u32 = end_day_match.as_str().parse().unwrap_or(0);
                    if end_day > 0 && end_day <= 31 {
                        let mut ec = context.create_components();
                        if let Some(y) = year {
                            ec.assign(Component::Year, y);
                        } else {
                            ec.imply(Component::Year, ref_date.year());
                        }
                        ec.assign(Component::Month, month as i32);
                        ec.assign(Component::Day, end_day as i32);
                        Some(ec)
                    } else {
                        None
                    }
                } else {
                    None
                };

                results.push(context.create_result(match_start, match_end, components, end_comp));
                start = match_end;
                continue;
            }

            // Try middle endian pattern
            if let Ok(Some(caps)) = MIDDLE_ENDIAN_PATTERN.captures(search_text)
                && let Some(m) = caps.name("month")
            {
                let month = dict::get_month(&m.as_str().to_lowercase()).unwrap_or(0);

                if month > 0 {
                    let full_match = caps.get(0).unwrap();
                    let match_start = start + full_match.start();
                    let match_end = start + full_match.end();

                    let day: u32 = caps
                        .name("day")
                        .and_then(|d| d.as_str().parse().ok())
                        .unwrap_or(1); // Default to 1st if only month/year

                    let year = caps
                        .name("year")
                        .or(caps.name("year_only"))
                        .map(|y| Self::parse_year(y.as_str()));

                    let actual_year = year.unwrap_or(ref_date.year());

                    // Validate the date
                    if !Self::is_valid_date(actual_year, month, day) {
                        start = match_end;
                        continue;
                    }

                    let mut components = context.create_components();
                    if let Some(y) = year {
                        components.assign(Component::Year, y);
                    } else {
                        components.imply(Component::Year, ref_date.year());
                    }
                    components.assign(Component::Month, month as i32);
                    components.assign(Component::Day, day as i32);

                    results.push(context.create_result(match_start, match_end, components, None));
                    start = match_end;
                    continue;
                }
            }

            // No match at this position - advance to next character (UTF-8 safe)
            if let Some(c) = search_text.chars().next() {
                start += c.len_utf8();
            } else {
                break;
            }
        }

        Ok(results)
    }
}

impl Default for ITMonthNameParser {
    fn default() -> Self {
        Self::new()
    }
}
