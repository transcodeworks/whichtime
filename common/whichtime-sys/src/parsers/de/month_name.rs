//! German month name little endian parser
//!
//! Handles German date expressions with month names like:
//! - "10. August 2012"
//! - "10. August 85 n. Chr." / "10. August 113 v. Chr."
//! - "10. - 22. August 2012" (date ranges)
//! - "am 10. August", "am Dienstag, den 10. Januar"
//! - "So 15.Sep", "Di, 10. Januar"
//! - Various year suffixes: v.u.Z., n.u.Z., d.g.Z., v.d.Z., etc.

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::de::{get_month, get_weekday};
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use chrono::Datelike;
use fancy_regex::Regex;
use std::sync::LazyLock;

// Main pattern for German month name dates
// Supports: "10. August 2012", "am 10. August", "Di, 10. Januar", "10. August 85 n. Chr."
static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?ix)
        (?:
            (?:am\s+)?
            (?:(?P<weekday>sonntag|so|montag|mo|dienstag|di|mittwoch|mi|donnerstag|do|freitag|fr|samstag|sa)
                (?:\s*,?\s*(?:den\s+)?)?
            )?
        )?
        (?P<day>\d{1,2})\.?\s*
        (?:
            (?:(?:bis(?:\s*(?:am|zum))?|\-|–)\s*(?P<end_day>\d{1,2})\.?\s*)?
        )?
        (?P<month>januar|jänner|janner|jan\.?|februar|feber|feb\.?|märz|maerz|mär\.?|mrz\.?|april|apr\.?|mai|juni|jun\.?|juli|jul\.?|august|aug\.?|september|sep\.?|sept\.?|oktober|okt\.?|november|nov\.?|dezember|dez\.?)
        (?:
            (?:\s*[\-/,]?\s*)?
            (?P<year>\d{1,4}(?!:))?
            (?:\s*(?P<era>
                v\.?\s*(?:Chr\.?|u\.?\s*Z\.?|d\.?\s*(?:g\.?\s*)?Z\.?)
                |n\.?\s*(?:Chr\.?|C|u\.?\s*Z\.?|d\.?\s*(?:g\.?\s*)?Z\.?)
                |u\.?\s*Z\.?
                |d\.?\s*g\.?\s*Z\.?
            ))?
        )?
        (?=\W|$)
        "
    ).unwrap()
});

// Pattern for cross-month date ranges: "10. Oktober - 12. Dezember", "10. August - 12. Oktober 2013"
static RANGE_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?ix)
        (?P<start_day>\d{1,2})\.?\s*
        (?P<start_month>januar|jänner|janner|jan\.?|februar|feber|feb\.?|märz|maerz|mär\.?|mrz\.?|april|apr\.?|mai|juni|jun\.?|juli|jul\.?|august|aug\.?|september|sep\.?|sept\.?|oktober|okt\.?|november|nov\.?|dezember|dez\.?)
        \s*(?:-|–|bis)\s*
        (?P<end_day>\d{1,2})\.?\s*
        (?P<end_month>januar|jänner|janner|jan\.?|februar|feber|feb\.?|märz|maerz|mär\.?|mrz\.?|april|apr\.?|mai|juni|jun\.?|juli|jul\.?|august|aug\.?|september|sep\.?|sept\.?|oktober|okt\.?|november|nov\.?|dezember|dez\.?)
        (?:\s*(?P<year>\d{1,4}))?
        (?=\W|$)
        "
    ).unwrap()
});

// Pattern for abbreviated weekday + date format: "So 15.Sep", "SO 15.SEPT"
static ABBREV_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)(?P<weekday>so|mo|di|mi|do|fr|sa)\s+(?P<day>\d{1,2})\.(?P<month>jan\.?|feb\.?|mär\.?|mrz\.?|maerz|apr\.?|mai|jun\.?|jul\.?|aug\.?|sep\.?|sept\.?|okt\.?|nov\.?|dez\.?)(?:\s*(?P<year>\d{2,4}))?(?=\W|$)"
    ).unwrap()
});

/// German month name parser
pub struct DEMonthNameParser;

impl DEMonthNameParser {
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
            let era_lower = era.to_lowercase().replace([' ', '.'], "");

