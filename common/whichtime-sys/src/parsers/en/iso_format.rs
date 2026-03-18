//! ISO 8601 format parser

use crate::components::Component;
use crate::context::ParsingContext;
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use regex::Regex;
use std::sync::LazyLock;

// ISO 8601 pattern: YYYY-MM-DD[THH:MM[:SS[.sss]]][Z|±HH:MM]
static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)(\d{4})-(\d{1,2})-(\d{1,2})(?:T(\d{1,2}):(\d{2})(?::(\d{2}))?(?:\.(\d{1,3}))?)?(?:(Z)|([+-])(\d{2}):?(\d{2}))?"
    ).unwrap()
});

/// Parser for ISO 8601-style date strings.
pub struct ISOFormatParser;

impl Parser for ISOFormatParser {
    fn name(&self) -> &'static str {
        "ISOFormatParser"
    }

    fn should_apply(&self, context: &ParsingContext) -> bool {
        // Quick check: must contain digits and dashes in right pattern
        let text = context.lower_text();
        text.contains('-') && text.bytes().any(|b| b.is_ascii_digit())
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();

        for mat in PATTERN.find_iter(context.text) {
            let matched_text = mat.as_str();
            let index = mat.start();

            // Re-capture to get groups
            let Some(caps) = PATTERN.captures(matched_text) else {
                continue;
            };

            let mut components = context.create_components();

            // Year (group 1)
            if let Some(year_match) = caps.get(1)
                && let Ok(year) = year_match.as_str().parse::<i32>()
            {
                components.assign(Component::Year, year);
            }

            // Month (group 2)
            if let Some(month_match) = caps.get(2)
                && let Ok(month) = month_match.as_str().parse::<i32>()
            {
                if !(1..=12).contains(&month) {
                    continue;
                }
                components.assign(Component::Month, month);
            }

            // Day (group 3)
            if let Some(day_match) = caps.get(3)
                && let Ok(day) = day_match.as_str().parse::<i32>()
            {
                if !(1..=31).contains(&day) {
                    continue;
                }
                components.assign(Component::Day, day);
            }

            // Hour (group 4)
            if let Some(hour_match) = caps.get(4)
                && let Ok(hour) = hour_match.as_str().parse::<i32>()
            {
                if !(0..=23).contains(&hour) {
                    continue;
                }
                components.assign(Component::Hour, hour);
            }

            // Minute (group 5)
            if let Some(min_match) = caps.get(5)
                && let Ok(min) = min_match.as_str().parse::<i32>()
            {
                components.assign(Component::Minute, min);
            }

            // Second (group 6)
            if let Some(sec_match) = caps.get(6)
                && let Ok(sec) = sec_match.as_str().parse::<i32>()
            {
                components.assign(Component::Second, sec);
            }

            // Millisecond (group 7)
            if let Some(ms_match) = caps.get(7)
                && let Ok(ms) = ms_match.as_str().parse::<i32>()
            {
                components.assign(Component::Millisecond, ms);
            }

            // Timezone Z (group 8)
            if caps.get(8).is_some() {
                components.assign(Component::TimezoneOffset, 0);
            }
            // Timezone offset (groups 9, 10, 11)
            else if let (Some(sign), Some(tz_hour), Some(tz_min)) =
                (caps.get(9), caps.get(10), caps.get(11))
            {
                let sign = if sign.as_str() == "-" { -1 } else { 1 };
                let hours: i32 = tz_hour.as_str().parse().unwrap_or(0);
                let mins: i32 = tz_min.as_str().parse().unwrap_or(0);
                let offset = sign * (hours * 60 + mins);
                components.assign(Component::TimezoneOffset, offset);
            }

            // Validate date
            if !components.is_valid_date() {
                continue;
            }

            results.push(context.create_result(
                index,
                index + matched_text.len(),
                components,
                None,
            ));
        }

        Ok(results)
    }
}
