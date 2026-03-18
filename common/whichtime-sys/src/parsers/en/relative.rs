//! Relative date parser: "this week", "next month", "last year", etc.

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::RelativeModifier;
use crate::dictionaries::en::{get_relative_modifier, get_time_unit, parse_number_pattern};
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use crate::scanner::TokenType;
use crate::types::{Duration, TimeUnit, add_duration};
use chrono::Datelike;
use regex::Regex;
use std::sync::LazyLock;

static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)\b(this|next|last|past|previous)\s*(week|month|year|quarter|hour|minute|day)\b",
    )
    .unwrap()
});

// Pattern for "next 2 weeks", "last 3 months"
static NUMBERED_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)\b(next|last|past)\s+(\d+|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve)\s*(weeks?|months?|years?|quarters?|hours?|minutes?|days?)\b"
    ).unwrap()
});

/// Parser for English relative calendar periods such as "next month".
pub struct RelativeDateParser;

impl Parser for RelativeDateParser {
    fn name(&self) -> &'static str {
        "RelativeDateParser"
    }

    fn should_apply(&self, context: &ParsingContext) -> bool {
        context.has_token_type(TokenType::RelativeModifier)
            && context.has_token_type(TokenType::TimeUnit)
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();
        let ref_date = context.reference.instant;

        // Try numbered pattern first ("next 2 weeks")
        for mat in NUMBERED_PATTERN.find_iter(context.text) {
            let matched_text = mat.as_str();
            let index = mat.start();

            let Some(caps) = NUMBERED_PATTERN.captures(matched_text) else {
                continue;
            };

            let modifier_str = caps
                .get(1)
                .map(|m| m.as_str().to_lowercase())
                .unwrap_or_default();
            let num_str = caps.get(2).map(|m| m.as_str()).unwrap_or("1");
            let unit_str = caps
                .get(3)
                .map(|m| m.as_str().to_lowercase())
                .unwrap_or_default();

            let Some(modifier) = get_relative_modifier(&modifier_str) else {
                continue;
            };
            let num = parse_number_pattern(num_str);
            let Some(unit) = get_time_unit(&unit_str) else {
                continue;
            };

            let multiplier = match modifier {
                RelativeModifier::Next => 1.0,
                RelativeModifier::Last => -1.0,
                RelativeModifier::This => 0.0,
            };

            let mut duration = Duration::new();
            let value = num * multiplier;
            match unit {
                TimeUnit::Second => duration.second = Some(value),
                TimeUnit::Minute => duration.minute = Some(value),
                TimeUnit::Hour => duration.hour = Some(value),
                TimeUnit::Day => duration.day = Some(value),
                TimeUnit::Week => duration.week = Some(value),
                TimeUnit::Month => duration.month = Some(value),
                TimeUnit::Year => duration.year = Some(value),
                TimeUnit::Quarter => duration.quarter = Some(value),
                TimeUnit::Millisecond => duration.millisecond = Some(value),
            }

            let target_date = add_duration(ref_date, &duration);

            let mut components = context.create_components();
            components.assign(Component::Year, target_date.year());
            components.assign(Component::Month, target_date.month() as i32);
            components.assign(Component::Day, target_date.day() as i32);

            if duration.has_time_component() {
                use chrono::Timelike;
                components.assign(Component::Hour, target_date.hour() as i32);
                components.assign(Component::Minute, target_date.minute() as i32);
            }

            results.push(context.create_result(
                index,
                index + matched_text.len(),
                components,
                None,
            ));
        }

        // Try simple pattern ("this week", "next month")
        for mat in PATTERN.find_iter(context.text) {
            let matched_text = mat.as_str();
            let index = mat.start();

            // Skip if already matched
            if results
                .iter()
                .any(|r| r.index <= index && r.end_index > index)
            {
                continue;
            }

            let Some(caps) = PATTERN.captures(matched_text) else {
                continue;
            };

            let modifier_str = caps
                .get(1)
                .map(|m| m.as_str().to_lowercase())
                .unwrap_or_default();
            let unit_str = caps
                .get(2)
                .map(|m| m.as_str().to_lowercase())
                .unwrap_or_default();

            let Some(modifier) = get_relative_modifier(&modifier_str) else {
                continue;
            };
            let Some(unit) = get_time_unit(&unit_str) else {
                continue;
            };

            let mut duration = Duration::new();
            let value = match modifier {
                RelativeModifier::Next => 1.0,
                RelativeModifier::Last => -1.0,
                RelativeModifier::This => 0.0,
            };

            match unit {
                TimeUnit::Hour => duration.hour = Some(value),
                TimeUnit::Day => duration.day = Some(value),
                TimeUnit::Week => duration.week = Some(value),
                TimeUnit::Month => duration.month = Some(value),
                TimeUnit::Year => duration.year = Some(value),
                TimeUnit::Quarter => duration.quarter = Some(value),
                _ => continue,
            }

            let target_date = add_duration(ref_date, &duration);

            let mut components = context.create_components();

            // For "this week/month/year", set to start of period
            match unit {
                TimeUnit::Week => {
                    components.assign(Component::Year, target_date.year());
                    components.assign(Component::Month, target_date.month() as i32);
                    components.assign(Component::Day, target_date.day() as i32);
                }
                TimeUnit::Month => {
                    components.assign(Component::Year, target_date.year());
                    components.assign(Component::Month, target_date.month() as i32);
                    components.assign(Component::Day, 1);
                }
                TimeUnit::Year => {
                    components.assign(Component::Year, target_date.year());
                    components.assign(Component::Month, 1);
                    components.assign(Component::Day, 1);
                }
                _ => {
                    components.assign(Component::Year, target_date.year());
                    components.assign(Component::Month, target_date.month() as i32);
                    components.assign(Component::Day, target_date.day() as i32);
                    if duration.has_time_component() {
                        use chrono::Timelike;
                        components.assign(Component::Hour, target_date.hour() as i32);
                        components.assign(Component::Minute, target_date.minute() as i32);
                    }
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
