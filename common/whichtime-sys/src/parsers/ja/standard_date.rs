//! Japanese standard date parser
//!
//! Handles Japanese date formats like:
//! - "2012年3月31日" (YYYY年M月D日)
//! - "９月3日" (M月D日 with full-width numbers)
//! - "平成26年12月29日" (Era year format)
//! - "令和元年5月1日" (Reiwa era with gannen)
//! - "同年7月27日", "本年7月27日", "今年7月27日" (relative year)
//! - Date ranges: "2013年12月26日-2014年1月7日"

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::ja::to_hankaku;
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use chrono::Datelike;
use fancy_regex::Regex;
use std::sync::LazyLock;

// Pattern for standard Japanese date: [Era/Year]年[M]月[D]日
static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?:(?P<era>平成|令和|昭和|大正|明治)(?P<era_year>[0-9０-９]+|元)年|(?P<year_prefix>同年|本年|今年)|(?P<year>[0-9０-９]{2,4})年)?(?P<month>[0-9０-９]{1,2})月(?P<day>[0-9０-９]{1,2})日"
    ).unwrap()
});

// Pattern for date range
static RANGE_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?P<year1>[0-9０-９]{2,4})年(?P<month1>[0-9０-９]{1,2})月(?P<day1>[0-9０-９]{1,2})日\s*[-~～ー]\s*(?P<year2>[0-9０-９]{2,4})年(?P<month2>[0-9０-９]{1,2})月(?P<day2>[0-9０-９]{1,2})日"
    ).unwrap()
});

/// Japanese standard date parser
pub struct JAStandardDateParser;

impl JAStandardDateParser {
    pub fn new() -> Self {
        Self
    }

    fn parse_number(s: &str) -> i32 {
        let hankaku = to_hankaku(s);
        hankaku.parse().unwrap_or(0)
    }

    fn era_to_year(era: &str, era_year: &str) -> Option<i32> {
        let year_num = if era_year == "元" {
            1
        } else {
            Self::parse_number(era_year)
        };

        let base_year = match era {
            "令和" => 2018, // Reiwa 1 = 2019
            "平成" => 1988, // Heisei 1 = 1989
            "昭和" => 1925, // Showa 1 = 1926
            "大正" => 1911, // Taisho 1 = 1912
            "明治" => 1867, // Meiji 1 = 1868
            _ => return None,
        };

        Some(base_year + year_num)
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

impl Parser for JAStandardDateParser {
    fn name(&self) -> &'static str {
        "JAStandardDateParser"
    }

    fn should_apply(&self, context: &ParsingContext) -> bool {
        context.text.contains('月') && context.text.contains('日')
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();
        let ref_date = context.reference.instant;

        let mut start = 0;
        while start < context.text.len() {
            let search_text = &context.text[start..];

            // Try range pattern first
            if let Ok(Some(caps)) = RANGE_PATTERN.captures(search_text) {
                let full_match = caps.get(0).unwrap();
                let match_start = start + full_match.start();
                let match_end = start + full_match.end();

                let year1 = caps
                    .name("year1")
                    .map(|m| Self::parse_number(m.as_str()))
                    .unwrap_or(0);
                let month1 = caps
                    .name("month1")
                    .map(|m| Self::parse_number(m.as_str()))
                    .unwrap_or(0);
                let day1 = caps
                    .name("day1")
                    .map(|m| Self::parse_number(m.as_str()))
                    .unwrap_or(0);

                let year2 = caps
                    .name("year2")
                    .map(|m| Self::parse_number(m.as_str()))
                    .unwrap_or(0);
                let month2 = caps
                    .name("month2")
                    .map(|m| Self::parse_number(m.as_str()))
                    .unwrap_or(0);
                let day2 = caps
                    .name("day2")
                    .map(|m| Self::parse_number(m.as_str()))
                    .unwrap_or(0);

                if Self::is_valid_date(year1, month1, day1)
                    && Self::is_valid_date(year2, month2, day2)
                {
                    let mut components = context.create_components();
                    components.assign(Component::Year, year1);
                    components.assign(Component::Month, month1);
                    components.assign(Component::Day, day1);

                    let mut end_comp = context.create_components();
                    end_comp.assign(Component::Year, year2);
                    end_comp.assign(Component::Month, month2);
                    end_comp.assign(Component::Day, day2);

                    results.push(context.create_result(
                        match_start,
                        match_end,
                        components,
                        Some(end_comp),
                    ));
                    start = match_end;
                    continue;
                }
            }

            // Try standard pattern
            if let Ok(Some(caps)) = PATTERN.captures(search_text) {
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

                // Determine year
                let year = if let (Some(era), Some(era_year)) =
                    (caps.name("era"), caps.name("era_year"))
                {
                    Self::era_to_year(era.as_str(), era_year.as_str()).unwrap_or(ref_date.year())
                } else if let Some(_year_prefix) = caps.name("year_prefix") {
                    // 同年, 本年, 今年 all mean current year
                    ref_date.year()
                } else if let Some(year_match) = caps.name("year") {
                    let y = Self::parse_number(year_match.as_str());
                    if y < 100 {
                        if y > 50 { 1900 + y } else { 2000 + y }
                    } else {
                        y
                    }
                } else {
                    // No year specified - imply current year
                    ref_date.year()
                };

                if Self::is_valid_date(year, month, day) {
                    let mut components = context.create_components();

                    // If year was explicitly specified, assign it; otherwise imply it
                    if caps.name("year").is_some() || caps.name("era").is_some() {
                        components.assign(Component::Year, year);
                    } else if caps.name("year_prefix").is_some() {
                        // 同年, 本年, 今年 - assign the year
                        components.assign(Component::Year, year);
                    } else {
                        components.imply(Component::Year, year);
                    }
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

impl Default for JAStandardDateParser {
    fn default() -> Self {
        Self::new()
    }
}
