//! Chinese time expression parser
//!
//! Handles Chinese time formats like:
//! - "上午6点13分" (AM 6:13)
//! - "下午8点" (PM 8:00)
//! - "6点30分" (6:30)
//! - "下午三點半" (PM 3:30 with traditional characters)

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::zh::{fullwidth_to_halfwidth, parse_number_pattern};
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use crate::types::Meridiem;
use fancy_regex::Regex;
use std::sync::LazyLock;

// Pattern for Chinese time: [上午/下午]H点[M分][S秒]
static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?P<meridiem>上午|下午|凌晨|早上|晚上|中午)?(?P<hour>[0-9０-９一二三四五六七八九十两兩]+)(?:点|點)(?:(?P<minute>[0-9０-９一二三四五六七八九十]+)分)?(?P<half>半)?(?:(?P<second>[0-9０-９一二三四五六七八九十]+)秒)?"
    ).unwrap()
});

/// Chinese time expression parser
pub struct ZHTimeExpressionParser;

impl ZHTimeExpressionParser {
    pub fn new() -> Self {
        Self
    }

    fn parse_number(s: &str) -> i32 {
        let hankaku = fullwidth_to_halfwidth(s);
        if let Ok(n) = hankaku.parse::<i32>() {
            return n;
        }
        parse_number_pattern(s) as i32
    }

    fn get_meridiem_info(meridiem: &str) -> (Option<Meridiem>, bool) {
        match meridiem {
            "上午" | "凌晨" | "早上" => (Some(Meridiem::AM), true),
            "下午" | "晚上" => (Some(Meridiem::PM), true),
            "中午" => (Some(Meridiem::PM), false), // noon, don't adjust
            _ => (None, false),
        }
    }
}

impl Parser for ZHTimeExpressionParser {
    fn name(&self) -> &'static str {
        "ZHTimeExpressionParser"
    }

    fn should_apply(&self, context: &ParsingContext) -> bool {
        context.text.contains('点') || context.text.contains('點')
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();

        let mut start = 0;
        while start < context.text.len() {
            let search_text = &context.text[start..];

            if let Ok(Some(caps)) = PATTERN.captures(search_text) {
                let full_match = caps.get(0).unwrap();
                let match_start = start + full_match.start();
                let match_end = start + full_match.end();

                let mut hour = caps
                    .name("hour")
                    .map(|m| Self::parse_number(m.as_str()))
                    .unwrap_or(0);
                let mut minute = caps
                    .name("minute")
                    .map(|m| Self::parse_number(m.as_str()))
                    .unwrap_or(0);
                let second = caps
                    .name("second")
                    .map(|m| Self::parse_number(m.as_str()))
                    .unwrap_or(0);

                // Handle "半" (half) = 30 minutes
                if caps.name("half").is_some() {
                    minute = if minute == 0 { 30 } else { minute + 30 };
                }

                let meridiem_str = caps.name("meridiem").map(|m| m.as_str());

                // Validate
                if hour > 24 || minute >= 60 || second >= 60 {
                    start = match_end;
                    continue;
                }

                // Apply meridiem
                let mut meridiem_val = None;
                if let Some(m_str) = meridiem_str {
                    let (mer, should_adjust) = Self::get_meridiem_info(m_str);
                    meridiem_val = mer;

                    if should_adjust {
                        if let Some(Meridiem::PM) = mer {
                            if hour < 12 {
                                hour += 12;
                            }
                        } else if let Some(Meridiem::AM) = mer
                            && hour == 12
                        {
                            hour = 0;
                        }
                    }
                }

                let mut components = context.create_components();
                components.assign(Component::Hour, hour);
                components.assign(Component::Minute, minute);
                if second > 0 {
                    components.assign(Component::Second, second);
                }
                if let Some(m) = meridiem_val {
                    components.assign(Component::Meridiem, m as i32);
                }

                results.push(context.create_result(match_start, match_end, components, None));
                start = match_end;
                continue;
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

impl Default for ZHTimeExpressionParser {
    fn default() -> Self {
        Self::new()
    }
}
