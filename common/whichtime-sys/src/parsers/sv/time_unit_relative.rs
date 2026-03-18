//! Swedish time unit relative parser
//!
//! Handles Swedish relative time expressions like:
//! - "nästa 2 veckor" (next 2 weeks)
//! - "förra 2 veckor" (past 2 weeks)
//! - "efter en timme" (after one hour)
//! - "+15 minuter" (plus 15 minutes)
//! - "-3år" (minus 3 years)

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::sv::{get_time_unit, parse_number_pattern};
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use crate::types::{Duration, TimeUnit, add_duration};
use chrono::{Datelike, Timelike};
use fancy_regex::Regex;
use std::sync::LazyLock;

// Pattern for "nästa/förra N units"
static RELATIVE_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)(?<![a-zA-ZåäöÅÄÖ])(?P<modifier>nästa|nasta|förra|forra|kommande)\s+(?P<num>\d+|en|ett|två|tva|tre|fyra|fem|sex|sju|åtta|atta|nio|tio|elva|tolv)\s+(?P<unit>sekunder?|minuter?|timm(?:ar|e)?|dagar?|veckor?|månader?|manader?|år|ar)(?![a-zA-ZåäöÅÄÖ])"
    ).unwrap()
});

// Pattern for "efter N units" (after N units)
static EFTER_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)(?<![a-zA-ZåäöÅÄÖ])efter\s+(?P<num>\d+|en|ett|två|tva|tre|fyra|fem|sex|sju|åtta|atta|nio|tio|elva|tolv)\s+(?P<unit>sekunder?|minuter?|timm(?:ar|e)?|dagar?|veckor?|månader?|manader?|år|ar)(?![a-zA-ZåäöÅÄÖ])"
    ).unwrap()
});

// Pattern for "+/-N units"
static SIGN_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?<![a-zA-ZåäöÅÄÖ0-9])(?P<sign>[+\-])(?P<num>\d+)\s*(?P<unit>sekunder?|minuter?|timm(?:ar|e)?|dagar?|veckor?|månader?|manader?|år|ar)(?![a-zA-ZåäöÅÄÖ])"
    ).unwrap()
});

/// Swedish time unit relative parser
pub struct SVTimeUnitRelativeParser;

impl SVTimeUnitRelativeParser {
    pub fn new() -> Self {
        Self
    }

    fn parse_unit(unit_str: &str) -> Option<TimeUnit> {
        let lower = unit_str.to_lowercase();
        get_time_unit(&lower)
    }
}

