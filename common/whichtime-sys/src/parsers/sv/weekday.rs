//! Swedish weekday parser
//!
//! Handles Swedish weekday expressions like:
//! - "måndag", "tisdag", etc.
//! - "på måndag" (on Monday)
//! - "nästa måndag" (next Monday)
//! - "förra måndag" (last Monday)

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::sv::get_weekday;
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use chrono::{Datelike, Duration, Weekday as ChronoWeekday};
use fancy_regex::Regex;
use std::sync::LazyLock;

static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)(?<![a-zA-ZåäöÅÄÖ])(?:(?P<modifier>nästa|nasta|förra|forra|kommande|denna|i)\s+)?(?P<prep>på\s+)?(?P<weekday>söndag|sondag|måndag|mandag|tisdag|onsdag|torsdag|fredag|lördag|lordag|sön\.?|son\.?|mån\.?|man\.?|tis\.?|ons\.?|tor\.?|tors\.?|fre\.?|lör\.?|lor\.?)(?![a-zA-ZåäöÅÄÖ])"
    ).unwrap()
});

/// Swedish weekday parser
pub struct SVWeekdayParser;

impl SVWeekdayParser {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SVWeekdayParser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser for SVWeekdayParser {
    fn name(&self) -> &'static str {
        "SVWeekdayParser"
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

            let modifier = captures.name("modifier").map(|m| m.as_str().to_lowercase());
            let weekday_str = captures
                .name("weekday")
                .map(|m| m.as_str().to_lowercase())
                .unwrap_or_default();

            // Clean weekday string (remove trailing dots)
            let clean_weekday = weekday_str.trim_end_matches('.');

            let Some(weekday) = get_weekday(clean_weekday) else {
                start = match_end;
                continue;
            };

            let mut components = context.create_components();

            // Convert our weekday to chrono weekday for calculation
            let target_weekday = match weekday {
                crate::types::Weekday::Sunday => ChronoWeekday::Sun,
                crate::types::Weekday::Monday => ChronoWeekday::Mon,
                crate::types::Weekday::Tuesday => ChronoWeekday::Tue,
                crate::types::Weekday::Wednesday => ChronoWeekday::Wed,
                crate::types::Weekday::Thursday => ChronoWeekday::Thu,
                crate::types::Weekday::Friday => ChronoWeekday::Fri,
                crate::types::Weekday::Saturday => ChronoWeekday::Sat,
            };

            let current_weekday = ref_date.weekday();
            let current_day_num = current_weekday.num_days_from_sunday() as i64;
            let target_day_num = target_weekday.num_days_from_sunday() as i64;

            // Calculate days difference
            let mut days_diff = target_day_num - current_day_num;

            // Handle modifiers
            let target_date = match modifier.as_deref() {
                Some(m) if m.starts_with("nästa") || m.starts_with("nasta") || m == "kommande" => {
                    // Next occurrence (always in the future, at least 1 day ahead)
                    if days_diff <= 0 {
                        days_diff += 7;
                    }
                    ref_date + Duration::days(days_diff)
                }
                Some(m) if m.starts_with("förra") || m.starts_with("forra") => {
                    // Last occurrence (always in the past)
                    if days_diff >= 0 {
                        days_diff -= 7;
                    }
                    ref_date + Duration::days(days_diff)
                }
                _ => {
                    // Default: find the closest occurrence (prefer past for same day)
                    if days_diff > 0 {
                        days_diff -= 7;
                    }
                    ref_date + Duration::days(days_diff)
                }
            };

            components.assign(Component::Year, target_date.year());
            components.assign(Component::Month, target_date.month() as i32);
            components.assign(Component::Day, target_date.day() as i32);
            components.assign(Component::Weekday, weekday as i32);

            results.push(context.create_result(match_start, match_end, components, None));
            start = match_end;
        }

        Ok(results)
    }
}
