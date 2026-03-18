//! French month name parser
//!
//! Handles French date formats with month names like:
//! - "10 août 2012"
//! - "le 10 août"
//! - "10 - 22 août 2012" (ranges)
//! - "10 au 22 août 2012" (ranges with "au")
//! - "10 août 234 AC" (year suffixes)
//! - "Dim 15 Sept" (abbreviated weekday + date)
//! - "Mardi 10 janvier" (weekday prefix)

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::fr as dict;
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use crate::scanner::TokenType;
use chrono::Datelike;
use fancy_regex::Regex;
use std::sync::LazyLock;

// Main pattern for date with month name
// Year pattern requires digits NOT followed by : (to avoid matching time like "12:00")
static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)(?:le\s+)?(?:(?P<weekday>lundi|mardi|mercredi|jeudi|vendredi|samedi|dimanche|lun|mar|mer|jeu|ven|sam|dim)\.?\s+)?(?P<day>\d{1,2})(?:er|ème|e)?(?:\s*(?:-|–|au?)\s*(?P<end_day>\d{1,2})(?:er|ème|e)?)?\s+(?P<month>janvier|février|fevrier|mars|avril|mai|juin|juillet|août|aout|septembre|octobre|novembre|décembre|decembre|jan|janv|fév|fev|mar|avr|jun|juil|jul|aoû|aou|sep|sept|oct|nov|déc|dec)\.?(?:\s+(?P<year>\d{1,4})(?![\d:]))?(?:\s*(?P<era>a\.?\s*c\.?|p\.?\s*chr\.?\s*n\.?|av\.?\s*j\.?-?c\.?))?"
    ).unwrap()
});

// Pattern for abbreviated weekday + day + month (Dim 15 Sept, DIM 15SEPT)
static ABBREV_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)(?P<weekday>lun|mar|mer|jeu|ven|sam|dim)\.?\s*(?P<day>\d{1,2})\s*(?P<month>jan|janv|fév|fev|mar|avr|mai|jun|juil|jul|aoû|aou|sep|sept|oct|nov|déc|dec)t?\.?"
    ).unwrap()
});

// Pattern for cross-month ranges (handles "24 août 2023 au 26 août 2023")
static RANGE_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)(?P<start_day>\d{1,2})\s*(?P<start_month>janvier|février|fevrier|mars|avril|mai|juin|juillet|août|aout|septembre|octobre|novembre|décembre|decembre|jan|janv|fév|fev|mar|avr|jun|juil|jul|aoû|aou|sep|sept|oct|nov|déc|dec)\.?(?:\s+(?P<start_year>\d{4}))?\s*(?:-|–|au?)\s*(?P<end_day>\d{1,2})\s*(?P<end_month>janvier|février|fevrier|mars|avril|mai|juin|juillet|août|aout|septembre|octobre|novembre|décembre|decembre|jan|janv|fév|fev|mar|avr|jun|juil|jul|aoû|aou|sep|sept|oct|nov|déc|dec)\.?(?:\s+(?P<end_year>\d{1,4}))?"
    ).unwrap()
});

/// French month name parser
pub struct FRMonthNameParser;

impl FRMonthNameParser {
    pub fn new() -> Self {
        Self
    }

    fn is_valid_date(year: i32, month: u32, day: u32) -> bool {
        if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
            return false;
        }
        // Check days in month
        let days_in_month = match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                // Leap year check
                if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                    29
                } else {
                    28
                }
            }
            _ => return false,
        };
        day <= days_in_month
    }

    fn parse_year_with_era(year_str: &str, era_str: Option<&str>) -> i32 {
        let mut year: i32 = year_str.parse().unwrap_or(0);

        // Check if era is present - if so, don't convert 2-digit years
        let has_era = era_str.is_some() && !era_str.unwrap().trim().is_empty();

        // Handle 2-digit years only if no era suffix
        if !has_era && year < 100 {
            year = if year > 50 { 1900 + year } else { 2000 + year };
        }

        // Handle era suffixes
        if let Some(era) = era_str {
            let era_lower = era.to_lowercase().replace(['.', ' ', '-'], "");
            if era_lower.contains("ac") || era_lower.contains("av") {
                year = -year;
            }
            // "p. Chr. n." and similar keep positive year
        }

        year
    }
}

