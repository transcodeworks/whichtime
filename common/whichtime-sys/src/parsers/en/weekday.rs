//! Weekday parser: Monday, next Friday, last Tuesday, etc.

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::RelativeModifier;
use crate::dictionaries::en::{get_relative_modifier, get_weekday};
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use crate::scanner::TokenType;
use chrono::{Datelike, Duration};
use regex::Regex;
use std::sync::LazyLock;

static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)(?:^|\W)(?:(this|next|last|past|previous)\s+)?(sun(?:day)?|mon(?:day)?|tue(?:s(?:day)?)?|wed(?:nesday)?|thu(?:rs(?:day)?)?|fri(?:day)?|sat(?:urday)?)(?:\W|$)"
    ).unwrap()
});

/// Parser for English weekday expressions such as "next Friday".
pub struct WeekdayParser;

impl Parser for WeekdayParser {
    fn name(&self) -> &'static str {
        "WeekdayParser"
    }

    fn should_apply(&self, context: &ParsingContext) -> bool {
        context.has_token_type(TokenType::Weekday)
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();
        let ref_date = context.reference.instant;
        let ref_weekday = ref_date.weekday().num_days_from_sunday();

        for mat in PATTERN.find_iter(context.text) {
            let matched_text = mat.as_str();
            let index = mat.start();

            let Some(caps) = PATTERN.captures(matched_text) else {
                continue;
            };

            let modifier_str = caps.get(1).map(|m| m.as_str().to_lowercase());
            let weekday_str = caps
                .get(2)
                .map(|m| m.as_str().to_lowercase())
                .unwrap_or_default();

            let Some(weekday) = get_weekday(&weekday_str) else {
                continue;
            };

            let modifier = modifier_str.as_deref().and_then(get_relative_modifier);

            // Calculate days offset
            let days_offset = match modifier {
                Some(RelativeModifier::Next) => {
                    let diff = (weekday as i64) - (ref_weekday as i64);
                    if diff <= 0 { diff + 7 } else { diff }
                }
                Some(RelativeModifier::Last) => {
                    let diff = (weekday as i64) - (ref_weekday as i64);
                    if diff >= 0 { diff - 7 } else { diff }
                }
                Some(RelativeModifier::This) | None => {
                    let diff = (weekday as i64) - (ref_weekday as i64);
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
            components.assign(Component::Weekday, weekday as i32);

            // Find actual text bounds
            let actual_start = matched_text
                .find(|c: char| c.is_alphanumeric())
                .unwrap_or(0);
            let actual_end = matched_text
                .rfind(|c: char| c.is_alphanumeric())
                .map(|i| i + matched_text[i..].chars().next().map_or(1, char::len_utf8))
                .unwrap_or(matched_text.len());

            results.push(context.create_result(
                index + actual_start,
                index + actual_end,
                components,
                None,
            ));
        }

        Ok(results)
    }
}
