//! Spanish casual date parser
//!
//! Handles Spanish casual date expressions like:
//! - "ahora" (now), "hoy" (today), "mañana" (tomorrow), "ayer" (yesterday)
//! - "esta mañana" (this morning), "ayer de noche" (last night)
//! - "pasado mañana" (day after tomorrow), "anteayer" (day before yesterday)
//! - Combined: "hoy a las 5PM", "esta noche a las 8", "mañana a mediodía"

use crate::components::Component;
use crate::context::ParsingContext;
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use crate::types::Meridiem;
use chrono::{Datelike, Duration, Timelike};
use fancy_regex::Regex;
use std::sync::LazyLock;

static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)(?<![a-zA-ZáéíóúüñÁÉÍÓÚÜÑ])(ahora|hoy|esta\s+mañana|esta\s+manana|esta\s+tarde|esta\s+noche|mañana|manana|ayer|pasado\s*mañana|pasado\s*manana|anteayer|antes\s*de\s*ayer|anoche)(?:\s+(?:de\s+)?(la\s+)?(mañana|manana|tarde|noche))?(?:\s+a(?:\s+las?)?\s+(\d{1,2})(?::(\d{2}))?(?:\s*(a\.?m\.?|p\.?m\.?))?|\s+a\s+(mediodía|mediodia))?(?=\W|$)"
    ).unwrap()
});

const DATE_GROUP: usize = 1;
const TIME_GROUP: usize = 3;
const HOUR_GROUP: usize = 4;
const MINUTE_GROUP: usize = 5;
const MERIDIEM_GROUP: usize = 6;
const MEDIODIA_GROUP: usize = 7;

/// Spanish casual date parser
pub struct ESCasualDateParser;

impl ESCasualDateParser {
    pub fn new() -> Self {
        Self
    }

    fn extract_time_components(
        components: &mut crate::components::FastComponents,
        time_keyword: &str,
    ) {
        let lower = time_keyword.to_lowercase();
        if lower.contains("mañana") || lower.contains("manana") {
            // "mañana" as time of day = morning (not tomorrow)
            components.imply(Component::Hour, 6);
            components.imply(Component::Minute, 0);
            components.assign(Component::Meridiem, Meridiem::AM as i32);
        } else if lower.contains("tarde") {
            components.imply(Component::Hour, 15);
            components.imply(Component::Minute, 0);
            components.assign(Component::Meridiem, Meridiem::PM as i32);
        } else if lower.contains("noche") {
            components.imply(Component::Hour, 22);
            components.imply(Component::Minute, 0);
            components.assign(Component::Meridiem, Meridiem::PM as i32);
        }
    }
}

impl Parser for ESCasualDateParser {
    fn name(&self) -> &'static str {
        "ESCasualDateParser"
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

            let date_keyword = captures
                .get(DATE_GROUP)
                .map(|m| m.as_str().to_lowercase())
                .unwrap_or_default();

            let time_keyword = captures.get(TIME_GROUP).map(|m| m.as_str().to_lowercase());

            let mut components = context.create_components();
            let mut target_date = ref_date;

