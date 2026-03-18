//! Ukrainian weekday parser
//!
//! Handles Ukrainian weekday expressions like:
//! - "понеділок", "вівторок", etc.
//! - "у п'ятницю", "в середу" (with preposition)
//! - "в минулий четвер", "в наступний вівторок" (with modifier)
//! - "в середу вранці" (with time)

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::uk::get_weekday;
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use crate::types::Meridiem;
use chrono::{Datelike, Duration, Weekday as ChronoWeekday};
use fancy_regex::Regex;
use std::sync::LazyLock;

static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)(?<![а-яА-ЯіїєґІЇЄҐ'])(?:(?P<prep>[уву])\s+)?(?:(?P<modifier>минулий|минулу|минула|наступний|наступну|наступна|цей|цю|ця)\s+)?(?P<weekday>понеділок|понеділка|вівторок|вівторка|середа|середу|середи|четвер|четверга|п'ятниця|п'ятницю|п'ятниці|пятниця|пятницю|субота|суботу|суботи|неділя|неділю|неділі|пн|вт|ср|чт|пт|сб|нд)(?:\s+(?P<time>вранці|зранку|опівдні|вдень|ввечері|увечері|вночі))?(?![а-яА-ЯіїєґІЇЄҐ'])"
    ).unwrap()
});

/// Ukrainian weekday parser
pub struct UKWeekdayParser;

impl UKWeekdayParser {
    pub fn new() -> Self {
        Self
    }

    fn assign_time_part(components: &mut crate::components::FastComponents, time_part: &str) {
        match time_part.to_lowercase().as_str() {
            "вранці" | "зранку" => {
                components.assign(Component::Hour, 6);
                components.assign(Component::Minute, 0);
                components.assign(Component::Meridiem, Meridiem::AM as i32);
            }
            "опівдні" => {
                components.assign(Component::Hour, 12);
                components.assign(Component::Minute, 0);
                components.assign(Component::Meridiem, Meridiem::PM as i32);
            }
            "вдень" => {
                components.assign(Component::Hour, 14);
                components.assign(Component::Minute, 0);
                components.assign(Component::Meridiem, Meridiem::PM as i32);
            }
            "ввечері" | "увечері" => {
                components.assign(Component::Hour, 20);
                components.assign(Component::Minute, 0);
                components.assign(Component::Meridiem, Meridiem::PM as i32);
            }
            "вночі" => {
                components.assign(Component::Hour, 2);
                components.assign(Component::Minute, 0);
                components.assign(Component::Meridiem, Meridiem::AM as i32);
            }
            _ => {}
        }
    }
}

impl Default for UKWeekdayParser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser for UKWeekdayParser {
    fn name(&self) -> &'static str {
        "UKWeekdayParser"
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

            let modifier = captures.name("modifier").map(|m| m.as_str().to_lowercase());
            let weekday_str = captures
                .name("weekday")
                .map(|m| m.as_str().to_lowercase())
                .unwrap_or_default();
            let time_part = captures.name("time").map(|m| m.as_str());

            // Clean weekday string (remove trailing dots)
            let clean_weekday = weekday_str.trim_end_matches('.');

            let Some(weekday) = get_weekday(clean_weekday) else {
                start = match_end;
                continue;
            };

            let mut components = context.create_components();

            // Convert our weekday to chrono weekday for calculation
            let target_weekday = match weekday {
                crate::types::Weekday::Sunday => ChronoWeekday::Sun,
                crate::types::Weekday::Monday => ChronoWeekday::Mon,
                crate::types::Weekday::Tuesday => ChronoWeekday::Tue,
                crate::types::Weekday::Wednesday => ChronoWeekday::Wed,
                crate::types::Weekday::Thursday => ChronoWeekday::Thu,
                crate::types::Weekday::Friday => ChronoWeekday::Fri,
                crate::types::Weekday::Saturday => ChronoWeekday::Sat,
            };

            let current_weekday = ref_date.weekday();
            let current_day_num = current_weekday.num_days_from_sunday() as i64;
            let target_day_num = target_weekday.num_days_from_sunday() as i64;

            // Calculate days difference
            let mut days_diff = target_day_num - current_day_num;

            // Check if preposition is present
            let has_preposition = captures.name("prep").is_some();

            // Handle modifiers
            let target_date = match modifier.as_deref() {
                Some(m) if m.starts_with("наступн") => {
                    // Next occurrence (always in the future, at least 1 day ahead)
                    if days_diff <= 0 {
                        days_diff += 7;
                    }
                    ref_date + Duration::days(days_diff)
                }
                Some(m) if m.starts_with("минул") => {
                    // Last occurrence (always in the past)
                    if days_diff >= 0 {
                        days_diff -= 7;
                    }
                    ref_date + Duration::days(days_diff)
                }
                _ if has_preposition => {
                    // With preposition (у/в середу) - find nearest occurrence (closest)
                    // Compare absolute distance to past vs future occurrence
                    let past_diff = if days_diff > 0 {
                        days_diff - 7
                    } else {
                        days_diff
                    };
                    let future_diff = if days_diff <= 0 {
                        days_diff + 7
                    } else {
                        days_diff
                    };

                    // Use whichever is closer (prefer future for equal distance)
                    if past_diff.abs() < future_diff.abs() {
                        days_diff = past_diff;
                    } else {
                        days_diff = future_diff;
                    }
                    ref_date + Duration::days(days_diff)
                }
                _ => {
                    // Without preposition - find nearest past/current occurrence
                    if days_diff > 0 {
                        days_diff -= 7;
                    }
                    ref_date + Duration::days(days_diff)
                }
            };

            components.assign(Component::Year, target_date.year());
            components.assign(Component::Month, target_date.month() as i32);
            components.assign(Component::Day, target_date.day() as i32);
            components.assign(Component::Weekday, weekday as i32);

            // Apply time part if present
            if let Some(tp) = time_part {
                Self::assign_time_part(&mut components, tp);
            }

            results.push(context.create_result(match_start, match_end, components, None));
            start = match_end;
        }

        Ok(results)
    }
}
