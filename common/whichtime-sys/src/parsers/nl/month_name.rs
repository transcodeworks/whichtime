//! Dutch month name little endian parser
//!
//! Handles Dutch date expressions with month names like:
//! - "10 januari 2012"
//! - "10 augustus 88 na Christus" / "10 augustus 234 voor Christus"
//! - "10 - 22 augustus 2012" (date ranges)
//! - "10 tot 22 augustus 2012"
//! - "Zon 15 Sept", "di, 10 januari"

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::nl::{get_month, get_weekday};
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use chrono::Datelike;
use fancy_regex::Regex;
use std::sync::LazyLock;

// Main pattern for Dutch month name dates
// Supports: "10 augustus 2012", "10 tot 22 augustus", "di, 10 januari", "10 augustus 88 na Christus"
static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?ix)
        (?:
            (?P<weekday>zondag|zon|zo|maandag|maan|ma|dinsdag|dins|di|woensdag|woens|wo|donderdag|donder|do|vrijdag|vrij|vr|zaterdag|zater|za)
            (?:\s*,?\s*)?
        )?
        (?P<day>\d{1,2})(?:e|ste|de)?\s*
        (?:
            (?:(?:tot|\-|–)\s*(?P<end_day>\d{1,2})(?:e|ste|de)?\s*)?
        )?
        (?P<month>januari|jan\.?|februari|feb\.?|maart|mrt\.?|april|apr\.?|mei|juni|jun\.?|juli|jul\.?|augustus|aug\.?|september|sep\.?|sept\.?|oktober|okt\.?|november|nov\.?|december|dec\.?)
        (?:
            (?:\s*[\-/,]?\s*)?
            (?P<year>\d{1,4}(?!:))?
            (?:\s*(?P<era>
                (?:voor|v\.?)\s*(?:Christus|Chr\.?)
                |(?:na|n\.?)\s*(?:Christus|Chr\.?)
            ))?
        )?
        (?=\W|$)
        "
    ).unwrap()
});

// Pattern for abbreviated weekday + date format: "Zon 15 Sept"
static ABBREV_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?ix)(?P<weekday>zondag|zon|zo|maandag|maan|ma|dinsdag|dins|di|woensdag|woens|wo|donderdag|donder|do|vrijdag|vrij|vr|zaterdag|zater|za)\.?\s+(?P<day>\d{1,2})(?:e|ste|de)?\s+(?P<month>januari|jan\.?|februari|feb\.?|maart|mrt\.?|april|apr\.?|mei|juni|jun\.?|juli|jul\.?|augustus|aug\.?|september|sep\.?|sept\.?|oktober|okt\.?|november|nov\.?|december|dec\.?)(?:\s*(?P<year>\d{2,4}))?(?=\W|$)"
    ).unwrap()
});

/// Dutch month name parser
pub struct NLMonthNameParser;

impl NLMonthNameParser {
    pub fn new() -> Self {
        Self
    }

    fn parse_year_with_era(year_str: Option<&str>, era_str: Option<&str>) -> Option<i32> {
        let year_text = year_str?;
        let mut year: i32 = year_text.parse().ok()?;

        // Handle two-digit years
        if year < 100 && era_str.is_none() {
            year = if year > 50 { 1900 + year } else { 2000 + year };
        }

        // Handle era suffixes
        if let Some(era) = era_str {
            let era_lower = era.to_lowercase();

            // BC: voor Christus, v. Chr.
            if era_lower.contains("voor") || era_lower.starts_with('v') {
                // 1 BC is year 0 in chrono? No, chrono uses astronomical year numbering: 1 BC is 0, 2 BC is -1.
                // Wait, chrono::Datelike::year() returns i32. 1 CE = 1. 1 BCE = 0.
                // If input says "234 voor Christus" (234 BC), we want year -233 (astronomical) or just let chrono handle it?
                // Whichtime usually returns the year number.
                // If I return -234, it means 235 BC.
                // Let's stick to negative for BC. 1 BC -> 0. 234 BC -> -233.
                // Formula: 1 - year_bc
                // year 234 BC -> 1 - 234 = -233.

                // BUT, let's check how other parsers handle it.
                // DE parser: starts_with('v') -> returns -year.
                // If year is 234, returns -234.
                // Let's assume simplistic -year for now.
                return Some(-year);
            }
        }

        Some(year)
    }
}

