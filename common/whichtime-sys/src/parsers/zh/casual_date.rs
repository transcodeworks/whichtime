//! Chinese casual date parser
//!
//! Handles Chinese casual date expressions like:
//! - "今天", "今日" (today)
//! - "明天", "明日" (tomorrow)
//! - "昨天", "昨日" (yesterday)
//! - "今晚", "今夜" (tonight)
//! - "而家" (now - Cantonese)
//! - "聽日" (tomorrow - Cantonese)
//! - Combined: "今天下午5点", "明天早上8点"

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::zh::{NUMBER_MAP, parse_number_pattern};
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use crate::types::Meridiem;
use chrono::{Datelike, Duration, Timelike};
use fancy_regex::Regex;
use std::sync::LazyLock;

// Pattern for casual date with optional time period and optional explicit time
static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?P<date>今天|今日|今晚|今夜|明天|明日|昨天|昨日|后天|後天|前天|现在|現在|而家|聽日|尋日|琴日)(?P<time_part>早上|早晨|上午|中午|正午|下午|傍晚|晚上|晚间|晚間|夜里|夜裡|夜晚|凌晨|午夜|半夜)?(?:(?P<hour>[0-9一二三四五六七八九十零〇两兩]+)(?:点|點))?"
    ).unwrap()
});

// Pattern for standalone time period (implies today)
static TIME_ONLY_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?P<time_only>早上|早晨|上午|中午|正午|下午|傍晚|晚上|晚间|晚間|夜里|夜裡|夜晚|凌晨|午夜|半夜)"
    ).unwrap()
});

/// Chinese casual date parser
pub struct ZHCasualDateParser;

impl ZHCasualDateParser {
    pub fn new() -> Self {
        Self
    }

    fn get_time_period_hour(period: &str) -> Option<(i32, Option<Meridiem>)> {
        match period {
            "早上" | "早晨" | "上午" => Some((6, Some(Meridiem::AM))),
            "中午" | "正午" => Some((12, Some(Meridiem::PM))),
            "下午" => Some((15, Some(Meridiem::PM))),
            "傍晚" => Some((18, Some(Meridiem::PM))),
            "晚上" | "晚间" | "晚間" => Some((22, Some(Meridiem::PM))),
            "夜里" | "夜裡" | "夜晚" => Some((22, Some(Meridiem::PM))),
            "凌晨" | "午夜" | "半夜" => Some((0, Some(Meridiem::AM))),
            _ => None,
        }
    }

    fn parse_hour(s: &str) -> i32 {
        // First check if it's a single character Chinese number
        if let Some(&val) = NUMBER_MAP.get(s) {
            return val as i32;
        }
        // Then try full conversion
        parse_number_pattern(s) as i32
    }
}

impl Parser for ZHCasualDateParser {
    fn name(&self) -> &'static str {
        "ZHCasualDateParser"
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

            // First try the main pattern with date keywords
            if let Ok(Some(caps)) = PATTERN.captures(search_text) {
                let full_match = caps.get(0).unwrap();
                let match_start = start + full_match.start();
                let match_end = start + full_match.end();

                let date_keyword = caps.name("date").map(|m| m.as_str()).unwrap_or_default();

                let time_part = caps.name("time_part").map(|m| m.as_str());
                let hour_str = caps.name("hour").map(|m| m.as_str());

                let mut components = context.create_components();
                let target_date;

                // Process date keyword
                match date_keyword {
                    "今天" | "今日" => {
                        components.assign(Component::Year, ref_date.year());
                        components.assign(Component::Month, ref_date.month() as i32);
                        components.assign(Component::Day, ref_date.day() as i32);
                    }
                    "明天" | "明日" | "聽日" => {
                        target_date = ref_date + Duration::days(1);
                        components.assign(Component::Year, target_date.year());
                        components.assign(Component::Month, target_date.month() as i32);
                        components.assign(Component::Day, target_date.day() as i32);
                    }
                    "昨天" | "昨日" | "尋日" | "琴日" => {
                        target_date = ref_date - Duration::days(1);
                        components.assign(Component::Year, target_date.year());
                        components.assign(Component::Month, target_date.month() as i32);
                        components.assign(Component::Day, target_date.day() as i32);
                    }
                    "后天" | "後天" => {
                        target_date = ref_date + Duration::days(2);
                        components.assign(Component::Year, target_date.year());
                        components.assign(Component::Month, target_date.month() as i32);
                        components.assign(Component::Day, target_date.day() as i32);
                    }
                    "前天" => {
                        target_date = ref_date - Duration::days(2);
                        components.assign(Component::Year, target_date.year());
                        components.assign(Component::Month, target_date.month() as i32);
                        components.assign(Component::Day, target_date.day() as i32);
                    }
                    "今晚" | "今夜" => {
                        components.assign(Component::Year, ref_date.year());
                        components.assign(Component::Month, ref_date.month() as i32);
                        components.assign(Component::Day, ref_date.day() as i32);
                        components.imply(Component::Hour, 22);
                        components.assign(Component::Meridiem, Meridiem::PM as i32);
                    }
                    "现在" | "現在" | "而家" => {
                        components.assign(Component::Year, ref_date.year());
                        components.assign(Component::Month, ref_date.month() as i32);
                        components.assign(Component::Day, ref_date.day() as i32);
                        components.assign(Component::Hour, ref_date.hour() as i32);
                        components.assign(Component::Minute, ref_date.minute() as i32);
                        components.assign(Component::Second, ref_date.second() as i32);
                    }
                    _ => {
                        start = match_end;
                        continue;
                    }
                }

                // Apply time period if present
                if let Some(period) = time_part
                    && let Some((hour, meridiem)) = Self::get_time_period_hour(period)
                {
                    // If we also have explicit hour, use it with the meridiem adjustment
                    if let Some(h_str) = hour_str {
                        let mut h = Self::parse_hour(h_str);
                        // Apply meridiem adjustment
                        if let Some(Meridiem::PM) = meridiem
                            && h < 12
                        {
                            h += 12;
                        }
                        components.assign(Component::Hour, h);
                    } else {
                        components.imply(Component::Hour, hour);
                    }
                    if let Some(m) = meridiem {
                        components.assign(Component::Meridiem, m as i32);
                    }
                }

                results.push(context.create_result(match_start, match_end, components, None));
                start = match_end;
                continue;
            }

            // Try standalone time period (implies today)
            if let Ok(Some(caps)) = TIME_ONLY_PATTERN.captures(search_text) {
                let full_match = caps.get(0).unwrap();
                let match_start = start + full_match.start();
                let match_end = start + full_match.end();

                let time_only = caps.name("time_only").map(|m| m.as_str()).unwrap_or("");

                if let Some((hour, meridiem)) = Self::get_time_period_hour(time_only) {
                    let mut components = context.create_components();
                    components.assign(Component::Year, ref_date.year());
                    components.assign(Component::Month, ref_date.month() as i32);
                    components.assign(Component::Day, ref_date.day() as i32);
                    components.imply(Component::Hour, hour);
                    if let Some(m) = meridiem {
                        components.assign(Component::Meridiem, m as i32);
                    }

                    results.push(context.create_result(match_start, match_end, components, None));
                    start = match_end;
                    continue;
                }
            }

            // No match - advance
            if let Some(c) = search_text.chars().next() {
                start += c.len_utf8();
            } else {
                break;
            }
        }

        Ok(results)
    }
}

impl Default for ZHCasualDateParser {
    fn default() -> Self {
        Self::new()
    }
}
