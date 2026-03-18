//! Chinese standard date parser
//!
//! Handles Chinese date formats like:
//! - "2016年9月3号" / "2016年9月3號" (YYYY年M月D号/號)
//! - "9月3号" (M月D号)
//! - "二零一六年九月三号" (Chinese numerals)
//! - "二零一六年，九月三号" (with comma separator)

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::zh::{fullwidth_to_halfwidth, zh_string_to_number};
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use chrono::Datelike;
use fancy_regex::Regex;
use std::sync::LazyLock;

// Pattern for Chinese date: YYYY年M月D号/號/日
// Supports both Arabic numerals and Chinese numerals
// Also supports comma/space between year and month
static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?P<year>[0-9０-９一二三四五六七八九十零〇]+)年[，,\s]*(?P<month>[0-9０-９一二三四五六七八九十]+)月(?P<day>[0-9０-９一二三四五六七八九十]+)(?:号|號|日)"
    ).unwrap()
});

// Pattern for month-day only
static MONTH_DAY_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?P<month>[0-9０-９一二三四五六七八九十]+)月(?P<day>[0-9０-９一二三四五六七八九十]+)(?:号|號|日)"
    ).unwrap()
});

/// Chinese standard date parser
pub struct ZHStandardDateParser;

impl ZHStandardDateParser {
    pub fn new() -> Self {
        Self
    }

    fn parse_number(s: &str) -> i32 {
        // First convert full-width to half-width
        let hankaku = fullwidth_to_halfwidth(s);

        // Try parsing as regular number
        if let Ok(n) = hankaku.parse::<i32>() {
            return n;
        }

        // Try as Chinese numerals
        zh_string_to_number(s) as i32
    }

    fn parse_year(s: &str) -> i32 {
        // For years like "二零一六", we need special handling
        // It's written digit by digit, not like "二千零一十六"
        let hankaku = fullwidth_to_halfwidth(s);

        // Try parsing as regular number first
        if let Ok(n) = hankaku.parse::<i32>() {
            // Handle 2-digit years
            if n < 100 {
                return if n > 50 { 1900 + n } else { 2000 + n };
            }
            return n;
        }

        // For Chinese numerals written digit-by-digit (二零一六)
        // We need to handle each character
        let mut result = 0i32;
        let mut has_chinese = false;

        for c in s.chars() {
            let c_str = c.to_string();
            let digit = match c_str.as_str() {
                "零" | "〇" => {
                    has_chinese = true;
                    0
                }
                "一" => {
                    has_chinese = true;
                    1
                }
                "二" => {
                    has_chinese = true;
                    2
                }
                "三" => {
                    has_chinese = true;
                    3
                }
                "四" => {
                    has_chinese = true;
                    4
                }
                "五" => {
                    has_chinese = true;
                    5
                }
                "六" => {
                    has_chinese = true;
                    6
                }
                "七" => {
                    has_chinese = true;
                    7
                }
                "八" => {
                    has_chinese = true;
                    8
                }
                "九" => {
                    has_chinese = true;
                    9
                }
                _ => continue,
            };
            result = result * 10 + digit;
        }

        if has_chinese && result > 0 {
            // Handle 2-digit years
            if result < 100 {
                return if result > 50 {
                    1900 + result
                } else {
                    2000 + result
                };
            }
            return result;
        }

        // Fallback to standard conversion
        zh_string_to_number(s) as i32
    }

    fn is_valid_date(year: i32, month: i32, day: i32) -> bool {
        if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
            return false;
        }
        let days_in_month = match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                    29
                } else {
                    28
                }
            }
            _ => return false,
        };
        day <= days_in_month
    }
}

impl Parser for ZHStandardDateParser {
    fn name(&self) -> &'static str {
        "ZHStandardDateParser"
    }

    fn should_apply(&self, context: &ParsingContext) -> bool {
        context.text.contains('月')
            && (context.text.contains('号')
                || context.text.contains('號')
                || context.text.contains('日'))
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();
        let ref_date = context.reference.instant;

        let mut start = 0;
        while start < context.text.len() {
            let search_text = &context.text[start..];

            // Try full pattern (with year) first
            if let Ok(Some(caps)) = PATTERN.captures(search_text) {
                let full_match = caps.get(0).unwrap();
                let match_start = start + full_match.start();
                let match_end = start + full_match.end();

                let year = caps
                    .name("year")
                    .map(|m| Self::parse_year(m.as_str()))
                    .unwrap_or(0);
                let month = caps
                    .name("month")
                    .map(|m| Self::parse_number(m.as_str()))
                    .unwrap_or(0);
                let day = caps
                    .name("day")
                    .map(|m| Self::parse_number(m.as_str()))
                    .unwrap_or(0);

                if Self::is_valid_date(year, month, day) {
                    let mut components = context.create_components();
                    components.assign(Component::Year, year);
                    components.assign(Component::Month, month);
                    components.assign(Component::Day, day);

                    results.push(context.create_result(match_start, match_end, components, None));
                    start = match_end;
                    continue;
                }
            }

            // Try month-day pattern
            if let Ok(Some(caps)) = MONTH_DAY_PATTERN.captures(search_text) {
                let full_match = caps.get(0).unwrap();
                let match_start = start + full_match.start();
                let match_end = start + full_match.end();

                let month = caps
                    .name("month")
                    .map(|m| Self::parse_number(m.as_str()))
                    .unwrap_or(0);
                let day = caps
                    .name("day")
                    .map(|m| Self::parse_number(m.as_str()))
                    .unwrap_or(0);
                let year = ref_date.year();

                if Self::is_valid_date(year, month, day) {
                    let mut components = context.create_components();
                    components.imply(Component::Year, year);
                    components.assign(Component::Month, month);
                    components.assign(Component::Day, day);

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

impl Default for ZHStandardDateParser {
    fn default() -> Self {
        Self::new()
    }
}