impl Parser for FRMonthNameParser {
    fn name(&self) -> &'static str {
        "FRMonthNameParser"
    }

    fn should_apply(&self, context: &ParsingContext) -> bool {
        context.has_token_type(TokenType::Month) || context.text.bytes().any(|b| b.is_ascii_digit())
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();
        let ref_date = context.reference.instant;

        let mut start = 0;
        while start < context.text.len() {
            let search_text = &context.text[start..];

            // Try cross-month range pattern first
            if let Ok(Some(caps)) = RANGE_PATTERN.captures(search_text)
                && let (Some(sd), Some(sm), Some(ed), Some(em)) = (
                    caps.name("start_day"),
                    caps.name("start_month"),
                    caps.name("end_day"),
                    caps.name("end_month"),
                )
            {
                let start_day: u32 = sd.as_str().parse().unwrap_or(0);
                let start_month = dict::get_month(&sm.as_str().to_lowercase()).unwrap_or(0);
                let end_day: u32 = ed.as_str().parse().unwrap_or(0);
                let end_month = dict::get_month(&em.as_str().to_lowercase()).unwrap_or(0);

                if start_day > 0
                    && start_day <= 31
                    && start_month > 0
                    && end_day > 0
                    && end_day <= 31
                    && end_month > 0
                {
                    // Get years - start_year or end_year, preferring end_year if both present
                    let start_year = caps
                        .name("start_year")
                        .map(|y| Self::parse_year_with_era(y.as_str(), None));
                    let end_year = caps
                        .name("end_year")
                        .map(|y| Self::parse_year_with_era(y.as_str(), None));

                    let year = end_year.or(start_year).unwrap_or(ref_date.year());

                    let full_match = caps.get(0).unwrap();
                    let match_start = start + full_match.start();
                    let match_end = start + full_match.end();

                    let mut components = context.create_components();
                    components.assign(Component::Year, start_year.unwrap_or(year));
                    components.assign(Component::Month, start_month as i32);
                    components.assign(Component::Day, start_day as i32);

                    let mut end_comp = context.create_components();
                    end_comp.assign(Component::Year, year);
                    end_comp.assign(Component::Month, end_month as i32);
                    end_comp.assign(Component::Day, end_day as i32);

                    results.push(context.create_result(
                        match_start,
                        match_end,
                        components,
                        Some(end_comp),
                    ));
                    start = match_end;
                    continue;
                }
            }

            // Try abbreviated weekday pattern
            if let Ok(Some(caps)) = ABBREV_PATTERN.captures(search_text)
                && let (Some(d), Some(m)) = (caps.name("day"), caps.name("month"))
            {
                let day: u32 = d.as_str().parse().unwrap_or(0);
                let month = dict::get_month(&m.as_str().to_lowercase()).unwrap_or(0);

                if day > 0 && day <= 31 && month > 0 {
                    let full_match = caps.get(0).unwrap();
                    let match_start = start + full_match.start();
                    let match_end = start + full_match.end();

                    let mut components = context.create_components();
                    components.imply(Component::Year, ref_date.year());
                    components.assign(Component::Month, month as i32);
                    components.assign(Component::Day, day as i32);

                    // Add weekday if present
                    if let Some(wd) = caps.name("weekday")
                        && let Some(weekday) = dict::get_weekday(&wd.as_str().to_lowercase())
                    {
                        components.assign(Component::Weekday, weekday as i32);
                    }

                    results.push(context.create_result(match_start, match_end, components, None));
                    start = match_end;
                    continue;
                }
            }

            // Try main pattern
            if let Ok(Some(caps)) = PATTERN.captures(search_text)
                && let (Some(d), Some(m)) = (caps.name("day"), caps.name("month"))
            {
                let day: u32 = d.as_str().parse().unwrap_or(0);
                let month = dict::get_month(&m.as_str().to_lowercase()).unwrap_or(0);

                let full_match = caps.get(0).unwrap();
                let matched_text = full_match.as_str();

                // Skip "le " prefix if present
                let prefix_offset = if matched_text.to_lowercase().starts_with("le ") {
                    3
                } else {
                    0
                };

                let match_start = start + full_match.start() + prefix_offset;
                let match_end = start + full_match.end();

                let year = caps.name("year").map(|y| {
                    let era = caps.name("era").map(|e| e.as_str());
                    Self::parse_year_with_era(y.as_str(), era)
                });

                let actual_year = year.unwrap_or(ref_date.year());

                // Validate the date
                if day == 0 || month == 0 || !Self::is_valid_date(actual_year, month, day) {
                    start = match_end;
                    continue;
                }

                let mut components = context.create_components();
                if let Some(y) = year {
                    components.assign(Component::Year, y);
                } else {
                    // No explicit year - imply it so ForwardDateRefiner can adjust
                    components.imply(Component::Year, ref_date.year());
                }
                components.assign(Component::Month, month as i32);
                components.assign(Component::Day, day as i32);

                // Add weekday if present
                if let Some(wd) = caps.name("weekday")
                    && let Some(weekday) = dict::get_weekday(&wd.as_str().to_lowercase())
                {
                    components.assign(Component::Weekday, weekday as i32);
                }

                // Handle date range within same month
                let end_comp = if let Some(end_day_match) = caps.name("end_day") {
                    let end_day: u32 = end_day_match.as_str().parse().unwrap_or(0);
                    if end_day > 0 && end_day <= 31 {
                        let mut ec = context.create_components();
                        if let Some(y) = year {
                            ec.assign(Component::Year, y);
                        } else {
                            ec.imply(Component::Year, ref_date.year());
                        }
                        ec.assign(Component::Month, month as i32);
                        ec.assign(Component::Day, end_day as i32);
                        Some(ec)
                    } else {
                        None
                    }
                } else {
                    None
                };

                results.push(context.create_result(match_start, match_end, components, end_comp));
                start = match_end;
                continue;
            }

            // No match at this position - advance to next character (UTF-8 safe)
            if let Some(c) = search_text.chars().next() {
                start += c.len_utf8();
            } else {
                break;
            }
        }

        Ok(results)
    }
}

impl Default for FRMonthNameParser {
    fn default() -> Self {
        Self::new()
    }
}
