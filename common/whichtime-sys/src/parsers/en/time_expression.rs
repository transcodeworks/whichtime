//! Time expression parser: 3:30 PM, 15:00, 10:30:45, etc.

use crate::components::Component;
use crate::context::ParsingContext;
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use crate::types::Meridiem;
use regex::Regex;
use std::sync::LazyLock;

// Time pattern: HH:MM[:SS] [AM/PM] or HH[h]MM
static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)(?:^|[^\d])(\d{1,2})(?::(\d{2}))?(?::(\d{2}))?(?:\s*(a\.?m\.?|p\.?m\.?))?(?:[^\d]|$)",
    )
    .unwrap()
});

// 24-hour time pattern
static PATTERN_24H: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:^|[^\d])(\d{1,2}):(\d{2})(?::(\d{2}))?(?:[^\d]|$)").unwrap()
});

/// Parser for English clock-time expressions.
pub struct TimeExpressionParser;

impl Parser for TimeExpressionParser {
    fn name(&self) -> &'static str {
        "TimeExpressionParser"
    }

    fn should_apply(&self, context: &ParsingContext) -> bool {
        // Must contain digits and either : or AM/PM
        let text = context.lower_text();
        text.bytes().any(|b| b.is_ascii_digit())
            && (text.contains(':') || text.contains("am") || text.contains("pm"))
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();

        // Try 24-hour pattern first
        for mat in PATTERN_24H.find_iter(context.text) {
            let matched_text = mat.as_str();
            let index = mat.start();

            let Some(caps) = PATTERN_24H.captures(matched_text) else {
                continue;
            };

            let hour: i32 = caps
                .get(1)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(-1);
            let minute: i32 = caps
                .get(2)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);
            let second: i32 = caps
                .get(3)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);

            if !(0..=23).contains(&hour) || !(0..=59).contains(&minute) {
                continue;
            }

            let mut components = context.create_components();
            components.assign(Component::Hour, hour);
            components.assign(Component::Minute, minute);
            if caps.get(3).is_some() {
                components.assign(Component::Second, second);
            }

            // Set meridiem based on hour
            if hour >= 12 {
                components.assign(Component::Meridiem, Meridiem::PM as i32);
            } else {
                components.assign(Component::Meridiem, Meridiem::AM as i32);
            }

            let actual_start = matched_text.find(|c: char| c.is_ascii_digit()).unwrap_or(0);
            let actual_end = matched_text
                .rfind(|c: char| c.is_ascii_digit() || c == 'm' || c == 'M')
                .map(|i| i + matched_text[i..].chars().next().map_or(1, char::len_utf8))
                .unwrap_or(matched_text.len());

            results.push(context.create_result(
                index + actual_start,
                index + actual_end,
                components,
                None,
            ));
        }

        // Try AM/PM pattern
        for mat in PATTERN.find_iter(context.text) {
            let matched_text = mat.as_str();
            let index = mat.start();

            // Skip if already matched
            if results
                .iter()
                .any(|r| r.index <= index && r.end_index > index)
            {
                continue;
            }

            let Some(caps) = PATTERN.captures(matched_text) else {
                continue;
            };

            let meridiem_match = caps.get(4);

            // Only process if has AM/PM (otherwise 24h pattern should have caught it)
            if meridiem_match.is_none() && caps.get(2).is_none() {
                continue;
            }

            let mut hour: i32 = caps
                .get(1)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(-1);
            let minute: i32 = caps
                .get(2)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);
            let second: i32 = caps
                .get(3)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);

            if !(0..=23).contains(&hour) || minute > 59 {
                continue;
            }

            // Handle AM/PM
            let meridiem = if let Some(m) = meridiem_match {
                let m_str = m.as_str().to_lowercase();
                if m_str.starts_with('p') {
                    if hour > 12 {
                        continue; // Invalid: 14PM
                    }
                    if hour < 12 {
                        hour += 12;
                    }
                    Some(Meridiem::PM)
                } else {
                    if hour > 12 {
                        continue; // Invalid: 14AM
                    }
                    if hour == 12 {
                        hour = 0;
                    }
                    Some(Meridiem::AM)
                }
            } else if hour >= 12 {
                Some(Meridiem::PM)
            } else {
                Some(Meridiem::AM)
            };

            let mut components = context.create_components();
            components.assign(Component::Hour, hour);
            components.assign(Component::Minute, minute);
            if caps.get(3).is_some() {
                components.assign(Component::Second, second);
            }
            if let Some(m) = meridiem {
                components.assign(Component::Meridiem, m as i32);
            }

            let actual_start = matched_text.find(|c: char| c.is_ascii_digit()).unwrap_or(0);
            let actual_end = matched_text
                .rfind(|c: char| c.is_ascii_alphanumeric())
                .map(|i| i + matched_text[i..].chars().next().map_or(1, char::len_utf8))
                .unwrap_or(matched_text.len());

            results.push(context.create_result(
                index + actual_start,
                index + actual_end,
                components,
                None,
            ));
        }

        Ok(results)
    }
}
