//! Japanese casual time parser
//!
//! Handles expressions like:
//! - "正午", "真夜中", "昼"
//! - "午前", "午後", "夕方", "夜"
//! - Phrases with simple modifiers such as "来週の午後"

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::ja::{get_casual_time, get_relative_modifier};
use crate::dictionaries::{CasualTimeType, RelativeModifier};
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use crate::scanner::TokenType;
use crate::types::Meridiem;
use chrono::{Datelike, Duration, Timelike};
use fancy_regex::Regex;
use std::sync::LazyLock;

static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?:(?P<modifier>今|この|今週|次|来|来週|前|先|先週)(?:\s*の)?)?(?P<time>正午|昼|真夜中|夜中|朝|午前|午後|夕方|夜)",
    )
    .unwrap()
});

pub struct JACasualTimeParser;

impl JACasualTimeParser {
    pub fn new() -> Self {
        Self
    }

    fn is_digit_like(ch: char) -> bool {
        ch.is_ascii_digit()
            || ('０'..='９').contains(&ch)
            || matches!(
                ch,
                '〇' | '一' | '二' | '三' | '四' | '五' | '六' | '七' | '八' | '九' | '十'
            )
    }

    fn has_trailing_number(text: &str, idx: usize) -> bool {
        if idx >= text.len() {
            return false;
        }
        let chars = text[idx..].chars();
        for ch in chars {
            if ch.is_whitespace() {
                continue;
            }
            return Self::is_digit_like(ch);
        }
        false
    }
}

impl Parser for JACasualTimeParser {
    fn name(&self) -> &'static str {
        "JACasualTimeParser"
    }

    fn should_apply(&self, context: &ParsingContext) -> bool {
        context.has_token_type(TokenType::CasualTime)
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();
        let ref_date = context.reference.instant;

        let mut start = 0;
        while start < context.text.len() {
            let search_text = &context.text[start..];
            let mat = match PATTERN.find(search_text) {
                Ok(Some(m)) => m,
                _ => break,
            };

            let match_start = start + mat.start();
            let match_end = start + mat.end();

            // Skip if immediately followed by a numeric time (e.g., "午前8時")
            if Self::has_trailing_number(context.text, match_end) {
                start = match_end;
                continue;
            }

            let Some(caps) = PATTERN.captures(mat.as_str()).ok().flatten() else {
                start = match_end;
                continue;
            };

            let time_word = match caps.name("time") {
                Some(m) => m.as_str(),
                None => {
                    start = match_end;
                    continue;
                }
            };

            let Some(time_type) = get_casual_time(time_word) else {
                start = match_end;
                continue;
            };

            let modifier = caps
                .name("modifier")
                .and_then(|m| get_relative_modifier(m.as_str()));

            let mut target_date = ref_date;
            match modifier {
                Some(RelativeModifier::Last) => {
                    if !(ref_date.hour() <= 6 && matches!(time_type, CasualTimeType::Night)) {
                        target_date = ref_date - Duration::days(1);
                    }
                }
                Some(RelativeModifier::Next) => {
                    target_date = ref_date + Duration::days(1);
                }
                _ => {}
            }

            let mut components = context.create_components();
            components.assign(Component::Year, target_date.year());
            components.assign(Component::Month, target_date.month() as i32);
            components.assign(Component::Day, target_date.day() as i32);

            match time_type {
                CasualTimeType::Noon => {
                    components.assign(Component::Hour, 12);
                    components.assign(Component::Minute, 0);
                    components.assign(Component::Second, 0);
                    components.assign(Component::Meridiem, Meridiem::PM as i32);
                }
                CasualTimeType::Midnight => {
                    if modifier.is_none() && ref_date.hour() >= 6 {
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
                    components.imply(Component::Hour, 22);
                    components.imply(Component::Minute, 0);
                    components.assign(Component::Meridiem, Meridiem::PM as i32);
                }
            }

            let result = context.create_result(match_start, match_end, components, None);
            results.push(result);

            start = match_end;
        }

        Ok(results)
    }
}

impl Default for JACasualTimeParser {
    fn default() -> Self {
        Self::new()
    }
}
