//! Slash date format parser: MM/DD/YYYY or DD/MM/YYYY

use crate::components::Component;
use crate::context::ParsingContext;
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use regex::Regex;
use std::sync::LazyLock;

// Pattern: M/D/YYYY, MM/DD/YYYY, M-D-YYYY, etc.
static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:^|[^\d])(\d{1,2})[/\-.](\d{1,2})[/\-.](\d{2,4})(?:[^\d]|$)").unwrap()
});

/// Parser for slash- or dash-separated numeric dates.
pub struct SlashDateParser {
    /// If true, interpret as DD/MM/YYYY (little endian, UK style)
    /// If false, interpret as MM/DD/YYYY (middle endian, US style)
    little_endian: bool,
}

impl SlashDateParser {
    /// Create a slash-date parser with either little-endian or middle-endian ordering.
    pub fn new(little_endian: bool) -> Self {
        Self { little_endian }
    }

    fn parse_year(year_str: &str) -> Option<i32> {
        let year: i32 = year_str.parse().ok()?;
        Some(if year < 100 {
            if year > 50 { 1900 + year } else { 2000 + year }
        } else {
            year
        })
    }
}

impl Parser for SlashDateParser {
    fn name(&self) -> &'static str {
        "SlashDateParser"
    }

    fn should_apply(&self, context: &ParsingContext) -> bool {
        let text = context.text;
        // Quick check: must contain / or - and digits
        // For non-English locales, we might support space as separator? But regex doesn't.
        // Also, we might need to check if text is long enough.
        (text.contains('/') || text.contains('-') || text.contains('.'))
            && text.bytes().any(|b| b.is_ascii_digit())
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();

        for mat in PATTERN.find_iter(context.text) {
            let matched_text = mat.as_str();
            let index = mat.start();

            let Some(caps) = PATTERN.captures(matched_text) else {
                continue;
            };

            let first: i32 = caps
                .get(1)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);
            let second: i32 = caps
                .get(2)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);
            let year = caps
                .get(3)
                .and_then(|m| Self::parse_year(m.as_str()))
                .unwrap_or(0);

            let (month, day) = if self.little_endian {
                (second, first) // DD/MM/YYYY
            } else {
                (first, second) // MM/DD/YYYY
            };

            // Validate
            if !(1..=12).contains(&month) || !(1..=31).contains(&day) || year == 0 {
                continue;
            }

            let mut components = context.create_components();
            components.assign(Component::Year, year);
            components.assign(Component::Month, month);
            components.assign(Component::Day, day);

            if !components.is_valid_date() {
                continue;
            }

            // Trim the leading/trailing non-digit characters from the matched text
            let actual_start = matched_text.find(|c: char| c.is_ascii_digit()).unwrap_or(0);
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
