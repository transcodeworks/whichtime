//! Ukrainian casual date parser
//!
//! Handles Ukrainian casual date expressions like:
//! - "сьогодні", "завтра", "вчора"
//! - "завтра вранці", "вчора ввечері"
//! - "завтра опівдні"

use crate::components::Component;
use crate::context::ParsingContext;
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use crate::types::Meridiem;
use chrono::{Datelike, Duration, Timelike};
use fancy_regex::Regex;
use std::sync::LazyLock;

static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)(?<![а-яА-ЯіїєґІЇЄҐ'])(?:(?P<date>зараз|сьогодні|сьогодни|завтра|післязавтра|вчора|позавчора)(?:\s+(?P<time>вранці|зранку|опівдні|вдень|ввечері|увечері|вночі|опівночі))?|(?P<time_only>вранці|зранку|опівдні|вдень|ввечері|увечері|вночі|опівночі))(?![а-яА-ЯіїєґІЇЄҐ'])"
    ).unwrap()
});

/// Ukrainian casual date parser
pub struct UKCasualDateParser;

impl UKCasualDateParser {
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
            "опівночі" => {
                components.assign(Component::Hour, 0);
                components.assign(Component::Minute, 0);
                components.assign(Component::Second, 0);
            }
            _ => {}
        }
    }
}

impl Default for UKCasualDateParser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser for UKCasualDateParser {
    fn name(&self) -> &'static str {
        "UKCasualDateParser"
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

            let date_keyword = captures.name("date").map(|m| m.as_str().to_lowercase());
            let time_part = captures.name("time").map(|m| m.as_str());
            let time_only = captures.name("time_only").map(|m| m.as_str());

            let mut components = context.create_components();
            let mut target_date = ref_date;

            if let Some(ref kw) = date_keyword {
                match kw.as_str() {
                    "зараз" => {
                        components.assign(Component::Hour, ref_date.hour() as i32);
                        components.assign(Component::Minute, ref_date.minute() as i32);
                        components.assign(Component::Second, ref_date.second() as i32);
                    }
                    "сьогодні" | "сьогодни" => {}
                    "завтра" => {
                        target_date = ref_date + Duration::days(1);
                    }
                    "післязавтра" => {
                        target_date = ref_date + Duration::days(2);
                    }
                    "вчора" => {
                        target_date = ref_date - Duration::days(1);
                    }
                    "позавчора" => {
                        target_date = ref_date - Duration::days(2);
                    }
                    _ => {}
                }
            }

            components.assign(Component::Year, target_date.year());
            components.assign(Component::Month, target_date.month() as i32);
            components.assign(Component::Day, target_date.day() as i32);

            // Apply time part if present
            if let Some(tp) = time_part.or(time_only) {
                Self::assign_time_part(&mut components, tp);
            }

            results.push(context.create_result(match_start, match_end, components, None));
            start = match_end;
        }

        Ok(results)
    }
}
