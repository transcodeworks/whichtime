//! Swedish casual date parser
//!
//! Handles Swedish casual date expressions like:
//! - "idag", "imorgon", "igår"
//! - "idag på kvällen", "imorgon vid middag"

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
        r"(?i)(?<![a-zA-ZåäöÅÄÖ])(nu|idag|i\s*dag|ikväll|i\s*kväll|ikvall|i\s*kvall|imorgon|i\s*morgon|igår|i\s*går|igar|i\s*gar|övermorgon|i\s*övermorgon|overmorgon|i\s*overmorgon|förrgår|i\s*förrgår|forrgar|i\s*forrgar)(?:\s+(på|vid)\s+(morgonen|förmiddagen|formiddagen|middagen|eftermiddagen|kvällen|kvallen|natten|midnatt|middag))?(?![a-zA-ZåäöÅÄÖ])"
    ).unwrap()
});

const DATE_GROUP: usize = 1;
const TIME_GROUP: usize = 3;

/// Swedish casual date parser
pub struct SVCasualDateParser;

impl SVCasualDateParser {
    pub fn new() -> Self {
        Self
    }

    fn assign_time_part(components: &mut crate::components::FastComponents, time_part: &str) {
        match time_part.to_lowercase().as_str() {
            "morgonen" => {
                components.assign(Component::Hour, 6);
                components.assign(Component::Minute, 0);
                components.assign(Component::Meridiem, Meridiem::AM as i32);
            }
            "förmiddagen" | "formiddagen" => {
                components.assign(Component::Hour, 9);
                components.assign(Component::Minute, 0);
                components.assign(Component::Meridiem, Meridiem::AM as i32);
            }
            "middagen" | "middag" => {
                components.assign(Component::Hour, 12);
                components.assign(Component::Minute, 0);
                components.assign(Component::Meridiem, Meridiem::PM as i32);
            }
            "eftermiddagen" => {
                components.assign(Component::Hour, 15);
                components.assign(Component::Minute, 0);
                components.assign(Component::Meridiem, Meridiem::PM as i32);
            }
            "kvällen" | "kvallen" => {
                components.assign(Component::Hour, 20);
                components.assign(Component::Minute, 0);
                components.assign(Component::Meridiem, Meridiem::PM as i32);
            }
            "natten" => {
                components.assign(Component::Hour, 2);
                components.assign(Component::Minute, 0);
                components.assign(Component::Meridiem, Meridiem::AM as i32);
            }
            "midnatt" => {
                components.assign(Component::Hour, 0);
                components.assign(Component::Minute, 0);
                components.assign(Component::Second, 0);
            }
            _ => {}
        }
    }
}

impl Default for SVCasualDateParser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser for SVCasualDateParser {
    fn name(&self) -> &'static str {
        "SVCasualDateParser"
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

            let date_keyword = captures
                .get(DATE_GROUP)
                .map(|m| m.as_str().to_lowercase().replace(" ", ""))
                .unwrap_or_default();

            let time_part = captures.get(TIME_GROUP).map(|m| m.as_str());

            let mut components = context.create_components();
            let mut target_date = ref_date;

            match date_keyword.as_str() {
                "nu" => {
                    components.assign(Component::Hour, ref_date.hour() as i32);
                    components.assign(Component::Minute, ref_date.minute() as i32);
                    components.assign(Component::Second, ref_date.second() as i32);
                }
                "idag" => {}
                "ikväll" | "ikvall" => {
                    components.imply(Component::Hour, 20);
                    components.imply(Component::Minute, 0);
                }
                "imorgon" => {
                    target_date = ref_date + Duration::days(1);
                }
                "igår" | "igar" => {
                    target_date = ref_date - Duration::days(1);
                }
                "övermorgon" | "overmorgon" | "iövermorgon" | "iovermorgon" => {
                    target_date = ref_date + Duration::days(2);
                }
                "förrgår" | "forrgar" | "iförrgår" | "iforrgar" => {
                    target_date = ref_date - Duration::days(2);
                }
                _ => {}
            }

            components.assign(Component::Year, target_date.year());
            components.assign(Component::Month, target_date.month() as i32);
            components.assign(Component::Day, target_date.day() as i32);

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
