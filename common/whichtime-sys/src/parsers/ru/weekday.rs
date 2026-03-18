//! Russian weekday parser
//!
//! Handles Russian weekday expressions like:
//! - "понедельник", "в пятницу"
//! - "следующая среда", "прошлый вторник"
//! - "через 2 пятницы" (future)

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::RelativeModifier;
use crate::dictionaries::ru::{get_relative_modifier, get_weekday};
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use chrono::{Datelike, Duration};
use fancy_regex::Regex;
use std::sync::LazyLock;

static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?ix)
        (?<![a-zA-Zа-яА-Я])
        (?:(?:в|во|на|к)\s+)?
        (?:
            (?P<prefix>этот|эта|это|эту|следующий|следующая|следующее|следующую|будущий|будущая|будущее|прошлый|прошлая|прошлое|прошлую|предыдущий|предыдущая|предыдущее|последний|последняя)\s+
        )?
        (?P<weekday>понедельник|понедельника|пн|вторник|вторника|вт|среда|среды|среду|ср|четверг|четверга|чт|пятница|пятницу|пятницы|пт|суббота|субботу|субботы|сб|воскресенье|воскресенья|вс|вск)
        (?:\.|,)?
        (?:
            \s+
            (?P<suffix>прошлый|прошлая|прошлое|прошлую|следующий|следующая|следующее|следующую)
        )?
        (?=\W|$)"
    ).unwrap()
});

/// Russian weekday parser
pub struct RUWeekdayParser;

impl RUWeekdayParser {
    pub fn new() -> Self {
        Self
    }
}

impl Default for RUWeekdayParser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser for RUWeekdayParser {
    fn name(&self) -> &'static str {
        "RUWeekdayParser"
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
            let suffix_str = captures.name("suffix").map(|m| m.as_str().to_lowercase());

            let clean_wd = weekday_str.trim_end_matches('.').trim_end_matches(',');
            let Some(weekday) = get_weekday(clean_wd) else {
                start = match_end;
                continue;
            };

            let target_weekday = weekday as i64;

            // Determine modifier from prefix or suffix
            let modifier = prefix_str
                .as_ref()
                .and_then(|s| get_relative_modifier(s))
                .or_else(|| suffix_str.as_ref().and_then(|s| get_relative_modifier(s)));

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