impl Default for SVTimeUnitRelativeParser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser for SVTimeUnitRelativeParser {
    fn name(&self) -> &'static str {
        "SVTimeUnitRelativeParser"
    }

    fn should_apply(&self, _context: &ParsingContext) -> bool {
        true
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();
        let ref_date = context.reference.instant;

        // Parse "nästa/förra N units" patterns
        let mut start = 0;
        while start < context.text.len() {
            let search_text = &context.text[start..];
            let captures = match RELATIVE_PATTERN.captures(search_text) {
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

            let modifier = captures
                .name("modifier")
                .map(|m| m.as_str().to_lowercase())
                .unwrap_or_default();
            let num_str = captures.name("num").map(|m| m.as_str()).unwrap_or("1");
            let unit_str = captures
                .name("unit")
                .map(|m| m.as_str())
                .unwrap_or_default();

            let num = parse_number_pattern(num_str);
            if let Some(unit) = Self::parse_unit(unit_str) {
                let is_past = modifier.starts_with("förra") || modifier.starts_with("forra");
                let multiplier = if is_past { -1.0 } else { 1.0 };
                let adjusted_num = num * multiplier;

                let mut duration = Duration::new();
                match unit {
                    TimeUnit::Second => duration.second = Some(adjusted_num),
                    TimeUnit::Minute => duration.minute = Some(adjusted_num),
                    TimeUnit::Hour => duration.hour = Some(adjusted_num),
                    TimeUnit::Day => duration.day = Some(adjusted_num),
                    TimeUnit::Week => duration.week = Some(adjusted_num),
                    TimeUnit::Month => duration.month = Some(adjusted_num),
                    TimeUnit::Year => duration.year = Some(adjusted_num),
                    _ => {}
                }

                let target_date = add_duration(ref_date, &duration);

                let mut components = context.create_components();
                components.assign(Component::Year, target_date.year());
                components.assign(Component::Month, target_date.month() as i32);
                components.assign(Component::Day, target_date.day() as i32);

                if duration.has_time_component() {
                    components.assign(Component::Hour, target_date.hour() as i32);
                    components.assign(Component::Minute, target_date.minute() as i32);
                    components.assign(Component::Second, target_date.second() as i32);
                } else {
                    components.imply(Component::Hour, ref_date.hour() as i32);
                    components.imply(Component::Minute, ref_date.minute() as i32);
                }

                results.push(context.create_result(match_start, match_end, components, None));
            }

            start = match_end;
        }

        // Parse "efter N units" patterns
        start = 0;
        while start < context.text.len() {
            let search_text = &context.text[start..];
            let captures = match EFTER_PATTERN.captures(search_text) {
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

            // Skip if overlaps with existing results
            let overlaps = results.iter().any(|r| {
                (match_start >= r.index && match_start < r.index + r.text.len())
                    || (r.index >= match_start && r.index < match_end)
            });
            if overlaps {
                start = match_end;
                continue;
            }

            let num_str = captures.name("num").map(|m| m.as_str()).unwrap_or("1");
            let unit_str = captures
                .name("unit")
                .map(|m| m.as_str())
                .unwrap_or_default();

            let num = parse_number_pattern(num_str);
            if let Some(unit) = Self::parse_unit(unit_str) {
                let mut duration = Duration::new();
                match unit {
                    TimeUnit::Second => duration.second = Some(num),
                    TimeUnit::Minute => duration.minute = Some(num),
                    TimeUnit::Hour => duration.hour = Some(num),
                    TimeUnit::Day => duration.day = Some(num),
                    TimeUnit::Week => duration.week = Some(num),
                    TimeUnit::Month => duration.month = Some(num),
                    TimeUnit::Year => duration.year = Some(num),
                    _ => {}
                }

                let target_date = add_duration(ref_date, &duration);

                let mut components = context.create_components();
                components.assign(Component::Year, target_date.year());
                components.assign(Component::Month, target_date.month() as i32);
                components.assign(Component::Day, target_date.day() as i32);

                if duration.has_time_component() {
                    components.assign(Component::Hour, target_date.hour() as i32);
                    components.assign(Component::Minute, target_date.minute() as i32);
                    components.assign(Component::Second, target_date.second() as i32);
                } else {
                    components.imply(Component::Hour, ref_date.hour() as i32);
                    components.imply(Component::Minute, ref_date.minute() as i32);
                }

                results.push(context.create_result(match_start, match_end, components, None));
            }

            start = match_end;
        }

        // Parse "+/-N units" patterns
        start = 0;
        while start < context.text.len() {
            let search_text = &context.text[start..];
            let captures = match SIGN_PATTERN.captures(search_text) {
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

            // Skip if overlaps with existing results
            let overlaps = results.iter().any(|r| {
                (match_start >= r.index && match_start < r.index + r.text.len())
                    || (r.index >= match_start && r.index < match_end)
            });
            if overlaps {
                start = match_end;
                continue;
            }

            let sign = captures.name("sign").map(|m| m.as_str()).unwrap_or("+");
            let num_str = captures.name("num").map(|m| m.as_str()).unwrap_or("1");
            let unit_str = captures
                .name("unit")
                .map(|m| m.as_str())
                .unwrap_or_default();

            let num: f64 = num_str.parse().unwrap_or(0.0);
            let adjusted_num = if sign == "-" { -num } else { num };

            if let Some(unit) = Self::parse_unit(unit_str) {
                let mut duration = Duration::new();
                match unit {
                    TimeUnit::Second => duration.second = Some(adjusted_num),
                    TimeUnit::Minute => duration.minute = Some(adjusted_num),
                    TimeUnit::Hour => duration.hour = Some(adjusted_num),
                    TimeUnit::Day => duration.day = Some(adjusted_num),
                    TimeUnit::Week => duration.week = Some(adjusted_num),
                    TimeUnit::Month => duration.month = Some(adjusted_num),
                    TimeUnit::Year => duration.year = Some(adjusted_num),
                    _ => {}
                }

                let target_date = add_duration(ref_date, &duration);

                let mut components = context.create_components();
                components.assign(Component::Year, target_date.year());
                components.assign(Component::Month, target_date.month() as i32);
                components.assign(Component::Day, target_date.day() as i32);

                // For signed operations, always include time
                components.assign(Component::Hour, target_date.hour() as i32);
                components.assign(Component::Minute, target_date.minute() as i32);

                results.push(context.create_result(match_start, match_end, components, None));
            }

            start = match_end;
        }

        Ok(results)
    }
}
