//! Russian time unit relative parser
//!
//! Handles Russian relative time expressions like:
//! - "следующие 2 недели" (next 2 weeks)
//! - "на этой неделе" (this week)
//! - "прошлые 3 дня" (past 3 days)
//! - "5 дней назад" (5 days ago)

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::ru::{get_time_unit, parse_number_pattern};
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use crate::types::{Duration, TimeUnit, add_duration};
use chrono::{Datelike, Timelike};
use fancy_regex::Regex;
use std::sync::LazyLock;

// Pattern for "следующие N единиц" (next N units), "прошлые N единиц" (past N units)
static RELATIVE_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)(?<![а-яА-Я])(?:(?P<modifier>следующи[еих]|ближайши[еих]|прошлы[еих]|последни[еих]|предыдущи[еих])\s+)?(?P<num>\d+|один|одна|одну|два|две|три|четыре|пять|шесть|семь|восемь|девять|десять)\s+(?P<unit>секунд[уы]?|минут[уы]?|час(?:ов|а)?|дн(?:ей|я|и|ь)?|день|недел[юиьей]|месяц(?:ев|а)?|год(?:а|ов)?|лет)(?![а-яА-Я])"
    ).unwrap()
});

// Pattern for "на этой/следующей/прошлой неделе" (this/next/last week)
static THIS_WEEK_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)(?<![а-яА-Я])(?:на|в)\s+(?P<modifier>это[йм]|следующ(?:ей|ем|ую)|прошло[йм]|будущ(?:ей|ем|ую))\s+(?P<unit>недел[еюи]|месяц[еа]?|году?)(?![а-яА-Я])"
    ).unwrap()
});

// Pattern for "N дней/недель/... назад" (N days/weeks/... ago)
static AGO_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)(?<![а-яА-Я])(?P<num>\d+|один|одна|одну|два|две|три|четыре|пять|шесть|семь|восемь|девять|десять)\s+(?P<unit>секунд[уы]?|минут[уы]?|час(?:ов|а)?|дн(?:ей|я|и|ь)?|день|недел[юиьей]|месяц(?:ев|а)?|год(?:а|ов)?|лет)\s+назад(?![а-яА-Я])"
    ).unwrap()
});

/// Russian time unit relative parser
pub struct RUTimeUnitRelativeParser;

impl RUTimeUnitRelativeParser {
    pub fn new() -> Self {
        Self
    }

    fn parse_unit(unit_str: &str) -> Option<TimeUnit> {
        let lower = unit_str.to_lowercase();
        get_time_unit(&lower)
    }
}