            // BC/negative years: v.Chr., v.u.Z., v.d.Z., v.d.g.Z.
            if era_lower.starts_with('v') {
                return Some(-year);
            }
            // AD/positive years: n.Chr., n.u.Z., n.d.Z., n.d.g.Z., nC, uZ, d.g.Z.
            // These are already positive, just return as-is
        }

        Some(year)
    }
}

impl Default for DEMonthNameParser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser for DEMonthNameParser {
    fn name(&self) -> &'static str {
        "DEMonthNameParser"
    }

    fn should_apply(&self, _context: &ParsingContext) -> bool {
        true
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();
        let ref_date = context.reference.instant;

        // Try cross-month range pattern first (10. Oktober - 12. Dezember)
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

            let start_day_str = captures
                .name("start_day")
                .map(|m| m.as_str())
                .unwrap_or("1");
            let start_month_str = captures
                .name("start_month")
                .map(|m| m.as_str().to_lowercase())
                .unwrap_or_default();
            let end_day_str = captures.name("end_day").map(|m| m.as_str()).unwrap_or("1");
            let end_month_str = captures
                .name("end_month")
                .map(|m| m.as_str().to_lowercase())
                .unwrap_or_default();
            let year_str = captures.name("year").map(|m| m.as_str());

            let start_month_clean = start_month_str.trim_end_matches('.');
            let end_month_clean = end_month_str.trim_end_matches('.');

            let Some(start_month) = get_month(start_month_clean) else {
                start = match_end;
                continue;
            };
            let Some(end_month) = get_month(end_month_clean) else {
                start = match_end;
                continue;
            };

            let start_day: i32 = start_day_str.parse().unwrap_or(1);
            let end_day: i32 = end_day_str.parse().unwrap_or(1);

            if !(1..=31).contains(&start_day) || !(1..=31).contains(&end_day) {
                start = match_end;
                continue;
            }

            let year = if let Some(y) = year_str {
                Self::parse_year_with_era(Some(y), None).unwrap_or(ref_date.year())
            } else {
                ref_date.year()
            };

            let mut start_components = context.create_components();
            start_components.assign(Component::Year, year);
            start_components.assign(Component::Month, start_month as i32);
            start_components.assign(Component::Day, start_day);

            let mut end_components = context.create_components();
            end_components.assign(Component::Year, year);
            end_components.assign(Component::Month, end_month as i32);
            end_components.assign(Component::Day, end_day);

            if start_components.is_valid_date() && end_components.is_valid_date() {
                results.push(context.create_result(
                    match_start,
                    match_end,
                    start_components,
                    Some(end_components),
                ));
            }

            start = match_end;
        }

        // Try abbreviated pattern (So 15.Sep)
        start = 0;
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
                // When no year is specified, imply the reference year
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
            let end_day_str = captures
                .name("end_day")
                .or_else(|| captures.name("end_day2"))
                .map(|m| m.as_str());
            let end_month_str = captures
                .name("end_month")
                .map(|m| m.as_str().to_lowercase());

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

            if year_str.is_some() || era_str.is_some() {
                let year = Self::parse_year_with_era(year_str, era_str).unwrap_or(ref_date.year());
                components.assign(Component::Year, year);
            } else {
                // When no year is specified, imply the reference year
                // The ForwardDateRefiner will handle forward-looking behavior if needed
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

            // Handle end date for ranges
            let end_components = if let Some(end_day_text) = end_day_str {
                let end_day: i32 = end_day_text.parse().unwrap_or(0);
                if end_day > 0 && end_day <= 31 {
                    let end_month = if let Some(ref em_str) = end_month_str {
                        get_month(em_str.trim_end_matches('.')).unwrap_or(month)
                    } else {
                        month
                    };

                    let mut end_comp = context.create_components();
                    // Copy year from start components
                    if let Some(start_year) = components.get(Component::Year) {
                        if year_str.is_some() || era_str.is_some() {
                            end_comp.assign(Component::Year, start_year);
                        } else {
                            end_comp.imply(Component::Year, start_year);
                        }
                    }
                    end_comp.assign(Component::Month, end_month as i32);
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
