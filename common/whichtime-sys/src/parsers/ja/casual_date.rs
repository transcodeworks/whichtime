//! Japanese casual date parser
//!
//! Handles Japanese casual date expressions like:
//! - "今日", "きょう", "本日" (today)
//! - "明日", "あした" (tomorrow)
//! - "昨日", "きのう" (yesterday)
//! - "今朝" (this morning)
//! - "今晩", "今夜" (tonight)
//! - Combined: "明日の正午", "来週の金曜日の午後3時"

use crate::components::Component;
use crate::context::ParsingContext;
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use crate::types::Meridiem;
use chrono::{Datelike, Duration};
use fancy_regex::Regex;
use std::sync::LazyLock;

static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(今日|きょう|本日|ほんじつ|明日|あした|あす|昨日|きのう|さくじつ|明後日|あさって|一昨日|おととい|今朝|けさ|今夜|こんや|今晩|こんばん|今夕|こんゆう)(?:の(正午|昼|朝|午前|午後|夕方|夜|真夜中))?"
    ).unwrap()
});

const DATE_GROUP: usize = 1;
const TIME_PERIOD_GROUP: usize = 2;

/// Japanese casual date parser
pub struct JACasualDateParser;

impl JACasualDateParser {
    pub fn new() -> Self {
        Self
    }
}

impl Parser for JACasualDateParser {
    fn name(&self) -> &'static str {
        "JACasualDateParser"
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
                .map(|m| m.as_str())
                .unwrap_or_default();

            let time_period = captures.get(TIME_PERIOD_GROUP).map(|m| m.as_str());

            let mut components = context.create_components();
            let target_date;

            // Process date keyword
            match date_keyword {
                "今日" | "きょう" | "本日" | "ほんじつ" => {
                    components.assign(Component::Year, ref_date.year());
                    components.assign(Component::Month, ref_date.month() as i32);
                    components.assign(Component::Day, ref_date.day() as i32);
                }
                "明日" | "あした" | "あす" => {
                    target_date = ref_date + Duration::days(1);
                    components.assign(Component::Year, target_date.year());
                    components.assign(Component::Month, target_date.month() as i32);
                    components.assign(Component::Day, target_date.day() as i32);
                }
                "昨日" | "きのう" | "さくじつ" => {
                    target_date = ref_date - Duration::days(1);
                    components.assign(Component::Year, target_date.year());
                    components.assign(Component::Month, target_date.month() as i32);
                    components.assign(Component::Day, target_date.day() as i32);
                }
                "明後日" | "あさって" => {
                    target_date = ref_date + Duration::days(2);
                    components.assign(Component::Year, target_date.year());
                    components.assign(Component::Month, target_date.month() as i32);
                    components.assign(Component::Day, target_date.day() as i32);
                }
                "一昨日" | "おととい" => {
                    target_date = ref_date - Duration::days(2);
                    components.assign(Component::Year, target_date.year());
                    components.assign(Component::Month, target_date.month() as i32);
                    components.assign(Component::Day, target_date.day() as i32);
                }
                "今朝" | "けさ" => {
                    components.assign(Component::Year, ref_date.year());
                    components.assign(Component::Month, ref_date.month() as i32);
                    components.assign(Component::Day, ref_date.day() as i32);
                    components.imply(Component::Hour, 6);
                    components.assign(Component::Meridiem, Meridiem::AM as i32);
                }
                "今夜" | "こんや" | "今晩" | "こんばん" | "今夕" | "こんゆう" => {
                    components.assign(Component::Year, ref_date.year());
                    components.assign(Component::Month, ref_date.month() as i32);
                    components.assign(Component::Day, ref_date.day() as i32);
                    components.imply(Component::Hour, 22);
                    components.assign(Component::Meridiem, Meridiem::PM as i32);
                }
                _ => {
                    start = match_end;
                    continue;
                }
            }

            // Apply time period modifier if present
            if let Some(period) = time_period {
                match period {
                    "正午" | "昼" => {
                        components.assign(Component::Hour, 12);
                        components.assign(Component::Minute, 0);
                        components.assign(Component::Meridiem, Meridiem::PM as i32);
                    }
                    "朝" | "午前" => {
                        components.imply(Component::Hour, 8);
                        components.assign(Component::Meridiem, Meridiem::AM as i32);
                    }
                    "午後" => {
                        components.imply(Component::Hour, 14);
                        components.assign(Component::Meridiem, Meridiem::PM as i32);
                    }
                    "夕方" => {
                        components.imply(Component::Hour, 17);
                        components.assign(Component::Meridiem, Meridiem::PM as i32);
                    }
                    "夜" => {
                        components.imply(Component::Hour, 20);
                        components.assign(Component::Meridiem, Meridiem::PM as i32);
                    }
                    "真夜中" => {
                        components.assign(Component::Hour, 0);
                        components.assign(Component::Minute, 0);
                    }
                    _ => {}
                }
            }

            results.push(context.create_result(match_start, match_end, components, None));

            start = match_end;
        }

        Ok(results)
    }
}

impl Default for JACasualDateParser {
    fn default() -> Self {
        Self::new()
    }
}