impl Default for NLMonthNameParser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser for NLMonthNameParser {
    fn name(&self) -> &'static str {
        "NLMonthNameParser"
    }

    fn should_apply(&self, _context: &ParsingContext) -> bool {
        true
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();
        let ref_date = context.reference.instant;

        // Try abbreviated pattern (Zon 15 Sept)
        let mut start = 0;
        while start < context.text.len() {
            let search_text = &context.text[start..];
            let captures = match ABBREV_PATTERN.captures(search_text) {
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

            let weekday_str = captures.name("weekday").map(|m| m.as_str().to_lowercase());
            let day_str = captures.name("day").map(|m| m.as_str()).unwrap_or("1");
            let month_str = captures
                .name("month")
                .map(|m| m.as_str().to_lowercase())
                .unwrap_or_default();
            let year_str = captures.name("year").map(|m| m.as_str());

            // Clean up month string (remove trailing dot)
            let month_clean = month_str.trim_end_matches('.');

            let Some(month) = get_month(month_clean) else {
                start = match_end;
                continue;
            };

            let day: i32 = day_str.parse().unwrap_or(1);
            if !(1..=31).contains(&day) {
                start = match_end;
                continue;
            }

            let mut components = context.create_components();

            if let Some(y) = year_str {
                let year = Self::parse_year_with_era(Some(y), None).unwrap_or(ref_date.year());
                components.assign(Component::Year, year);
            } else {
                components.imply(Component::Year, ref_date.year());
            }

            components.assign(Component::Month, month as i32);
            components.assign(Component::Day, day);

            if let Some(ref wd_str) = weekday_str
                && let Some(weekday) = get_weekday(wd_str)
            {
                components.assign(Component::Weekday, weekday as i32);
            }

            if !components.is_valid_date() {
                start = match_end;
                continue;
            }

            results.push(context.create_result(match_start, match_end, components, None));
            start = match_end;
        }

        // Try main pattern
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

            let match_start = start + full_match.start();
            let match_end = start + full_match.end();

            let weekday_str = captures.name("weekday").map(|m| m.as_str().to_lowercase());
            let day_str = captures.name("day").map(|m| m.as_str()).unwrap_or("1");
            let month_str = captures
                .name("month")
                .map(|m| m.as_str().to_lowercase())
                .unwrap_or_default();
            let year_str = captures.name("year").map(|m| m.as_str());
            let era_str = captures.name("era").map(|m| m.as_str());
            let end_day_str = captures.name("end_day").map(|m| m.as_str());

            let month_clean = month_str.trim_end_matches('.');

            let Some(month) = get_month(month_clean) else {
                start = match_end;
                continue;
            };

            let day: i32 = day_str.parse().unwrap_or(1);
            if !(1..=31).contains(&day) {
                start = match_end;
                continue;
            }

            let mut components = context.create_components();

            if year_str.is_some() || era_str.is_some() {
                let year = Self::parse_year_with_era(year_str, era_str).unwrap_or(ref_date.year());
                components.assign(Component::Year, year);
            } else {
                components.imply(Component::Year, ref_date.year());
            }

            components.assign(Component::Month, month as i32);
            components.assign(Component::Day, day);

            if let Some(ref wd_str) = weekday_str
                && let Some(weekday) = get_weekday(wd_str)
            {
                components.assign(Component::Weekday, weekday as i32);
            }

            if !components.is_valid_date() {
                start = match_end;
                continue;
            }

            // Handle end date for ranges (10 tot 22 augustus)
            let end_components = if let Some(end_day_text) = end_day_str {
                let end_day: i32 = end_day_text.parse().unwrap_or(0);
                if end_day > 0 && end_day <= 31 {
                    let mut end_comp = context.create_components();
                    if let Some(start_year) = components.get(Component::Year) {
                        if year_str.is_some() || era_str.is_some() {
                            end_comp.assign(Component::Year, start_year);
                        } else {
                            end_comp.imply(Component::Year, start_year);
                        }
                    }
                    end_comp.assign(Component::Month, month as i32);
                    end_comp.assign(Component::Day, end_day);

                    if end_comp.is_valid_date() {
                        Some(end_comp)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            };

            results.push(context.create_result(match_start, match_end, components, end_components));
            start = match_end;
        }

        Ok(results)
    }
}
