//! Month name parser: "January 15, 2024", "15 January 2024", etc.

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::en::{get_month, parse_ordinal_pattern};
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use crate::scanner::TokenType;
use chrono::Datelike;
use regex::Regex;
use std::sync::LazyLock;

// Month name patterns
static MONTH_NAME_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)(?:^|\W)((?:jan(?:uary)?|feb(?:ruary)?|mar(?:ch)?|apr(?:il)?|may|june?|july?|aug(?:ust)?|sep(?:t(?:ember)?)?|oct(?:ober)?|nov(?:ember)?|dec(?:ember)?))\s*(?:(\d{1,2})(?:st|nd|rd|th)?\s*,?\s*)?(\d{2,4})?"
    ).unwrap()
});

// Little endian: "15 January 2024" or "15th January 2024"
static LITTLE_ENDIAN_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)(?:^|\W)(\d{1,2})(?:st|nd|rd|th)?\s+(jan(?:uary)?|feb(?:ruary)?|mar(?:ch)?|apr(?:il)?|may|june?|july?|aug(?:ust)?|sep(?:t(?:ember)?)?|oct(?:ober)?|nov(?:ember)?|dec(?:ember)?)\s*,?\s*(\d{2,4})?"
    ).unwrap()
});

/// Parser for English month-name date expressions.
pub struct MonthNameParser;

impl Parser for MonthNameParser {
    fn name(&self) -> &'static str {
        "MonthNameParser"
    }

    fn should_apply(&self, context: &ParsingContext) -> bool {
        context.has_token_type(TokenType::Month)
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();
        let ref_date = context.reference.instant;

        // Try little endian first (15 January 2024)
        for mat in LITTLE_ENDIAN_PATTERN.find_iter(context.text) {
            let matched_text = mat.as_str();
            let index = mat.start();

            let Some(caps) = LITTLE_ENDIAN_PATTERN.captures(matched_text) else {
                continue;
            };

            let day: i32 = caps
                .get(1)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);
            let month_str = caps
                .get(2)
                .map(|m| m.as_str().to_lowercase())
                .unwrap_or_default();
            let year_str = caps.get(3).map(|m| m.as_str());

            let Some(month) = get_month(&month_str) else {
                continue;
            };

            let year = if let Some(y) = year_str {
                parse_year(y)
            } else {
                ref_date.year()
            };

            if !(1..=31).contains(&day) {
                continue;
            }

            let mut components = context.create_components();
            components.assign(Component::Year, year);
            components.assign(Component::Month, month as i32);
            components.assign(Component::Day, day);

            if !components.is_valid_date() {
                continue;
            }

            // Find actual text bounds
            let actual_start = matched_text
                .find(|c: char| c.is_alphanumeric())
                .unwrap_or(0);
            results.push(context.create_result(
                index + actual_start,
                index + matched_text.len(),
                components,
                None,
            ));
        }

        // Try month-first pattern (January 15, 2024)
        for mat in MONTH_NAME_PATTERN.find_iter(context.text) {
            let matched_text = mat.as_str();
            let index = mat.start();

            // Skip if we already matched this region
            if results
                .iter()
                .any(|r| r.index <= index && r.end_index > index)
            {
                continue;
            }

            let Some(caps) = MONTH_NAME_PATTERN.captures(matched_text) else {
                continue;
            };

            let month_str = caps
                .get(1)
                .map(|m| m.as_str().to_lowercase())
                .unwrap_or_default();
            let day_str = caps.get(2).map(|m| m.as_str());
            let year_str = caps.get(3).map(|m| m.as_str());

            let Some(month) = get_month(&month_str) else {
                continue;
            };

            let day = if let Some(d) = day_str {
                parse_ordinal_pattern(d).unwrap_or(1) as i32
            } else {
                1 // Default to 1st of month
            };

            let year = if let Some(y) = year_str {
                parse_year(y)
            } else {
                // Determine year based on whether month is in future or past
                let current_month = ref_date.month() as i32;
                if (month as i32) < current_month {
                    ref_date.year() + 1
                } else {
                    ref_date.year()
                }
            };

            if !(1..=31).contains(&day) {
                continue;
            }

            let mut components = context.create_components();
            components.assign(Component::Year, year);
            components.assign(Component::Month, month as i32);
            components.assign(Component::Day, day);

            if !components.is_valid_date() {
                continue;
            }

            let actual_start = matched_text
                .find(|c: char| c.is_alphanumeric())
                .unwrap_or(0);
            results.push(context.create_result(
                index + actual_start,
                index + matched_text.len(),
                components,
                None,
            ));
        }

        Ok(results)
    }
}

fn parse_year(s: &str) -> i32 {
    let year: i32 = s.parse().unwrap_or(0);
    if year < 100 {
        if year > 50 { 1900 + year } else { 2000 + year }
    } else {
        year
    }
}
