//! Portuguese weekday parser
//!
//! Handles Portuguese weekday expressions like:
//! - "segunda", "na terça-feira"
//! - "próxima sexta", "sexta passada"
//! - "sexta que vem", "domingo anterior"

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::RelativeModifier;
use crate::dictionaries::pt::{get_relative_modifier, get_weekday};
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use chrono::{Datelike, Duration};
use fancy_regex::Regex;
use std::sync::LazyLock;

static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?ix)
        (?<![a-zA-Z])
        (?:(?:em|na|no)\s+)?
        (?:
            (?P<prefix>esta|este|próxima|proxima|próximo|proximo|passada|passado|última|ultima|último|ultimo)
            \s+
        )?
        (?P<weekday>domingo|dom\.?|segunda(?:-feira)?|seg\.?|terça(?:-feira)?|terca(?:-feira)?|ter\.?|quarta(?:-feira)?|qua\.?|quinta(?:-feira)?|qui\.?|sexta(?:-feira)?|sex\.?|sábado|sabado|sab\.?)
        (?:
            \s+
            (?P<suffix_mod>passada|passado|que\s+vem|seguinte|anterior|próxima|proxima|próximo|proximo)
        )?
        (?=\W|$)
        "
    ).unwrap()
});

/// Portuguese weekday parser
pub struct PTWeekdayParser;

impl PTWeekdayParser {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PTWeekdayParser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser for PTWeekdayParser {
    fn name(&self) -> &'static str {
        "PTWeekdayParser"
    }

    fn should_apply(&self, _context: &ParsingContext) -> bool {
        true
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();
        let ref_date = context.reference.instant;
        let ref_weekday = ref_date.weekday().num_days_from_sunday() as i64;

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
            let matched_text = full_match.as_str();

            let weekday_str = captures
                .name("weekday")
                .map(|m| m.as_str().to_lowercase())
                .unwrap_or_default();
            let prefix_str = captures.name("prefix").map(|m| m.as_str().to_lowercase());
            let suffix_mod_str = captures
                .name("suffix_mod")
                .map(|m| m.as_str().to_lowercase());

            let Some(weekday) = get_weekday(&weekday_str) else {
                start = match_end;
                continue;
            };

            let target_weekday = weekday as i64;

            // Determine modifier from prefix or suffix
            let modifier = prefix_str
                .as_ref()
                .and_then(|s| get_relative_modifier(s))
                .or_else(|| {
                    suffix_mod_str.as_ref().and_then(|s| {
                        if s.contains("que vem") || s.contains("seguinte") {
                            Some(RelativeModifier::Next)
                        } else {
                            get_relative_modifier(s)
                        }
                    })
                });

            // Calculate days offset
            let days_offset = match modifier {
                Some(RelativeModifier::Next) => {
                    let diff = target_weekday - ref_weekday;
                    if diff <= 0 { diff + 7 } else { diff }
                }
                Some(RelativeModifier::Last) => {
                    let diff = target_weekday - ref_weekday;
                    if diff >= 0 { diff - 7 } else { diff }
                }
                Some(RelativeModifier::This) | None => {
                    // Default behavior: find closest day
                    let diff = target_weekday - ref_weekday;
                    if diff == 0 {
                        0
                    } else if diff > 0 {
                        if diff <= 3 { diff } else { diff - 7 }
                    } else if diff >= -3 {
                        diff
                    } else {
                        diff + 7
                    }
                }
            };

            let target_date = ref_date + Duration::days(days_offset);

            let mut components = context.create_components();
            components.assign(Component::Year, target_date.year());
            components.assign(Component::Month, target_date.month() as i32);
            components.assign(Component::Day, target_date.day() as i32);
            components.assign(Component::Weekday, target_weekday as i32);

            // Find actual text bounds (trim leading/trailing non-alphanumeric)
            let actual_start = matched_text
                .find(|c: char| c.is_alphanumeric())
                .unwrap_or(0);
            let actual_end = matched_text
                .rfind(|c: char| c.is_alphanumeric())
                .map(|i| i + matched_text[i..].chars().next().map_or(1, char::len_utf8))
                .unwrap_or(matched_text.len());

            results.push(context.create_result(
                match_start + actual_start,
                match_start + actual_end,
                components,
                None,
            ));

            start = match_end;
        }

        Ok(results)
    }
}
