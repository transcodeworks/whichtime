//! German weekday parser
//!
//! Handles German weekday expressions like:
//! - "Montag", "am Donnerstag"
//! - "letzten Freitag", "nächsten Montag"
//! - "am Freitag nächste Woche", "am Dienstag nächste Woche"
//! - "Sonntag, den 7. Dezember 2014" (weekday with date - handled by overlap removal)

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::RelativeModifier;
use crate::dictionaries::de::{get_relative_modifier, get_weekday};
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use chrono::{Datelike, Duration};
use fancy_regex::Regex;
use std::sync::LazyLock;

// Pattern for German weekday expressions
// Supports: "Montag", "am Donnerstag", "letzten Freitag", "am Freitag nächste Woche"
static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?ix)
        (?<![a-zA-ZäöüÄÖÜß])
        (?:am\s+)?
        (?:
            (?P<prefix>diese[nmrs]?|nächste[nmrs]?|naechste[nmrs]?|kommende[nmrs]?|letzte[nmrs]?|vergangene[nmrs]?|vorige[nmrs]?)
            \s+
        )?
        (?P<weekday>sonntag|so|montag|mo|dienstag|di|mittwoch|mi|donnerstag|do|freitag|fr|samstag|sa)
        (?:
            \s+
            (?P<suffix>diese|nächste|naechste|kommende|letzte|vergangene|vorige)
            \s+woche
        )?
        (?=\W|$)
        "
    ).unwrap()
});

/// German weekday parser
pub struct DEWeekdayParser;

impl DEWeekdayParser {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DEWeekdayParser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser for DEWeekdayParser {
    fn name(&self) -> &'static str {
        "DEWeekdayParser"
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
            let suffix_str = captures.name("suffix").map(|m| m.as_str().to_lowercase());

            let Some(weekday) = get_weekday(&weekday_str) else {
                start = match_end;
                continue;
            };

            let target_weekday = weekday as i64;

            // Determine modifier from prefix or suffix
            let modifier = prefix_str
                .as_ref()
                .and_then(|s| get_relative_modifier(s))
                .or_else(|| suffix_str.as_ref().and_then(|s| get_relative_modifier(s)));

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
                    // Find closest occurrence (past or future)
                    let diff = target_weekday - ref_weekday;
                    if diff == 0 {
                        0 // Same day
                    } else if diff > 0 {
                        // Target is ahead in the week
                        if diff <= 3 {
                            diff // Go forward
                        } else {
                            diff - 7 // Go back to previous week
                        }
                    } else {
                        // diff < 0, target is behind in the week
                        if diff >= -3 {
                            diff // Go back
                        } else {
                            diff + 7 // Go to next week
                        }
                    }
                }
            };

            // If suffix mentions "nächste Woche", add an extra week
            let final_offset = if suffix_str.is_some() && modifier == Some(RelativeModifier::Next) {
                // "am Freitag nächste Woche" - ensure we go to next week
                let diff = target_weekday - ref_weekday;
                if diff > 0 {
                    diff // Already in the future this week, go to next week
                } else {
                    diff + 7 // Go to next week
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
