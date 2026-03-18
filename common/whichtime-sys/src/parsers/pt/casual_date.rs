//! Portuguese casual date parser
//!
//! Handles Portuguese casual date expressions like:
//! - "hoje", "amanhã", "ontem"
//! - "esta manhã", "esta tarde", "esta noite"
//! - "ontem à noite", "amanhã ao meio-dia"
//! - "hoje às 5pm", "esta noite às 8"

use crate::components::Component;
use crate::context::ParsingContext;
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use crate::types::Meridiem;
use chrono::{Datelike, Duration};
use fancy_regex::Regex;
use std::sync::LazyLock;

static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?ix)
        (?<![a-zA-Z])
        (?:(?P<modifier>esta|este)\s+)?
        (?P<keyword>hoje|amanhã|amanha|ontem|manhã|manha|tarde|noite)
        (?:
            \s+(?:à|a|ao)\s+
            (?P<time_of_day>noite|tarde|manhã|manha|meio-dia|meia-noite)
        )?
        (?:
            \s+(?:às|as|a|ao)\s+
            (?P<hour>\d{1,2})
            (?::(?P<minute>\d{1,2}))?
            (?:\s*(?P<meridiem>pm|am))?
        )?
        (?=\W|$)
        ",
    )
    .unwrap()
});

/// Portuguese casual date parser
pub struct PTCasualDateParser;

impl PTCasualDateParser {
    pub fn new() -> Self {
        Self
    }

    fn assign_time_part(components: &mut crate::components::FastComponents, time_part: &str) {
        match time_part {
            "manhã" | "manha" => {
                components.imply(Component::Hour, 6);
                components.imply(Component::Minute, 0);
                components.assign(Component::Meridiem, Meridiem::AM as i32);
            }
            "tarde" => {
                components.imply(Component::Hour, 15);
                components.imply(Component::Minute, 0);
                components.assign(Component::Meridiem, Meridiem::PM as i32);
            }
            "noite" => {
                components.imply(Component::Hour, 22);
                components.imply(Component::Minute, 0);
                components.assign(Component::Meridiem, Meridiem::PM as i32);
            }
            "meio-dia" => {
                components.imply(Component::Hour, 12);
                components.imply(Component::Minute, 0);
                components.assign(Component::Meridiem, Meridiem::PM as i32);
            }
            "meia-noite" => {
                components.imply(Component::Hour, 0);
                components.imply(Component::Minute, 0);
            }
            _ => {}
        }
    }
}

impl Parser for PTCasualDateParser {
    fn name(&self) -> &'static str {
        "PTCasualDateParser"
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
            let matched_text = full_match.as_str().to_lowercase();

            let modifier = captures.name("modifier").map(|m| m.as_str().to_lowercase());
            let keyword = captures
                .name("keyword")
                .map(|m| m.as_str().to_lowercase())
                .unwrap_or_default();
            let time_of_day = captures
                .name("time_of_day")
                .map(|m| m.as_str().to_lowercase());
            let explicit_hour = captures
                .name("hour")
                .and_then(|m| m.as_str().parse::<i32>().ok());
            let explicit_minute = captures
                .name("minute")
                .and_then(|m| m.as_str().parse::<i32>().ok());
            let explicit_meridiem = captures.name("meridiem").map(|m| m.as_str().to_lowercase());

            let mut components = context.create_components();
            let mut target_date = ref_date;

            let mut inferred_time_part = None;

            match keyword.as_str() {
                "hoje" => {
                    // target_date is today
                }
                "amanhã" | "amanha" => {
                    target_date = ref_date + Duration::days(1);
                }
                "ontem" => {
                    target_date = ref_date - Duration::days(1);
                }
                "manhã" | "manha" | "tarde" | "noite" => {
                    // "esta manhã", "esta tarde", "esta noite"
                    if modifier.is_some()
                        || matched_text.contains("esta")
                        || matched_text.contains("este")
                    {
                        inferred_time_part = Some(keyword.clone());
                    } else {
                        // If regex matched without modifier, it might be valid in some contexts, but
                        // usually requires modifier. However, since regex allows optional modifier,
                        // we must be careful.
                        // If time_of_day is present (unlikely here as keyword IS time_of_day),
                        // or explicit_hour is present, it might be valid "manhã às 8"? No, "de manhã às 8".
                        // "tarde às 5"? No.
                        // Let's skip if standalone without modifier/context.
                        // Unless... "noite" -> tonight?
                        start = match_end;
                        continue;
                    }
                }
                _ => {}
            }

            // Handle secondary time part (e.g. "amanhã ao meio-dia")
            if let Some(tod) = time_of_day {
                inferred_time_part = Some(tod);
            }

            // Adjust date components
            components.assign(Component::Year, target_date.year());
            components.assign(Component::Month, target_date.month() as i32);
            components.assign(Component::Day, target_date.day() as i32);

            // Apply inferred time (e.g. "night" -> 22:00)
            if let Some(tp) = &inferred_time_part {
                Self::assign_time_part(&mut components, tp);
            }

            // Apply explicit time override
            if let Some(hour) = explicit_hour {
                let mut adjusted_hour = hour;
                let mut meridiem = None;

                if let Some(m) = explicit_meridiem {
                    if m == "pm" {
                        meridiem = Some(Meridiem::PM);
                        if adjusted_hour < 12 {
                            adjusted_hour += 12;
                        }
                    } else if m == "am" {
                        meridiem = Some(Meridiem::AM);
                        if adjusted_hour == 12 {
                            adjusted_hour = 0;
                        }
                    }
                } else if let Some(tp) = &inferred_time_part {
                    // Use time part to infer meridiem/adjustment
                    // e.g. "esta noite às 8" -> 20:00
                    match tp.as_str() {
                        "noite" | "tarde" => {
                            if adjusted_hour < 12 {
                                adjusted_hour += 12;
                                meridiem = Some(Meridiem::PM);
                            }
                        }
                        "manhã" | "manha" => {
                            meridiem = Some(Meridiem::AM);
                            if adjusted_hour == 12 {
                                adjusted_hour = 0;
                            }
                        }
                        _ => {}
                    }
                }

                components.assign(Component::Hour, adjusted_hour);
                components.assign(Component::Minute, explicit_minute.unwrap_or(0));
                if let Some(m) = meridiem {
                    components.assign(Component::Meridiem, m as i32);
                }
            }

            results.push(context.create_result(match_start, match_end, components, None));

            start = match_end;
        }

        Ok(results)
    }
}

impl Default for PTCasualDateParser {
    fn default() -> Self {
        Self::new()
    }
}
