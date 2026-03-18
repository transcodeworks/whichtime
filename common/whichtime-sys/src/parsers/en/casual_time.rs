//! Casual time parser: noon, midnight, morning, afternoon, evening, night
//! Also handles compositional patterns: "last night", "this morning", "next evening"

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::en::{get_casual_time, get_relative_modifier};
use crate::dictionaries::{CasualTimeType, RelativeModifier};
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use crate::scanner::TokenType;
use crate::types::Meridiem;
use chrono::{Datelike, Duration, Timelike};
use regex::Regex;
use std::sync::LazyLock;

// Pattern for standalone casual time or modifier + time_of_day
static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(?:(this|last|next|past|previous)\s+)?(noon|midday|midnight|morning|afternoon|evening|night)\b").unwrap()
});

/// Parser for English casual time-of-day expressions.
pub struct CasualTimeParser;

impl Parser for CasualTimeParser {
    fn name(&self) -> &'static str {
        "CasualTimeParser"
    }

    fn should_apply(&self, context: &ParsingContext) -> bool {
        context.has_token_type(TokenType::CasualTime)
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();

        for mat in PATTERN.find_iter(context.text) {
            let matched_text = mat.as_str();
            let index = mat.start();

            let Some(caps) = PATTERN.captures(matched_text) else {
                continue;
            };

            let modifier_str = caps.get(1).map(|m| m.as_str().to_lowercase());
            let time_word = caps
                .get(2)
                .map(|m| m.as_str().to_lowercase())
                .unwrap_or_default();

            let Some(time_type) = get_casual_time(&time_word) else {
                continue;
            };

            let modifier = modifier_str.as_deref().and_then(get_relative_modifier);

            let mut components = context.create_components();
            let ref_date = context.reference.instant;

            // Calculate target date based on modifier
            let target_date = match modifier {
                Some(RelativeModifier::Last) => {
                    // "last night" - if currently before 6 AM, same night; otherwise yesterday
                    if ref_date.hour() <= 6 && matches!(time_type, CasualTimeType::Night) {
                        ref_date
                    } else {
                        ref_date - Duration::days(1)
                    }
                }
                Some(RelativeModifier::Next) => ref_date + Duration::days(1),
                Some(RelativeModifier::This) | None => ref_date,
            };

            // Set date components
            components.assign(Component::Year, target_date.year());
            components.assign(Component::Month, target_date.month() as i32);
            components.assign(Component::Day, target_date.day() as i32);

            // Set time components based on time_type
            match time_type {
                CasualTimeType::Noon => {
                    components.assign(Component::Hour, 12);
                    components.assign(Component::Minute, 0);
                    components.assign(Component::Second, 0);
                    components.assign(Component::Meridiem, Meridiem::PM as i32);
                }
                CasualTimeType::Midnight => {
                    // For "this midnight" or standalone "midnight", use start of next day
                    // For "last midnight", use start of current day
                    if matches!(modifier, Some(RelativeModifier::Last)) {
                        // Already adjusted date to yesterday, so midnight is at start of that day
                        components.assign(Component::Day, target_date.day() as i32);
                    } else if modifier.is_none() {
                        // Standalone "midnight" typically means upcoming midnight
                        let next_day = ref_date + Duration::days(1);
                        components.assign(Component::Year, next_day.year());
                        components.assign(Component::Month, next_day.month() as i32);
                        components.assign(Component::Day, next_day.day() as i32);
                    }
                    components.assign(Component::Hour, 0);
                    components.assign(Component::Minute, 0);
                    components.assign(Component::Second, 0);
                }
                CasualTimeType::Morning => {
                    components.imply(Component::Hour, 6);
                    components.imply(Component::Minute, 0);
                    components.assign(Component::Meridiem, Meridiem::AM as i32);
                }
                CasualTimeType::Afternoon => {
                    components.imply(Component::Hour, 15);
                    components.imply(Component::Minute, 0);
                    components.assign(Component::Meridiem, Meridiem::PM as i32);
                }
                CasualTimeType::Evening => {
                    components.imply(Component::Hour, 20);
                    components.imply(Component::Minute, 0);
                    components.assign(Component::Meridiem, Meridiem::PM as i32);
                }
                CasualTimeType::Night => {
                    components.imply(Component::Hour, 0);
                    components.imply(Component::Minute, 0);
                    components.assign(Component::Meridiem, Meridiem::AM as i32);
                }
            }

            results.push(context.create_result(
                index,
                index + matched_text.len(),
                components,
                None,
            ));
        }

        Ok(results)
    }
}
