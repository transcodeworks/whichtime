//! German time expression parser
//!
//! Handles German time expressions like:
//! - "14 Uhr", "14:30", "14h30", "um 16h"
//! - "um 7 morgens", "8 Uhr abends"
//! - "11:00 Uhr vormittags", "um 8 Uhr nachmittags"
//! - "um 5 Uhr in der Nacht"
//! - Time ranges: "18:10 - 22.32", "von 6:30 bis 23:00"

use crate::components::Component;
use crate::context::ParsingContext;
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use crate::types::Meridiem;
use fancy_regex::Regex;
use std::sync::LazyLock;

// Pattern for German time expressions
// Matches: "14 Uhr", "14:30", "14h30", "um 14 Uhr", "um 16h", "8 Uhr abends", etc.
static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?ix)
        (?:(?:von|um)\s+)?
        (\d{1,2})
        (?:[:h\.](\d{2}))?
        (?:\s*uhr)?
        (?:\s*(morgens?|vormittags?|nachmittags?|abends?|nachts?|in\s+der\s+nacht))?
        ",
    )
    .unwrap()
});

// Pattern for time ranges: "18:10 - 22.32", "von 6:30 bis 23:00", "von 6h30 bis 23h00"
static RANGE_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?ix)
        (?:von\s+)?
        (\d{1,2})[:h\.](\d{2})
        (?:\s*uhr)?
        \s*
        (?:-|–|bis)\s*
        (\d{1,2})[:h\.](\d{2})
        (?:\s*uhr)?
        ",
    )
    .unwrap()
});

// Pattern for "um Xh" format (e.g., "um 16h")
static H_FORMAT_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)(?:um\s+)?(\d{1,2})h(?:(\d{2}))?(?=\W|$)").unwrap());

/// German time expression parser
pub struct DETimeExpressionParser;

impl DETimeExpressionParser {
    pub fn new() -> Self {
        Self
    }
}

impl Parser for DETimeExpressionParser {
    fn name(&self) -> &'static str {
        "DETimeExpressionParser"
    }

    fn should_apply(&self, context: &ParsingContext) -> bool {
        context.text.bytes().any(|b| b.is_ascii_digit())
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();

        // First, try to match time ranges
        let mut start = 0;
        while start < context.text.len() {
            let search_text = &context.text[start..];
            let captures = match RANGE_PATTERN.captures(search_text) {
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

            let start_hour: i32 = captures
                .get(1)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(-1);
            let start_minute: i32 = captures
                .get(2)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);
            let end_hour: i32 = captures
                .get(3)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(-1);
            let end_minute: i32 = captures
                .get(4)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);

            if (0..=23).contains(&start_hour)
                && start_minute <= 59
                && (0..=23).contains(&end_hour)
                && end_minute <= 59
            {
                let mut start_components = context.create_components();
                start_components.assign(Component::Hour, start_hour);
                start_components.assign(Component::Minute, start_minute);
                if start_hour >= 12 {
                    start_components.assign(Component::Meridiem, Meridiem::PM as i32);
                } else {
                    start_components.assign(Component::Meridiem, Meridiem::AM as i32);
                }

                let mut end_components = context.create_components();
                end_components.assign(Component::Hour, end_hour);
                end_components.assign(Component::Minute, end_minute);
                if end_hour >= 12 {
                    end_components.assign(Component::Meridiem, Meridiem::PM as i32);
                } else {
                    end_components.assign(Component::Meridiem, Meridiem::AM as i32);
                }

                results.push(context.create_result(
                    match_start,
                    match_end,
                    start_components,
                    Some(end_components),
                ));
            }

            start = match_end;
        }

        // Try "um Xh" format
        start = 0;
        while start < context.text.len() {
            let search_text = &context.text[start..];
            let captures = match H_FORMAT_PATTERN.captures(search_text) {
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

            // Skip if this range was already captured by the range pattern
            if results
                .iter()
                .any(|r| r.index <= match_start && r.end_index >= match_end)
            {
                start = match_end;
                continue;
            }

            let hour: i32 = captures
                .get(1)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(-1);
            let minute: i32 = captures
                .get(2)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);

            if (0..=23).contains(&hour) && minute <= 59 {
                let mut components = context.create_components();
                components.assign(Component::Hour, hour);
                components.assign(Component::Minute, minute);
                if hour >= 12 {
                    components.assign(Component::Meridiem, Meridiem::PM as i32);
                } else {
                    components.assign(Component::Meridiem, Meridiem::AM as i32);
                }

                results.push(context.create_result(match_start, match_end, components, None));
            }

            start = match_end;
        }

        // Standard time patterns
        start = 0;
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

            let matched_text = full_match.as_str();
            let match_start = start + full_match.start();
            let match_end = start + full_match.end();

            // Skip if this range was already captured
            if results.iter().any(|r| {
                (r.index <= match_start && r.end_index > match_start)
                    || (match_start <= r.index && match_end > r.index)
            }) {
                start = match_end.max(start + 1);
                continue;
            }

            // Skip if match doesn't contain "uhr", "h" with minutes, ":", or a time modifier
            // This prevents matching bare numbers
            let matched_lower = matched_text.to_lowercase();
            let has_time_indicator = matched_lower.contains("uhr")
                || matched_lower.contains(':')
                || (matched_text.to_lowercase().contains('h') && captures.get(2).is_some())
                || captures.get(3).is_some();

            if !has_time_indicator {
                start = match_end.max(start + 1);
                continue;
            }

            let mut hour: i32 = captures
                .get(1)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(-1);

            let minute: i32 = captures
                .get(2)
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);

            let modifier = captures.get(3).map(|m| m.as_str().to_lowercase());

            // Validate hour and minute
            if !(0..=23).contains(&hour) || minute > 59 {
                start = match_end;
                continue;
            }

            // Determine meridiem and adjust hour based on modifier
            let meridiem = if let Some(ref mod_str) = modifier {
                if mod_str.starts_with("morgen") || mod_str.starts_with("vormittag") {
                    // Morning: keep hour as is (assumed AM)
                    if hour == 12 {
                        hour = 0;
                    }
                    Some(Meridiem::AM)
                } else if mod_str.starts_with("nachmittag") || mod_str.starts_with("abend") {
                    // Afternoon/Evening: add 12 if hour < 12
                    if hour < 12 {
                        hour += 12;
                    }
                    Some(Meridiem::PM)
                } else if mod_str.starts_with("nacht") || mod_str.contains("in der nacht") {
                    // Night: depends on hour
                    // 8 Uhr in der Nacht = 20:00 (PM)
                    // 5 Uhr in der Nacht = 05:00 (AM)
                    if hour <= 6 {
                        Some(Meridiem::AM)
                    } else if hour < 12 {
                        hour += 12;
                        Some(Meridiem::PM)
                    } else {
                        Some(Meridiem::PM)
                    }
                } else {
                    None
                }
            } else if hour >= 12 {
                Some(Meridiem::PM)
            } else {
                Some(Meridiem::AM)
            };

            let mut components = context.create_components();
            components.assign(Component::Hour, hour);
            components.assign(Component::Minute, minute);

            if let Some(m) = meridiem {
                components.assign(Component::Meridiem, m as i32);
            }

            results.push(context.create_result(match_start, match_end, components, None));

            start = match_end;
        }

        Ok(results)
    }
}

impl Default for DETimeExpressionParser {
    fn default() -> Self {
        Self::new()
    }
}
