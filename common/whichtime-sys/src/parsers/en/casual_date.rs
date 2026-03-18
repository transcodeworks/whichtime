//! Casual date parser: now, today, tomorrow, yesterday, etc.

use std::sync::LazyLock;

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::CasualDateType;
use crate::dictionaries::en::get_casual_date;
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use crate::scanner::TokenType;
use chrono::{Datelike, Duration, Timelike};
use regex::Regex;

static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(now|today|tonight|tomorrow|overmorrow|tmr|tmrw|yesterday)\b").unwrap()
});

/// Parser for English casual date expressions such as "tomorrow".
pub struct CasualDateParser;

impl Parser for CasualDateParser {
    fn name(&self) -> &'static str {
        "CasualDateParser"
    }

    fn should_apply(&self, context: &ParsingContext) -> bool {
        context.has_token_type(TokenType::CasualDate)
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();

        // for mat in PATTERN.find_iter(context.text) {
        for mat in PATTERN.find_iter(context.text) {
            let matched_text = mat.as_str();
            let lower = matched_text.to_lowercase();
            let index = mat.start();

            let mut components = context.create_components();
            let ref_date = context.reference.instant;

            let Some(casual_type) = get_casual_date(&lower) else {
                continue;
            };

            match casual_type {
                CasualDateType::Now => {
                    components.assign(Component::Year, ref_date.year());
                    components.assign(Component::Month, ref_date.month() as i32);
                    components.assign(Component::Day, ref_date.day() as i32);
                    components.assign(Component::Hour, ref_date.hour() as i32);
                    components.assign(Component::Minute, ref_date.minute() as i32);
                    components.assign(Component::Second, ref_date.second() as i32);
                }
                CasualDateType::Today => {
                    components.assign(Component::Year, ref_date.year());
                    components.assign(Component::Month, ref_date.month() as i32);
                    components.assign(Component::Day, ref_date.day() as i32);
                }
                CasualDateType::Tonight => {
                    components.assign(Component::Year, ref_date.year());
                    components.assign(Component::Month, ref_date.month() as i32);
                    components.assign(Component::Day, ref_date.day() as i32);
                    components.imply(Component::Hour, 22);
                }
                CasualDateType::Tomorrow => {
                    let tomorrow = ref_date + Duration::days(1);
                    components.assign(Component::Year, tomorrow.year());
                    components.assign(Component::Month, tomorrow.month() as i32);
                    components.assign(Component::Day, tomorrow.day() as i32);
                }
                CasualDateType::Yesterday => {
                    let yesterday = ref_date - Duration::days(1);
                    components.assign(Component::Year, yesterday.year());
                    components.assign(Component::Month, yesterday.month() as i32);
                    components.assign(Component::Day, yesterday.day() as i32);
                }
                CasualDateType::Overmorrow => {
                    let day_after = ref_date + Duration::days(2);
                    components.assign(Component::Year, day_after.year());
                    components.assign(Component::Month, day_after.month() as i32);
                    components.assign(Component::Day, day_after.day() as i32);
                }
                CasualDateType::DayBeforeYesterday => {
                    let day_before = ref_date - Duration::days(2);
                    components.assign(Component::Year, day_before.year());
                    components.assign(Component::Month, day_before.month() as i32);
                    components.assign(Component::Day, day_before.day() as i32);
                }
                CasualDateType::ThisMorning => {
                    components.assign(Component::Year, ref_date.year());
                    components.assign(Component::Month, ref_date.month() as i32);
                    components.assign(Component::Day, ref_date.day() as i32);
                    components.imply(Component::Hour, 6);
                }
                CasualDateType::ThisAfternoon => {
                    components.assign(Component::Year, ref_date.year());
                    components.assign(Component::Month, ref_date.month() as i32);
                    components.assign(Component::Day, ref_date.day() as i32);
                    components.imply(Component::Hour, 15);
                }
                CasualDateType::ThisEvening => {
                    components.assign(Component::Year, ref_date.year());
                    components.assign(Component::Month, ref_date.month() as i32);
                    components.assign(Component::Day, ref_date.day() as i32);
                    components.imply(Component::Hour, 20);
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
