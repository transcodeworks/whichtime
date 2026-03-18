//! Japanese slash date parser
//!
//! Handles Japanese slash date formats like:
//! - "2012/3/31" (YYYY/M/D)
//! - "12/31" (M/D)
//! - "8/5" (M/D)
//! - "2013/12/26~2014/1/7" (date range)

use crate::components::Component;
use crate::context::ParsingContext;
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use chrono::Datelike;
use regex::Regex;
use std::sync::LazyLock;

// Pattern for YYYY/M/D
static FULL_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(\d{4})/(\d{1,2})/(\d{1,2})").unwrap());

// Pattern for M/D
static SHORT_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(\d{1,2})/(\d{1,2})").unwrap());

// Pattern for date range
static RANGE_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(\d{4})/(\d{1,2})/(\d{1,2})\s*[~～ー-]\s*(\d{4})/(\d{1,2})/(\d{1,2})").unwrap()
});

/// Japanese slash date parser
pub struct JASlashDateParser;

impl JASlashDateParser {
    pub fn new() -> Self {
        Self
    }

    fn is_valid_date(year: i32, month: i32, day: i32) -> bool {
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
}

impl Parser for JASlashDateParser {
    fn name(&self) -> &'static str {
        "JASlashDateParser"
    }

    fn should_apply(&self, context: &ParsingContext) -> bool {
        context.text.contains('/') && context.text.bytes().any(|b| b.is_ascii_digit())
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();
        let ref_date = context.reference.instant;

        // Try range pattern first
        for mat in RANGE_PATTERN.find_iter(context.text) {
            let matched_text = mat.as_str();
            let index = mat.start();

            let Some(caps) = RANGE_PATTERN.captures(matched_text) else {
                continue;
            };

            let year1: i32 = caps
                .get(1)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);
            let month1: i32 = caps
                .get(2)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);
            let day1: i32 = caps
                .get(3)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);

            let year2: i32 = caps
                .get(4)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);
            let month2: i32 = caps
                .get(5)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);
            let day2: i32 = caps
                .get(6)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);

            if Self::is_valid_date(year1, month1, day1) && Self::is_valid_date(year2, month2, day2)
            {
                let mut components = context.create_components();
                components.assign(Component::Year, year1);
                components.assign(Component::Month, month1);
                components.assign(Component::Day, day1);

                let mut end_comp = context.create_components();
                end_comp.assign(Component::Year, year2);
                end_comp.assign(Component::Month, month2);
                end_comp.assign(Component::Day, day2);

                results.push(context.create_result(
                    index,
                    index + matched_text.len(),
                    components,
                    Some(end_comp),
                ));
            }
        }

        // If we found ranges, don't look for individual dates that overlap
        if !results.is_empty() {
            return Ok(results);
        }

        // Try full YYYY/M/D pattern first
        for mat in FULL_PATTERN.find_iter(context.text) {
            let matched_text = mat.as_str();
            let index = mat.start();

            let Some(caps) = FULL_PATTERN.captures(matched_text) else {
                continue;
            };

            let year: i32 = caps
                .get(1)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);
            let month: i32 = caps
                .get(2)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);
            let day: i32 = caps
                .get(3)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);

            if !Self::is_valid_date(year, month, day) {
                continue;
            }

            let mut components = context.create_components();
            components.assign(Component::Year, year);
            components.assign(Component::Month, month);
            components.assign(Component::Day, day);

            results.push(context.create_result(
                index,
                index + matched_text.len(),
                components,
                None,
            ));
        }

        // Then match M/D pattern (without year), avoiding overlaps with existing matches
        for mat in SHORT_PATTERN.find_iter(context.text) {
            let matched_text = mat.as_str();
            let index = mat.start();
            let end_index = index + matched_text.len();

            if results
                .iter()
                .any(|r| index < r.end_index && end_index > r.index)
            {
                continue;
            }

            let Some(caps) = SHORT_PATTERN.captures(matched_text) else {
                continue;
            };

            let month: i32 = caps
                .get(1)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);
            let day: i32 = caps
                .get(2)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);
            let year = ref_date.year();

            if !Self::is_valid_date(year, month, day) {
                continue;
            }

            let mut components = context.create_components();
            components.imply(Component::Year, year);
            components.assign(Component::Month, month);
            components.assign(Component::Day, day);

            results.push(context.create_result(index, end_index, components, None));
        }

        Ok(results)
    }
}

impl Default for JASlashDateParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dictionaries::Locale;
    use crate::results::ReferenceWithTimezone;
    use chrono::Local;

    #[test]
    fn parses_full_slash_date() {
        let parser = JASlashDateParser::new();
        let reference = ReferenceWithTimezone::new(Local::now(), None);
        let context = ParsingContext::with_locale("2012/3/31", &reference, Locale::Ja);
        let results = parser.parse(&context).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].text, "2012/3/31");
    }
}
