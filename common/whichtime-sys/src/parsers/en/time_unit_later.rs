//! Time unit later parser: "2 days later", "5 minutes from now", etc.

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::en::{get_time_unit, parse_number_pattern};
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
        r"(?i)(\d+|one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|a|an|the|few|several|couple)\s*(seconds?|minutes?|hours?|days?|weeks?|months?|years?|mins?|hrs?|secs?)\s*(?:later|after|from\s*now|forward|henceforth)\b"
    ).unwrap()
});

/// Parser for English future-relative expressions such as "2 days later".
pub struct TimeUnitLaterParser;

impl Parser for TimeUnitLaterParser {
    fn name(&self) -> &'static str {
        "TimeUnitLaterParser"
    }

    fn should_apply(&self, context: &ParsingContext) -> bool {
        context.has_token_type(TokenType::Later) && context.has_token_type(TokenType::TimeUnit)
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();
        let ref_date = context.reference.instant;

        for mat in PATTERN.find_iter(context.text) {
            let matched_text = mat.as_str();
            let index = mat.start();

            let Some(caps) = PATTERN.captures(matched_text) else {
                continue;
            };

            let num_str = caps.get(1).map(|m| m.as_str()).unwrap_or("1");
            let unit_str = caps
                .get(2)
                .map(|m| m.as_str().to_lowercase())
                .unwrap_or_default();

            let num = parse_number_pattern(num_str);
            let Some(unit) = get_time_unit(&unit_str) else {
                continue;
            };

            // Create positive duration
            let mut duration = Duration::new();
            match unit {
                TimeUnit::Second => duration.second = Some(num),
                TimeUnit::Minute => duration.minute = Some(num),
                TimeUnit::Hour => duration.hour = Some(num),
                TimeUnit::Day => duration.day = Some(num),
                TimeUnit::Week => duration.week = Some(num),
                TimeUnit::Month => duration.month = Some(num),
                TimeUnit::Year => duration.year = Some(num),
                TimeUnit::Quarter => duration.quarter = Some(num),
                TimeUnit::Millisecond => duration.millisecond = Some(num),
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
                components.assign(Component::Second, target_date.second() as i32);
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
