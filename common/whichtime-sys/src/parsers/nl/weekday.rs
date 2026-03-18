//! Dutch weekday parser
//!
//! Handles Dutch weekday expressions like:
//! - "maandag", "op dinsdag"
//! - "volgende vrijdag", "afgelopen vrijdag"
//! - "vrijdag volgende week"

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::RelativeModifier;
use crate::dictionaries::nl::{get_relative_modifier, get_weekday};
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use chrono::{Datelike, Duration};
use fancy_regex::Regex;
use std::sync::LazyLock;

static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?ix)
        (?<![a-zA-Z])
        (?:(?:op|om|tijdens)\s+)?
        (?:
            (?P<prefix>deze|dit|volgende|volgend|komende|komend|afgelopen|vorige|vorig|laatste)
            \s+
        )?
        (?P<weekday>zondag|zon|zo|maandag|maan|ma|dinsdag|dins|di|woensdag|woens|wo|donderdag|donder|do|vrijdag|vrij|vr|zaterdag|zater|za)
        (?:
            \s+
            (?P<suffix_mod>volgende|volgend|komende|komend|afgelopen|vorige|vorig)
            \s+week
        )?
        (?=\W|$)
        "
    ).unwrap()
});

/// Dutch weekday parser
pub struct NLWeekdayParser;

impl NLWeekdayParser {
    pub fn new() -> Self {
        Self
    }
}

impl Default for NLWeekdayParser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser for NLWeekdayParser {
    fn name(&self) -> &'static str {
        "NLWeekdayParser"
    }

    fn should_apply(&self, _context: &ParsingContext) -> bool {
        true
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();
        let ref_date = context.reference.instant;
        let ref_weekday = ref_date.weekday().num_days_from_sunday() as i64;

        let mut start = 0;
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

            let match_start = start + full_match.start();
            let match_end = start + full_match.end();
            let matched_text = full_match.as_str();

            let weekday_str = captures
                .name("weekday")
                .map(|m| m.as_str().to_lowercase())
                .unwrap_or_default();
            let prefix_str = captures.name("prefix").map(|m| m.as_str().to_lowercase());
            let suffix_mod_str = captures
                .name("suffix_mod")
                .map(|m| m.as_str().to_lowercase());

            let Some(weekday) = get_weekday(&weekday_str) else {
                start = match_end;
                continue;
            };

            let target_weekday = weekday as i64;

            // Determine modifier from prefix or suffix
            let modifier = prefix_str
                .as_ref()
                .and_then(|s| get_relative_modifier(s))
                .or_else(|| {
                    suffix_mod_str
                        .as_ref()
                        .and_then(|s| get_relative_modifier(s))
                });

            // Calculate days offset
            let days_offset = match modifier {
                Some(RelativeModifier::Next) => {
                    let diff = target_weekday - ref_weekday;
                    if diff <= 0 { diff + 7 } else { diff }
                }
                Some(RelativeModifier::Last) => {
                    let diff = target_weekday - ref_weekday;
                    if diff >= 0 { diff - 7 } else { diff }
                }
                Some(RelativeModifier::This) | None => {
                    // Default behavior: find closest day
                    // "This Friday" usually means coming Friday, or past Friday if recent?
                    // whichtime-sys conventions usually prefer forward or closest.
                    // Let's follow `de` parser logic: closest within -3 to +3 range or similar.

                    let diff = target_weekday - ref_weekday;
                    if diff == 0 {
                        0
                    } else if diff > 0 {
                        if diff <= 3 { diff } else { diff - 7 }
                    } else if diff >= -3 {
                        diff
                    } else {
                        diff + 7
                    }
                }
            };

            // Special handling for "vrijdag volgende week" (Next week's Friday)
            // If suffix "volgende week" is present, it implies +1 week relative to *this* week's Friday?
            // "vrijdag volgende week" -> Friday of the next week.
            // If today is Monday, "next Friday" usually means Friday of THIS week (if close) or NEXT week.
            // BUT "vrijdag volgende week" explicitly shifts the week.
            // So we find "Friday" of the current week, then add 7 days?
            // Or find "Next Friday" then add 7 days?

            // Interpretation: "vrijdag volgende week"
            // 1. Find Friday of this week.
            // 2. Add 7 days.

            let final_offset = if suffix_mod_str.is_some() {
                if modifier == Some(RelativeModifier::Next) {
                    // "vrijdag volgende week"
                    // Find "this week's Friday" (forward or backward)?
                    // Usually relative to start of week or just +7 days from "this Friday".
                    // Let's say: target is Friday of (Current Week + 1).
                    // Current week defined by reference date.
                    // Simple approach: Calculate days to next Friday, then ensure it's in next week.

                    // Alternative: (target_weekday - ref_weekday + 7) % 7 + (days until next week)
                    // Actually, let's use the logic: "Friday of next week".
                    // Calculate Friday of current week, add 7.
                    // Current week's Friday relative to today:
                    // ref_weekday is today's index (0=Sun).
                    // target_weekday is Friday (5).
                    // Friday of THIS week: target_weekday - ref_weekday.
                    // Example: Today=Wed(3), Fri(5). Diff=2. Date+2 is Friday. Next week Fri is Date+9.
                    // Example: Today=Sat(6), Fri(5). Diff=-1. Date-1 is Friday. Next week Fri is Date+6.
                    // Wait, "next week" usually starts on Monday or Sunday.
                    // "vrijdag volgende week" means the Friday that falls in the week after the current week.
                    // If today is Monday, "next week" starts next Monday. Friday next week is > 7 days away.

                    let diff = target_weekday - ref_weekday;

                    // Let's stick to `de` logic:
                    if diff > 0 {
                        diff + 7 // If Friday is later this week, add 7 to get to next week
                    } else {
                        // If Friday was earlier this week, add 7 to get to next week? No.
                        // If Today is Fri, "Friday next week" is +7.
                        // If Today is Sat, "Friday next week" is +6? No, that's "next Friday".
                        // "Friday next week" means Friday of the NEXT week.
                        // If Today is Sat(6), Friday(5) of THIS week was yesterday (-1).
                        // Friday of NEXT week is -1 + 7 = +6. (Which is the coming Friday).
                        // Is "Next Friday" same as "Friday next week"?
                        // Often "Next Friday" means the immediate next one.
                        // "Friday next week" is explicit.

                        // Let's assume: (target - ref) + 7, but adjusted so we land in next week.
                        // Simpler: Find the Friday that is in the week starting after this week.

                        diff + 7
                    }
                } else if modifier == Some(RelativeModifier::Last) {
                    // "vrijdag vorige week"
                    let diff = target_weekday - ref_weekday;
                    diff - 7
                } else {
                    days_offset
                }
            } else {
                days_offset
            };

            let target_date = ref_date + Duration::days(final_offset);

            let mut components = context.create_components();
            components.assign(Component::Year, target_date.year());
            components.assign(Component::Month, target_date.month() as i32);
            components.assign(Component::Day, target_date.day() as i32);
            components.assign(Component::Weekday, target_weekday as i32);

            // Find actual text bounds (trim leading/trailing non-alphanumeric)
            let actual_start = matched_text
                .find(|c: char| c.is_alphanumeric())
                .unwrap_or(0);
            let actual_end = matched_text
                .rfind(|c: char| c.is_alphanumeric())
                .map(|i| i + matched_text[i..].chars().next().map_or(1, char::len_utf8))
                .unwrap_or(matched_text.len());

            results.push(context.create_result(
                match_start + actual_start,
                match_start + actual_end,
                components,
                None,
            ));

            start = match_end;
        }

        Ok(results)
    }
}
