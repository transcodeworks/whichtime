//! Ukrainian month name parser
//!
//! Handles Ukrainian date expressions with month names like:
//! - "10 серпня 2012"
//! - "3 лют 82" (abbreviated month)
//! - "10.08.2012" (European dot format)
//! - "Четвер, 10 січня" (weekday + date)
//! - "10 - 22 серпня 2012" (date ranges)
//! - "неділя, 7 грудня 2014" (weekday + date)

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::uk::{get_month, get_weekday};
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use chrono::Datelike;
use fancy_regex::Regex;
use std::sync::LazyLock;

// Pattern for "DD.MM.YYYY" European format
static DOT_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?<![0-9])(?P<day>\d{1,2})\.(?P<month>\d{1,2})\.(?P<year>\d{4}|\d{2})(?![0-9])")
        .unwrap()
});

// Pattern for dates with month names
static MONTH_NAME_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?ix)
        (?:
            (?P<weekday>понеділок|вівторок|середа|середу|четвер|п'ятниця|п'ятницю|пятниця|субота|суботу|неділя|неділю|пн|вт|ср|чт|пт|сб|нд)(?:\.|,)?\s*
            (?:,\s*)?
        )?
        (?P<day>\d{1,2})
        (?:
            \s*(?:-|–|до)\s*
            (?P<end_day>\d{1,2})
        )?
        \s+
        (?P<month>січня?|лютого?|лют\.?|березня?|бер\.?|квітня?|квіт\.?|травня?|трав\.?|червня?|черв\.?|липня?|лип\.?|серпня?|серп\.?|вересня?|вер\.?|жовтня?|жовт\.?|листопада?|лист\.?|грудня?|груд\.?)
        (?:
            \s+
            (?P<year>\d{4}|\d{2})
            (?:\s*р\.?)?
        )?
        (?![а-яА-ЯіїєґІЇЄҐ])"
    ).unwrap()
});

/// Ukrainian month name parser
pub struct UKMonthNameParser;

impl UKMonthNameParser {
    pub fn new() -> Self {
        Self
    }
}

impl Default for UKMonthNameParser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser for UKMonthNameParser {
    fn name(&self) -> &'static str {
        "UKMonthNameParser"
    }

    fn should_apply(&self, _context: &ParsingContext) -> bool {
        true
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();
        let ref_date = context.reference.instant;

        // Parse DD.MM.YYYY format first
        let mut start = 0;
        while start < context.text.len() {
            let search_text = &context.text[start..];
            let captures = match DOT_PATTERN.captures(search_text) {
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

            let day: i32 = captures
                .name("day")
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);
            let month: i32 = captures
                .name("month")
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);
            let year_str = captures.name("year").map(|m| m.as_str());

            if !(1..=31).contains(&day) || !(1..=12).contains(&month) {
                start = match_end;
                continue;
            }

            let mut components = context.create_components();

            if let Some(y) = year_str {
                let mut year: i32 = y.parse().unwrap_or(ref_date.year());
                if year < 100 {
                    year = if year > 50 { 1900 + year } else { 2000 + year };
                }
                components.assign(Component::Year, year);
            } else {
                components.imply(Component::Year, ref_date.year());
            }

            components.assign(Component::Month, month);
            components.assign(Component::Day, day);

            if components.is_valid_date() {
                results.push(context.create_result(match_start, match_end, components, None));
            }

            start = match_end;
        }

        // Parse month name patterns
        start = 0;
        while start < context.text.len() {
            let search_text = &context.text[start..];
            let captures = match MONTH_NAME_PATTERN.captures(search_text) {
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

            // Skip if overlaps with dot pattern results
            let overlaps = results.iter().any(|r| {
                (match_start >= r.index && match_start < r.index + r.text.len())
                    || (r.index >= match_start && r.index < match_end)
            });
            if overlaps {
                start = match_end;
                continue;
            }

            let weekday_str = captures.name("weekday").map(|m| m.as_str().to_lowercase());
            let day: i32 = captures
                .name("day")
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);
            let month_str = captures
                .name("month")
                .map(|m| m.as_str().to_lowercase())
                .unwrap_or_default();
            let year_str = captures.name("year").map(|m| m.as_str());
            let end_day_str = captures.name("end_day").map(|m| m.as_str());

            // Parse month - remove trailing dots
            let clean_month = month_str.trim_end_matches('.');
            let month = get_month(clean_month).unwrap_or(0);

            if month == 0 || !(1..=31).contains(&day) {
                start = match_end;
                continue;
            }

            let mut components = context.create_components();

            // Parse year
            if let Some(y) = year_str {
                let mut year: i32 = y.parse().unwrap_or(ref_date.year());
                if year < 100 {
                    year = if year > 50 { 1900 + year } else { 2000 + year };
                }
                components.assign(Component::Year, year);
            } else {
                components.imply(Component::Year, ref_date.year());
            }

            components.assign(Component::Month, month as i32);
            components.assign(Component::Day, day);

            // Add weekday if present
            if let Some(ref wd_str) = weekday_str {
                let clean_wd = wd_str.trim_end_matches('.').trim_end_matches(',');
                if let Some(weekday) = get_weekday(clean_wd) {
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
                        if year_str.is_some() {
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
