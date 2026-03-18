//! Portuguese month name little endian parser
//!
//! Handles Portuguese date expressions with month names like:
//! - "10 Agosto 2012", "10 de Agosto de 2012"
//! - "10 a 22 Agosto 2012" (date ranges)
//! - "Dom 15Set", "DOM 15SET"
//! - "Qua, 10 Janeiro"

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::pt::{get_month, get_weekday};
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use chrono::Datelike;
use fancy_regex::Regex;
use std::sync::LazyLock;

static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?ix)
        (?:
            (?P<weekday>domingo|dom\.?|segunda(?:-feira)?|seg\.?|terça(?:-feira)?|terca(?:-feira)?|ter\.?|quarta(?:-feira)?|qua\.?|quinta(?:-feira)?|qui\.?|sexta(?:-feira)?|sex\.?|sábado|sabado|sab\.?)\s*,?\s*
        )?
        (?P<day>\d{1,2})(?:º|°)?\s*
        (?:
            (?:(?:a|até|\-|–)\s*(?P<end_day>\d{1,2})(?:º|°)?\s*)?
        )?
        (?:de\s+)?
        (?P<month>janeiro|jan\.?|fevereiro|fev\.?|março|marco|mar\.?|abril|abr\.?|maio|mai\.?|junho|jun\.?|julho|jul\.?|agosto|ago\.?|setembro|set\.?|outubro|out\.?|novembro|nov\.?|dezembro|dez\.?)
        (?:
            (?:\s*(?:de|,|-)?\s*)
            (?P<year>\d{1,4}(?!:))
            (?:\s*(?P<era>AC|d\.?\s*C\.?|A\.?\s*C\.?))?
        )?
        (?=\W|$)
        "
    ).unwrap()
});

// Pattern for abbreviated weekday + date format: "Dom 15Set"
static ABBREV_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?ix)(?P<weekday>dom|seg|ter|qua|qui|sex|sab)\.?\s*(?P<day>\d{1,2})(?P<month>jan|fev|mar|abr|mai|jun|jul|ago|set|out|nov|dez)\.?(?:\s*(?P<year>\d{2,4}))?(?=\W|$)"
    ).unwrap()
});

// Pattern for cross-month date ranges: "10 Agosto - 12 Setembro"
static RANGE_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?ix)
        (?P<start_day>\d{1,2})(?:º|°)?\s*(?:de\s+)?
        (?P<start_month>janeiro|jan\.?|fevereiro|fev\.?|março|marco|mar\.?|abril|abr\.?|maio|mai\.?|junho|jun\.?|julho|jul\.?|agosto|ago\.?|setembro|set\.?|outubro|out\.?|novembro|nov\.?|dezembro|dez\.?)
        \s*(?:-|a|até)\s*
        (?P<end_day>\d{1,2})(?:º|°)?\s*(?:de\s+)?
        (?P<end_month>janeiro|jan\.?|fevereiro|fev\.?|março|marco|mar\.?|abril|abr\.?|maio|mai\.?|junho|jun\.?|julho|jul\.?|agosto|ago\.?|setembro|set\.?|outubro|out\.?|novembro|nov\.?|dezembro|dez\.?)
        (?:\s*(?:de\s+)?(?P<year>\d{1,4}))?
        (?=\W|$)
        "
    ).unwrap()
});

/// Portuguese month name parser
pub struct PTMonthNameParser;

impl PTMonthNameParser {
    pub fn new() -> Self {
        Self
    }

    fn parse_year_with_era(year_str: Option<&str>, era_str: Option<&str>) -> Option<i32> {
        let year_text = year_str?;
        let mut year: i32 = year_text.parse().ok()?;

        // Handle two-digit years if needed (though typically 4 digits for ranges/formal dates)
        if year < 100 && era_str.is_none() {
            year = if year > 50 { 1900 + year } else { 2000 + year };
        }

        // Handle era suffixes
        if let Some(era) = era_str {
            let era_lower = era.to_lowercase().replace([' ', '.'], "");

            // AC (Before Christ)
            if era_lower == "ac" {
                return Some(-year);
            }
            // dC (After Christ) - default
        }

        Some(year)
    }
}

impl Default for PTMonthNameParser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser for PTMonthNameParser {
    fn name(&self) -> &'static str {
        "PTMonthNameParser"
    }

    fn should_apply(&self, _context: &ParsingContext) -> bool {
        true
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();
        let ref_date = context.reference.instant;

        // Try abbreviated pattern (Dom 15Set)
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

        // Try cross-month range pattern
        start = 0;
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

            let start_month = get_month(start_month_str.trim_end_matches('.')).unwrap_or(1);
            let end_month = get_month(end_month_str.trim_end_matches('.')).unwrap_or(1);

            let start_day: i32 = start_day_str.parse().unwrap_or(1);
            let end_day: i32 = end_day_str.parse().unwrap_or(1);

            let mut start_components = context.create_components();
            let mut end_components = context.create_components();

            if let Some(y) = year_str {
                let year = y.parse::<i32>().unwrap_or(ref_date.year());
                start_components.assign(Component::Year, year);
                end_components.assign(Component::Year, year);
            } else {
                start_components.imply(Component::Year, ref_date.year());
                end_components.imply(Component::Year, ref_date.year());
            }

            start_components.assign(Component::Month, start_month as i32);
            start_components.assign(Component::Day, start_day);

            end_components.assign(Component::Month, end_month as i32);
            end_components.assign(Component::Day, end_day);

            results.push(context.create_result(
                match_start,
                match_end,
                start_components,
                Some(end_components),
            ));
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

            if let Some(ref wd_str) = weekday_str {
                // Need to strip punctuation/trailing space from weekday match if captured roughly?
                // Regex should capture clean word.
                // But regex `domingo|dom\.?|...` matches "dom." or "dom".
                // Check if trailing comma/space is outside capture group. Yes, `\s*,?\s*` is outside `weekday` group.
                let wd_clean = wd_str.trim_end_matches(['.', ',']);
                if let Some(weekday) = get_weekday(wd_clean) {
                    components.assign(Component::Weekday, weekday as i32);
                }
            }

            if !components.is_valid_date() {
                start = match_end;
                continue;
            }

            // Handle end date for ranges
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
