//! Spanish month name parser
//!
//! Handles Spanish date formats with month names like:
//! - "10 Agosto 2012"
//! - "10 de Agosto de 2012"
//! - "10 - 22 Agosto 2012" (ranges)
//! - "10 a 22 Agosto 2012" (ranges with "a")
//! - "10 Agosto 234 AC" (year suffixes)
//! - "Dom 15Sep" (abbreviated weekday + date)
//! - "martes, 10 de enero" (weekday prefix)

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::es as dict;
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use crate::scanner::TokenType;
use chrono::Datelike;
use fancy_regex::Regex;
use std::sync::LazyLock;

// Main pattern for date with month name (with optional "el" that won't be captured)
static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)(?:el\s+)?(?:(?P<weekday>lunes|martes|miércoles|miercoles|jueves|viernes|sábado|sabado|domingo|lun|mar|mié|mie|jue|vie|sáb|sab|dom)[,\s]+)?(?P<day>\d{1,2})(?:º|ª|°)?(?:\s*(?:de|desde|-|–|a)\s*(?P<end_day>\d{1,2})(?:º|ª|°)?)?\s*(?:de\s+)?(?P<month>enero|febrero|marzo|abril|mayo|junio|julio|agosto|septiembre|setiembre|octubre|noviembre|diciembre|ene|feb|mar|abr|may|jun|jul|ago|sep|set|oct|nov|dic)(?:\s*(?:de|,)?\s*(?P<year>\d{1,4}))?(?:\s*(?P<era>a\.?\s*c\.?|d\.?\s*c\.?|a\.?\s*d\.?))?"
    ).unwrap()
});

// Pattern for abbreviated weekday + day + month (Dom 15Sep, DOM 15SEP)
static ABBREV_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)(?P<weekday>lun\.?|mar\.?|mié\.?|mie\.?|jue\.?|vie\.?|sáb\.?|sab\.?|dom\.?)\s*(?P<day>\d{1,2})\s*(?P<month>ene\.?|feb\.?|mar\.?|abr\.?|may\.?|jun\.?|jul\.?|ago\.?|sep\.?|set\.?|oct\.?|nov\.?|dic\.?)"
    ).unwrap()
});

// Pattern for cross-month ranges
static RANGE_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)(?P<start_day>\d{1,2})\s*(?P<start_month>enero|febrero|marzo|abril|mayo|junio|julio|agosto|septiembre|setiembre|octubre|noviembre|diciembre|ene|feb|mar|abr|may|jun|jul|ago|sep|set|oct|nov|dic)\s*(?:-|–|a)\s*(?P<end_day>\d{1,2})\s*(?P<end_month>enero|febrero|marzo|abril|mayo|junio|julio|agosto|septiembre|setiembre|octubre|noviembre|diciembre|ene|feb|mar|abr|may|jun|jul|ago|sep|set|oct|nov|dic)(?:\s*(?:de|,)?\s*(?P<year>\d{1,4}))?"
    ).unwrap()
});

/// Spanish month name parser
pub struct ESMonthNameParser;

impl ESMonthNameParser {
    pub fn new() -> Self {
        Self
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
            let era_lower = era.to_lowercase().replace(['.', ' '], "");
            if era_lower == "ac" {
                year = -year;
            }
            // "d.c." and "a.d." keep positive year
        }

        year
    }
}

impl Parser for ESMonthNameParser {
    fn name(&self) -> &'static str {
        "ESMonthNameParser"
    }

    fn should_apply(&self, context: &ParsingContext) -> bool {
        // Check for month token or digit patterns that might contain abbreviated months
        context.has_token_type(TokenType::Month) || context.text.bytes().any(|b| b.is_ascii_digit())
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();
        let ref_date = context.reference.instant;

        // Try cross-month range pattern first
        let mut start = 0;
        while start < context.text.len() {
            let search_text = &context.text[start..];

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
                    let year = caps
                        .name("year")
                        .map(|y| Self::parse_year_with_era(y.as_str(), None))
                        .unwrap_or(ref_date.year());

                    let full_match = caps.get(0).unwrap();
                    let match_start = start + full_match.start();
                    let match_end = start + full_match.end();

                    let mut components = context.create_components();
                    components.assign(Component::Year, year);
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

                if day > 0 && day <= 31 && month > 0 {
                    let full_match = caps.get(0).unwrap();
                    let matched_text = full_match.as_str();

                    // Skip "el " prefix if present
                    let prefix_offset = if matched_text.to_lowercase().starts_with("el ") {
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

                    let mut components = context.create_components();
                    let has_weekday = caps.name("weekday").is_some();

                    if let Some(y) = year {
                        components.assign(Component::Year, y);
                    } else if has_weekday {
                        // When weekday is specified, assign year to prevent ForwardDateRefiner from moving it
                        components.assign(Component::Year, ref_date.year());
                    } else {
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

                    results.push(context.create_result(
                        match_start,
                        match_end,
                        components,
                        end_comp,
                    ));
                    start = match_end;
                    continue;
                }
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

impl Default for ESMonthNameParser {
    fn default() -> Self {
        Self::new()
    }
}
