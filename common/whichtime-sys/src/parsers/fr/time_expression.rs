//! French time expression parser
//!
//! Handles French time expressions like:
//! - "8h10", "8h10m"
//! - "8:10 PM"
//! - "8:10 - 12.32" (ranges)
//! - "de 8h à 10h" (ranges)

use crate::components::Component;
use crate::context::ParsingContext;
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use crate::types::Meridiem;
use fancy_regex::Regex;
use std::sync::LazyLock;

// French time pattern: handles 8h10, 8h10m, 8:10, 8:10 PM, etc.
// Uses negative lookbehind (?<!\d) to ensure hour is not preceded by a digit
static PRIMARY_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)(?<!\d)(?:(?:de\s+)?(?:à\s+|a\s+)?)?(\d{1,2})(?:h(\d{2})?m?|[:\.](\d{2}))(?::(\d{2}))?(?:\s*(a\.?m\.?|p\.?m\.?))?(?:\s*(?:à|a|[\-–~])\s*(\d{1,2})(?:h(\d{2})?m?|[:\.](\d{2}))?(?::(\d{2}))?(?:\s*(a\.?m\.?|p\.?m\.?))?)?(?![\d:a-zA-Z])"
    ).unwrap()
});

/// French time expression parser
pub struct FRTimeExpressionParser;

impl FRTimeExpressionParser {
    pub fn new() -> Self {
        Self
    }

    fn parse_meridiem(s: &str) -> Option<Meridiem> {
        let lower = s.to_lowercase();
        if lower.starts_with('p') {
            Some(Meridiem::PM)
        } else if lower.starts_with('a') {
            Some(Meridiem::AM)
        } else {
            None
        }
    }

    fn adjust_hour(hour: i32, meridiem: Option<Meridiem>) -> i32 {
        match meridiem {
            Some(Meridiem::PM) => {
                if hour < 12 {
                    hour + 12
                } else {
                    hour
                }
            }
            Some(Meridiem::AM) => {
                if hour == 12 {
                    0
                } else {
                    hour
                }
            }
            None => hour,
        }
    }
}

impl Parser for FRTimeExpressionParser {
    fn name(&self) -> &'static str {
        "FRTimeExpressionParser"
    }

    fn should_apply(&self, context: &ParsingContext) -> bool {
        let text = context.text;
        // Must contain digits and time indicators
        text.bytes().any(|b| b.is_ascii_digit())
            && (text.contains('h')
                || text.contains(':')
                || text.contains('.')
                || text.to_lowercase().contains("am")
                || text.to_lowercase().contains("pm"))
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();
        let ref_date = context.reference.instant;

        let mut start = 0;
        while start < context.text.len() {
            let search_text = &context.text[start..];
            let mat = match PRIMARY_PATTERN.find(search_text) {
                Ok(Some(m)) => m,
                Ok(None) => break,
                Err(_) => break,
            };

            let matched_text = mat.as_str();
            let index = start + mat.start();

            // Ensure we're not matching a partial number (e.g., "3.12" from "13.12")
            if index > 0 {
                let prev_char = context.text.as_bytes().get(index - 1);
                if let Some(&c) = prev_char
                    && c.is_ascii_digit()
                {
                    start += mat.end();
                    continue;
                }
            }

            let caps = match PRIMARY_PATTERN.captures(matched_text) {
                Ok(Some(c)) => c,
                Ok(None) => {
                    start = index + 1;
                    continue;
                }
                Err(_) => {
                    start = index + 1;
                    continue;
                }
            };

            // Parse start time
            let hour1: i32 = caps
                .get(1)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(-1);

            if !(0..=23).contains(&hour1) {
                start += mat.end();
                continue;
            }

            // Minutes can be in group 2 (for 8h10) or group 3 (for 8:10)
            let minute1: i32 = caps
                .get(2)
                .or(caps.get(3))
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);

            // Validate minutes
            if !(0..=59).contains(&minute1) {
                start = index + 1;
                continue;
            }

            let second1: i32 = caps
                .get(4)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);

            // Validate seconds
            if !(0..=59).contains(&second1) {
                start += mat.end();
                continue;
            }

            let meridiem1 = caps
                .get(5)
                .map(|m| m.as_str())
                .and_then(Self::parse_meridiem);

            // If AM/PM is specified, hour must be in 12-hour format (1-12)
            // Also check if hour > 12 with decimal (like 13.12)
            if meridiem1.is_some() && hour1 > 12 {
                start += mat.end();
                continue;
            }
            // Hour 0 with AM/PM is invalid
            if meridiem1.is_some() && hour1 == 0 {
                start += mat.end();
                continue;
            }

            // Check for end time (range)
            let has_end_time = caps.get(6).is_some();
            let hour2: i32 = caps
                .get(6)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(-1);
            let minute2: i32 = caps
                .get(7)
                .or(caps.get(8))
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);
            let second2: i32 = caps
                .get(9)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);
            let meridiem2 = caps
                .get(10)
                .map(|m| m.as_str())
                .and_then(Self::parse_meridiem);

            // Adjust hours based on meridiem
            let adj_hour1 = Self::adjust_hour(hour1, meridiem1);

            // Infer meridiem for second time based on first
            let effective_meridiem2 = meridiem2.or(meridiem1);
            let adj_hour2 = if has_end_time && hour2 >= 0 {
                Self::adjust_hour(hour2, effective_meridiem2)
            } else {
                0
            };

            // Build start components
            let mut components = context.create_components();
            components.assign(Component::Hour, adj_hour1);
            components.assign(Component::Minute, minute1);
            if caps.get(4).is_some() {
                components.assign(Component::Second, second1);
            }
            if let Some(m) = meridiem1 {
                components.assign(Component::Meridiem, m as i32);
            } else if adj_hour1 >= 12 {
                components.assign(Component::Meridiem, Meridiem::PM as i32);
            }

            // Build end components if range
            let end_components = if has_end_time && hour2 >= 0 {
                let mut end_comp = context.create_components();
                end_comp.assign(Component::Hour, adj_hour2);
                end_comp.assign(Component::Minute, minute2);
                if caps.get(9).is_some() {
                    end_comp.assign(Component::Second, second2);
                }
                if let Some(m) = effective_meridiem2 {
                    end_comp.assign(Component::Meridiem, m as i32);
                } else if adj_hour2 >= 12 {
                    end_comp.assign(Component::Meridiem, Meridiem::PM as i32);
                }

                // Copy date from reference
                use chrono::Datelike;
                end_comp.imply(Component::Year, ref_date.year());
                end_comp.imply(Component::Month, ref_date.month() as i32);
                end_comp.imply(Component::Day, ref_date.day() as i32);

                Some(end_comp)
            } else {
                None
            };

            // Calculate actual matched text boundaries
            let actual_start = matched_text
                .find(|c: char| c.is_ascii_digit() || c == 'd' || c == 'D' || c == 'à' || c == 'a')
                .unwrap_or(0);
            let actual_text = &matched_text[actual_start..];
            let actual_end = actual_text
                .rfind(|c: char| c.is_ascii_alphanumeric())
                .map(|i| i + actual_text[i..].chars().next().map_or(1, char::len_utf8))
                .unwrap_or(actual_text.len());

            results.push(context.create_result(
                index + actual_start,
                index + actual_start + actual_end,
                components,
                end_components,
            ));

            start += mat.end();
        }

        Ok(results)
    }
}

impl Default for FRTimeExpressionParser {
    fn default() -> Self {
        Self::new()
    }
}
