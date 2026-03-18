//! Italian year.month.day parser
//!
//! Handles Italian date format: YYYY.MM.DD

use crate::components::Component;
use crate::context::ParsingContext;
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use regex::Regex;
use std::sync::LazyLock;

// Pattern: YYYY.MM.DD
static PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?:^|[^\d])(\d{4})\.(\d{2})\.(\d{2})(?:[^\d]|$)").unwrap());

/// Italian year.month.day parser
pub struct ITYearMonthDayParser;

impl ITYearMonthDayParser {
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

impl Parser for ITYearMonthDayParser {
    fn name(&self) -> &'static str {
        "ITYearMonthDayParser"
    }

    fn should_apply(&self, context: &ParsingContext) -> bool {
        context.text.contains('.') && context.text.bytes().any(|b| b.is_ascii_digit())
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();

        for mat in PATTERN.find_iter(context.text) {
            let matched_text = mat.as_str();
            let index = mat.start();

            let Some(caps) = PATTERN.captures(matched_text) else {
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

            // Validate
            if !Self::is_valid_date(year, month, day) {
                continue;
            }

            let mut components = context.create_components();
            components.assign(Component::Year, year);
            components.assign(Component::Month, month);
            components.assign(Component::Day, day);

            // Trim the leading/trailing non-digit characters from the matched text
            let actual_start = matched_text.find(|c: char| c.is_ascii_digit()).unwrap_or(0);
            let actual_end = matched_text
                .rfind(|c: char| c.is_ascii_digit())
                .map(|i| i + 1)
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

impl Default for ITYearMonthDayParser {
    fn default() -> Self {
        Self::new()
    }
}
