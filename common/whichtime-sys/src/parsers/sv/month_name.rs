//! Swedish month name parser
//!
//! Handles Swedish date expressions with month names like:
//! - "15 augusti 2012"
//! - "15 aug 2012"
//! - "den 15 augusti"
//! - "15-16 augusti" (date ranges)
//! - "15 till 16 augusti" (date ranges)

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::sv::get_month;
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use chrono::Datelike;
use fancy_regex::Regex;
use std::sync::LazyLock;

static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?ix)
        (?:den\s+)?
        (?P<day>\d{1,2})
        (?:
            \s*(?:-|–|till)\s*
            (?P<end_day>\d{1,2})
        )?
        \s+
        (?P<month>januari|februari|mars|april|maj|juni|juli|augusti|september|oktober|november|december|jan\.?|feb\.?|mar\.?|apr\.?|jun\.?|jul\.?|aug\.?|sep\.?|sept\.?|okt\.?|nov\.?|dec\.?)
        (?:
            \s+
            (?P<year>\d{4}|\d{2})
        )?
        (?![a-zA-ZåäöÅÄÖ])"
    ).unwrap()
});

/// Swedish month name parser
pub struct SVMonthNameParser;

impl SVMonthNameParser {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SVMonthNameParser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser for SVMonthNameParser {
    fn name(&self) -> &'static str {
        "SVMonthNameParser"
    }

    fn should_apply(&self, _context: &ParsingContext) -> bool {
        true
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();
        let ref_date = context.reference.instant;

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

            let day_str = captures.name("day").map(|m| m.as_str());
            let month_str = captures
                .name("month")
                .map(|m| m.as_str().to_lowercase())
                .unwrap_or_default();
            let year_str = captures.name("year").map(|m| m.as_str());
            let end_day_str = captures.name("end_day").map(|m| m.as_str());

            // Parse day
            let day: i32 = day_str.and_then(|d| d.parse().ok()).unwrap_or(1);

            // Parse month - remove trailing dot if present
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