            match date_keyword.as_str() {
                "ahora" => {
                    components.assign(Component::Year, ref_date.year());
                    components.assign(Component::Month, ref_date.month() as i32);
                    components.assign(Component::Day, ref_date.day() as i32);
                    components.assign(Component::Hour, ref_date.hour() as i32);
                    components.assign(Component::Minute, ref_date.minute() as i32);
                    components.assign(Component::Second, ref_date.second() as i32);
                }
                "hoy" => {
                    components.assign(Component::Year, ref_date.year());
                    components.assign(Component::Month, ref_date.month() as i32);
                    components.assign(Component::Day, ref_date.day() as i32);
                }
                "mañana" | "manana" => {
                    // "mañana" alone = tomorrow (not morning)
                    target_date = ref_date + Duration::days(1);
                    components.assign(Component::Year, target_date.year());
                    components.assign(Component::Month, target_date.month() as i32);
                    components.assign(Component::Day, target_date.day() as i32);
                }
                "ayer" => {
                    target_date = ref_date - Duration::days(1);
                    components.assign(Component::Year, target_date.year());
                    components.assign(Component::Month, target_date.month() as i32);
                    components.assign(Component::Day, target_date.day() as i32);
                }
                _ if date_keyword.contains("pasado")
                    && (date_keyword.contains("mañana") || date_keyword.contains("manana")) =>
                {
                    // "pasado mañana" = day after tomorrow
                    target_date = ref_date + Duration::days(2);
                    components.assign(Component::Year, target_date.year());
                    components.assign(Component::Month, target_date.month() as i32);
                    components.assign(Component::Day, target_date.day() as i32);
                }
                "anteayer" => {
                    // "anteayer" = day before yesterday
                    target_date = ref_date - Duration::days(2);
                    components.assign(Component::Year, target_date.year());
                    components.assign(Component::Month, target_date.month() as i32);
                    components.assign(Component::Day, target_date.day() as i32);
                }
                _ if date_keyword.contains("antes") && date_keyword.contains("ayer") => {
                    // "antes de ayer" = day before yesterday
                    target_date = ref_date - Duration::days(2);
                    components.assign(Component::Year, target_date.year());
                    components.assign(Component::Month, target_date.month() as i32);
                    components.assign(Component::Day, target_date.day() as i32);
                }
                "anoche" => {
                    if ref_date.hour() > 6 {
                        target_date = ref_date - Duration::days(1);
                    }
                    components.assign(Component::Year, target_date.year());
                    components.assign(Component::Month, target_date.month() as i32);
                    components.assign(Component::Day, target_date.day() as i32);
                    components.imply(Component::Hour, 22);
                    components.assign(Component::Meridiem, Meridiem::PM as i32);
                }
                _ if date_keyword.contains("esta") && date_keyword.contains("noche") => {
                    components.assign(Component::Year, ref_date.year());
                    components.assign(Component::Month, ref_date.month() as i32);
                    components.assign(Component::Day, ref_date.day() as i32);
                    components.imply(Component::Hour, 22);
                    components.assign(Component::Meridiem, Meridiem::PM as i32);
                }
                _ if date_keyword.contains("esta")
                    && (date_keyword.contains("mañana") || date_keyword.contains("manana")) =>
                {
                    components.assign(Component::Year, ref_date.year());
                    components.assign(Component::Month, ref_date.month() as i32);
                    components.assign(Component::Day, ref_date.day() as i32);
                    components.imply(Component::Hour, 6);
                    components.assign(Component::Meridiem, Meridiem::AM as i32);
                }
                _ if date_keyword.contains("esta") && date_keyword.contains("tarde") => {
                    components.assign(Component::Year, ref_date.year());
                    components.assign(Component::Month, ref_date.month() as i32);
                    components.assign(Component::Day, ref_date.day() as i32);
                    components.imply(Component::Hour, 15);
                    components.assign(Component::Meridiem, Meridiem::PM as i32);
                }
                _ => {
                    start = match_end;
                    continue;
                }
            }

            // Apply time component if present (time of day modifier)
            if let Some(ref time_kw) = time_keyword {
                Self::extract_time_components(&mut components, time_kw);
            }

            // Handle explicit time expression: "a las X" or "a mediodía"
            if captures.get(MEDIODIA_GROUP).is_some() {
                // "a mediodía" = noon
                components.assign(Component::Hour, 12);
                components.assign(Component::Minute, 0);
                components.assign(Component::Meridiem, Meridiem::PM as i32);
            } else if let Some(hour_match) = captures.get(HOUR_GROUP) {
                let hour_str = hour_match.as_str();
                let mut hour: i32 = hour_str.parse().unwrap_or(0);

                let minute: i32 = captures
                    .get(MINUTE_GROUP)
                    .map(|m| m.as_str().parse().unwrap_or(0))
                    .unwrap_or(0);

                // Handle AM/PM
                let has_pm = captures
                    .get(MERIDIEM_GROUP)
                    .map(|m| m.as_str().to_lowercase().starts_with('p'))
                    .unwrap_or(false);
                let has_am = captures
                    .get(MERIDIEM_GROUP)
                    .map(|m| m.as_str().to_lowercase().starts_with('a'))
                    .unwrap_or(false);

                // Infer PM for night context
                let infer_pm = time_keyword
                    .as_ref()
                    .map(|t| t.contains("noche"))
                    .unwrap_or(false)
                    || date_keyword.contains("noche");

                if has_pm {
                    if hour < 12 {
                        hour += 12;
                    }
                    components.assign(Component::Meridiem, Meridiem::PM as i32);
                } else if has_am {
                    if hour == 12 {
                        hour = 0;
                    }
                    components.assign(Component::Meridiem, Meridiem::AM as i32);
                } else if infer_pm && hour <= 12 {
                    if hour < 12 {
                        hour += 12;
                    }
                    components.assign(Component::Meridiem, Meridiem::PM as i32);
                }

                components.assign(Component::Hour, hour);
                components.assign(Component::Minute, minute);
            }

            results.push(context.create_result(match_start, match_end, components, None));

            start = match_end;
        }

        Ok(results)
    }
}

impl Default for ESCasualDateParser {
    fn default() -> Self {
        Self::new()
    }
}
