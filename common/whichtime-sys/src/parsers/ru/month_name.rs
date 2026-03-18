//! Russian month name little endian parser
//!
//! Handles Russian date expressions with month names like:
//! - "10 января 2012"
//! - "10.08.2012" (dot separator)
//! - "10 - 22 августа 2012" (date ranges)
//! - "четверг, 10 января"

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::ru::{get_month, get_weekday, parse_ordinal_pattern};
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use chrono::Datelike;
use fancy_regex::Regex;
use std::sync::LazyLock;

// Main pattern for dates like "10 августа 2012", "Четверг, 10 января"
static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?ix)
        (?:
            (?P<weekday>понедельник|вторник|среда|четверг|пятница|суббота|воскресенье|пн|вт|ср|чт|пт|сб|вс|вск)(?:\.|,)?\s*
            (?:,\s*)?
        )?
        (?:
            (?P<ordinal_day>первое|второе|третье|четвертое|пятое|шестое|седьмое|восьмое|девятое|десятое|[\w]+oe|[\w]+ье|[\w]+ое)\s+
            |
            (?P<day>\d{1,2})(?:\.|,|\s+)?
        )
        (?:
            (?:(?:\-|\–|по|до)\s*(?P<end_day>\d{1,2})(?:\.|,|\s+)?)?
        )
        (?P<month>января?|февраля?|марта?|апреля?|мая|июня?|июля?|августа?|сентября?|октября?|ноября?|декабря?|янв\.?|фев\.?|мар\.?|апр\.?|май|июн\.?|июл\.?|авг\.?|сен\.?|окт\.?|ноя\.?|дек\.?|01|02|03|04|05|06|07|08|09|10|11|12)(?:\.|,|\s+)?
        (?:
            (?:\s*[\-/,]?\s*)?
            (?:(?P<year>\d{1,4})|(?P<year_short>'\d{2}))?
            (?:\s*(?:г\.?|года?))?
        )?
        (?=\W|$)"
    ).unwrap()
});

// Pattern for "Month Year" format like "Сентябрь 2012"
static MONTH_YEAR_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?ix)
        (?P<month>январь|февраль|март|апрель|май|июнь|июль|август|сентябрь|октябрь|ноябрь|декабрь)\s+
        (?P<year>\d{4})"
    ).unwrap()
});

/// Russian month name parser
pub struct RUMonthNameParser;

impl RUMonthNameParser {
    pub fn new() -> Self {
        Self
    }
}

impl Default for RUMonthNameParser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser for RUMonthNameParser {
    fn name(&self) -> &'static str {
        "RUMonthNameParser"
    }

    fn should_apply(&self, _context: &ParsingContext) -> bool {
        true
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();
        let ref_date = context.reference.instant;

        // First, try month-year pattern (e.g., "Сентябрь 2012")
        let mut start = 0;
        while start < context.text.len() {
            let search_text = &context.text[start..];
            let captures = match MONTH_YEAR_PATTERN.captures(search_text) {
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

            let month_str = captures
                .name("month")
                .map(|m| m.as_str().to_lowercase())
                .unwrap_or_default();
            let year_str = captures.name("year").map(|m| m.as_str());

            if let Some(month) = get_month(&month_str)
                && let Some(year) = year_str.and_then(|y| y.parse::<i32>().ok())
            {
                let mut components = context.create_components();
                components.assign(Component::Year, year);
                components.assign(Component::Month, month as i32);
                components.assign(Component::Day, 1);

                results.push(context.create_result(match_start, match_end, components, None));
            }

            start = match_end;
        }

        // Then, try the main pattern for day-month-year
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

            // Skip if this match overlaps with a month-year result
            let overlaps = results.iter().any(|r| {
                (match_start >= r.index && match_start < r.index + r.text.len())
                    || (r.index >= match_start && r.index < match_end)
            });
            if overlaps {
                start = match_end;
                continue;
            }

            let weekday_str = captures.name("weekday").map(|m| m.as_str().to_lowercase());
            let day_str = captures.name("day").map(|m| m.as_str());
            let ordinal_day_str = captures.name("ordinal_day").map(|m| m.as_str());
            let month_str = captures
                .name("month")
                .map(|m| m.as_str().to_lowercase())
                .unwrap_or_default();
            let year_str = captures.name("year").map(|m| m.as_str());
            let year_short_str = captures.name("year_short").map(|m| m.as_str());
            let end_day_str = captures.name("end_day").map(|m| m.as_str());

            // Parse day
            let day = if let Some(d) = day_str {
                d.parse::<i32>().unwrap_or(1)
            } else if let Some(od) = ordinal_day_str {
                parse_ordinal_pattern(od).map(|v| v as i32).unwrap_or(1)
            } else {
                1
            };

            // Parse month
            let month = if let Ok(m_num) = month_str.trim_end_matches('.').parse::<u32>() {
                m_num
            } else {
                get_month(month_str.trim_end_matches('.')).unwrap_or(0)
            };

            if month == 0 {
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
            } else if let Some(y_short) = year_short_str {
                let val: i32 = y_short.trim_start_matches('\'').parse().unwrap_or(0);
                let year = if val > 50 { 1900 + val } else { 2000 + val };
                components.assign(Component::Year, year);
            } else {
                components.imply(Component::Year, ref_date.year());
            }

            components.assign(Component::Month, month as i32);
            components.assign(Component::Day, day);

            if let Some(ref wd_str) = weekday_str {
                // Clean up dots/commas
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
                        // If year was explicit, copy it. If implied, imply it.
                        if year_str.is_some() || year_short_str.is_some() {
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
