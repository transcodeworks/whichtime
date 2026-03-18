//! French slash date format parser: DD/MM/YYYY or DD/MM
//!
//! Handles French date formats like:
//! - "8/2/2016" (DD/MM/YYYY)
//! - "le 8/2" (DD/MM without year)

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::fr as dict;
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use chrono::Datelike;
use regex::Regex;
use std::sync::LazyLock;

// Pattern with optional weekday and year: [Weekday] DD/MM/YYYY or DD/MM
static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:^|[^\wàâäéèêëïîôùûüÿçœæ])(?:(lundi|mardi|mercredi|jeudi|vendredi|samedi|dimanche)\s+)?(\d{1,2})[/\-.](\d{1,2})(?:[/\-.](\d{2,4}))?(?:[^\d]|$)").unwrap()
});

// Pattern to check if followed by AM/PM (time indicator)
static AM_PM_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)^\s*(?:a\.?m\.?|p\.?m\.?)").unwrap());

pub struct FRSlashDateParser;

impl FRSlashDateParser {
    pub fn new() -> Self {
        Self
    }

    fn parse_year(year_str: &str) -> Option<i32> {
        let year: i32 = year_str.parse().ok()?;
        Some(if year < 100 {
            if year > 50 { 1900 + year } else { 2000 + year }
        } else {
            year
        })
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

impl Parser for FRSlashDateParser {
    fn name(&self) -> &'static str {
        "FRSlashDateParser"
    }

    fn should_apply(&self, context: &ParsingContext) -> bool {
        let text = context.text;
        // Quick check: must contain / or - and digits
        (text.contains('/') || text.contains('-') || text.contains('.'))
            && text.bytes().any(|b| b.is_ascii_digit())
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();
        let ref_date = context.reference.instant;

        for mat in PATTERN.find_iter(context.text) {
            let matched_text = mat.as_str();
            let index = mat.start();
            let match_end = mat.end();

            // Check if followed by AM/PM - if so, skip (it's a time, not a date)
            let remaining = &context.text[match_end..];
            if AM_PM_PATTERN.is_match(remaining) {
                continue;
            }

            let Some(caps) = PATTERN.captures(matched_text) else {
                continue;
            };

            // Optional weekday
            let weekday = caps
                .get(1)
                .and_then(|m| dict::get_weekday(&m.as_str().to_lowercase()));

            // DD/MM/YYYY format (little endian)
            let day: i32 = caps
                .get(2)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);
            let month: i32 = caps
                .get(3)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);

            // Year is optional
            let year = caps.get(4).and_then(|m| Self::parse_year(m.as_str()));

            // Validate
            let actual_year = year.unwrap_or(ref_date.year());
            if !Self::is_valid_date(actual_year, month, day) {
                continue;
            }

            let mut components = context.create_components();
            if let Some(y) = year {
                components.assign(Component::Year, y);
            } else {
                // No year provided - imply current year, let ForwardDateRefiner handle it
                components.imply(Component::Year, ref_date.year());
            }
            components.assign(Component::Month, month);
            components.assign(Component::Day, day);

            // Add weekday if present
            if let Some(wd) = weekday {
                components.assign(Component::Weekday, wd as i32);
            }

            // Trim the leading/trailing non-alphanumeric characters from the matched text
            let actual_start = matched_text
                .find(|c: char| c.is_ascii_alphanumeric() || c.is_alphabetic())
                .unwrap_or(0);
            let actual_end = matched_text
                .rfind(|c: char| c.is_ascii_digit())
                .map(|i| i + matched_text[i..].chars().next().map_or(1, char::len_utf8))
                .unwrap_or(matched_text.len());
            let clean_text = &matched_text[actual_start..actual_end];

            results.push(context.create_result(
                index + actual_start,
                index + actual_start + clean_text.len(),
                components,
                None,
            ));
        }

        Ok(results)
    }
}

impl Default for FRSlashDateParser {
    fn default() -> Self {
        Self::new()
    }
}