impl Default for RUTimeUnitRelativeParser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser for RUTimeUnitRelativeParser {
    fn name(&self) -> &'static str {
        "RUTimeUnitRelativeParser"
    }

    fn should_apply(&self, _context: &ParsingContext) -> bool {
        true
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();
        let ref_date = context.reference.instant;

        // Parse "N дней назад" patterns (time units ago)
        let mut start = 0;
        while start < context.text.len() {
            let search_text = &context.text[start..];
            let captures = match AGO_PATTERN.captures(search_text) {
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

            let num_str = captures.name("num").map(|m| m.as_str()).unwrap_or("1");
            let unit_str = captures
                .name("unit")
                .map(|m| m.as_str())
                .unwrap_or_default();

            let num = parse_number_pattern(num_str);
            if let Some(unit) = Self::parse_unit(unit_str) {
                let mut duration = Duration::new();
                match unit {
                    TimeUnit::Second => duration.second = Some(-num),
                    TimeUnit::Minute => duration.minute = Some(-num),
                    TimeUnit::Hour => duration.hour = Some(-num),
                    TimeUnit::Day => duration.day = Some(-num),
                    TimeUnit::Week => duration.week = Some(-num),
                    TimeUnit::Month => duration.month = Some(-num),
                    TimeUnit::Year => duration.year = Some(-num),
                    _ => {}
                }

                let target_date = add_duration(ref_date, &duration);

                let mut components = context.create_components();
                components.assign(Component::Year, target_date.year());
                components.assign(Component::Month, target_date.month() as i32);
                components.assign(Component::Day, target_date.day() as i32);

                if duration.has_time_component() {
                    components.assign(Component::Hour, target_date.hour() as i32);
                    components.assign(Component::Minute, target_date.minute() as i32);
                    components.assign(Component::Second, target_date.second() as i32);
                }

                results.push(context.create_result(match_start, match_end, components, None));
            }

            start = match_end;
        }

        // Parse "следующие N единиц" patterns (relative future)
        start = 0;
        while start < context.text.len() {
            let search_text = &context.text[start..];
            let captures = match RELATIVE_PATTERN.captures(search_text) {
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

            // Skip if overlaps with existing results
            let overlaps = results.iter().any(|r| {
                (match_start >= r.index && match_start < r.index + r.text.len())
                    || (r.index >= match_start && r.index < match_end)
            });
            if overlaps {
                start = match_end;
                continue;
            }

            let modifier = captures.name("modifier").map(|m| m.as_str().to_lowercase());
            let num_str = captures.name("num").map(|m| m.as_str()).unwrap_or("1");
            let unit_str = captures
                .name("unit")
                .map(|m| m.as_str())
                .unwrap_or_default();

            let num = parse_number_pattern(num_str);
            if let Some(unit) = Self::parse_unit(unit_str) {
                // Determine direction based on modifier
                let is_past = modifier.as_ref().is_some_and(|m| {
                    m.starts_with("прошл") || m.starts_with("последн") || m.starts_with("предыдущ")
                });

                let multiplier = if is_past { -1.0 } else { 1.0 };
                let adjusted_num = num * multiplier;

                let mut duration = Duration::new();
                match unit {
                    TimeUnit::Second => duration.second = Some(adjusted_num),
                    TimeUnit::Minute => duration.minute = Some(adjusted_num),
                    TimeUnit::Hour => duration.hour = Some(adjusted_num),
                    TimeUnit::Day => duration.day = Some(adjusted_num),
                    TimeUnit::Week => duration.week = Some(adjusted_num),
                    TimeUnit::Month => duration.month = Some(adjusted_num),
                    TimeUnit::Year => duration.year = Some(adjusted_num),
                    _ => {}
                }

                let target_date = add_duration(ref_date, &duration);

                let mut components = context.create_components();
                components.assign(Component::Year, target_date.year());
                components.assign(Component::Month, target_date.month() as i32);
                components.assign(Component::Day, target_date.day() as i32);

                if duration.has_time_component() {
                    components.assign(Component::Hour, target_date.hour() as i32);
                    components.assign(Component::Minute, target_date.minute() as i32);
                    components.assign(Component::Second, target_date.second() as i32);
                } else {
                    components.imply(Component::Hour, ref_date.hour() as i32);
                    components.imply(Component::Minute, ref_date.minute() as i32);
                }

                results.push(context.create_result(match_start, match_end, components, None));
            }

            start = match_end;
        }

        // Parse "на этой/следующей неделе" patterns
        start = 0;
        while start < context.text.len() {
            let search_text = &context.text[start..];
            let captures = match THIS_WEEK_PATTERN.captures(search_text) {
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

            // Skip if overlaps with existing results
            let overlaps = results.iter().any(|r| {
                (match_start >= r.index && match_start < r.index + r.text.len())
                    || (r.index >= match_start && r.index < match_end)
            });
            if overlaps {
                start = match_end;
                continue;
            }

            let modifier = captures
                .name("modifier")
                .map(|m| m.as_str().to_lowercase())
                .unwrap_or_default();
            let unit_str = captures
                .name("unit")
                .map(|m| m.as_str())
                .unwrap_or_default();

            // Determine offset based on modifier
            let offset = if modifier.starts_with("это") {
                0 // this week/month/year
            } else if modifier.starts_with("следующ") || modifier.starts_with("будущ") {
                1 // next week/month/year
            } else if modifier.starts_with("прошл") {
                -1 // last week/month/year
            } else {
                0
            };

            // Determine unit
            let unit = if unit_str.starts_with("недел") {
                Some(TimeUnit::Week)
            } else if unit_str.starts_with("месяц") {
                Some(TimeUnit::Month)
            } else if unit_str.starts_with("год") {
                Some(TimeUnit::Year)
            } else {
                None
            };

            if let Some(time_unit) = unit {
                let mut duration = Duration::new();
                match time_unit {
                    TimeUnit::Week => duration.week = Some(offset as f64),
                    TimeUnit::Month => duration.month = Some(offset as f64),
                    TimeUnit::Year => duration.year = Some(offset as f64),
                    _ => {}
                }

                let target_date = add_duration(ref_date, &duration);

                let mut components = context.create_components();
                components.assign(Component::Year, target_date.year());
                components.assign(Component::Month, target_date.month() as i32);
                components.assign(Component::Day, target_date.day() as i32);
                components.imply(Component::Hour, ref_date.hour() as i32);
                components.imply(Component::Minute, ref_date.minute() as i32);

                results.push(context.create_result(match_start, match_end, components, None));
            }

            start = match_end;
        }

        Ok(results)
    }
}
