//! Spanish time expression parser
//!
//! Handles Spanish time expressions like:
//! - "6.13 AM" (dot separator)
//! - "las 6:30pm"
//! - "de 6:30pm a 11:00pm" (ranges)
//! - "8:10 - 12.32" (ranges)

use crate::components::Component;
use crate::context::ParsingContext;
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use crate::types::Meridiem;
use fancy_regex::Regex;
use std::sync::LazyLock;

// Primary pattern: handles single times and "de X a Y" ranges
// Supports both : and . as separators
static PRIMARY_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)(?:^|[^\d])(?:de\s+)?(?:las?\s+)?(\d{1,2})(?:[:\.](\d{2}))?(?::(\d{2}))?(?:\s*(a\.?m\.?|p\.?m\.?))?(?:\s*(?:a(?:\s+las?)?|[\-–~])\s*(\d{1,2})(?:[:\.](\d{2}))?(?::(\d{2}))?(?:\s*(a\.?m\.?|p\.?m\.?))?)?(?=[^\d]|$)"
    ).unwrap()
});

/// Spanish time expression parser
pub struct ESTimeExpressionParser;

impl ESTimeExpressionParser {
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

impl Parser for ESTimeExpressionParser {
    fn name(&self) -> &'static str {
        "ESTimeExpressionParser"
    }

    fn should_apply(&self, context: &ParsingContext) -> bool {
        let text = context.text;
        // Must contain digits and time separators or meridiem indicators
        text.bytes().any(|b| b.is_ascii_digit())
            && (text.contains(':')
                || text.contains('.')
                || text.to_lowercase().contains("am")
                || text.to_lowercase().contains("pm")
                || text.to_lowercase().contains("a.m")
                || text.to_lowercase().contains("p.m"))
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
                start = index + 1;
                continue;
            }

            let minute1: i32 = caps
                .get(2)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);
            let second1: i32 = caps
                .get(3)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);
            let meridiem1 = caps
                .get(4)
                .map(|m| m.as_str())
                .and_then(Self::parse_meridiem);

            // Check for end time (range)
            let has_end_time = caps.get(5).is_some();
            let hour2: i32 = caps
                .get(5)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(-1);
            let minute2: i32 = caps
                .get(6)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);
            let second2: i32 = caps
                .get(7)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);
            let meridiem2 = caps
                .get(8)
                .map(|m| m.as_str())
                .and_then(Self::parse_meridiem);

            // If no explicit meridiem and no minutes, and not in a range, this might not be a time
            // Be strict about standalone numbers without context
            if meridiem1.is_none() && caps.get(2).is_none() && !has_end_time {
                // Check if there's a time prefix like "las" or "a las"
                let prefix =
                    &matched_text[..matched_text.find(|c: char| c.is_ascii_digit()).unwrap_or(0)];
                let lower_prefix = prefix.to_lowercase();
                if !lower_prefix.contains("las") && !lower_prefix.contains("de") {
                    start = index + 1;
                    continue;
                }
            }

            // Adjust hours based on meridiem
            let adj_hour1 = Self::adjust_hour(hour1, meridiem1);

            // Infer meridiem for second time based on first
            let effective_meridiem2 = meridiem2.or(meridiem1);
            let adj_hour2 = if has_end_time {
                Self::adjust_hour(hour2, effective_meridiem2)
            } else {
                0
            };

            // Build start components
            let mut components = context.create_components();
            components.assign(Component::Hour, adj_hour1);
            components.assign(Component::Minute, minute1);
            if caps.get(3).is_some() {
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
                if caps.get(7).is_some() {
                    end_comp.assign(Component::Second, second2);
                }
                if let Some(m) = effective_meridiem2 {
                    end_comp.assign(Component::Meridiem, m as i32);
                } else if adj_hour2 >= 12 {
                    end_comp.assign(Component::Meridiem, Meridiem::PM as i32);
                }

                // Copy date from reference for end component
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
                .find(|c: char| c.is_ascii_digit() || c == 'd' || c == 'D' || c == 'l' || c == 'L')
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

impl Default for ESTimeExpressionParser {
    fn default() -> Self {
        Self::new()
    }
}
