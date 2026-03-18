//! Chinese time unit within parser
//!
//! Handles Chinese "within X time" expressions like:
//! - "五日内" (within 5 days)
//! - "三天内" (within 3 days)
//! - "两周内" (within 2 weeks)

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::TimeUnit;
use crate::dictionaries::zh::{get_time_unit, parse_number_pattern};
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use chrono::{Datelike, Duration, Timelike};
use fancy_regex::Regex;
use std::sync::LazyLock;

// Pattern for "X units within" expressions in Chinese
static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?P<number>[0-9一二三四五六七八九十两兩]+)(?P<unit>秒|秒钟|秒鐘|分|分钟|分鐘|小时|小時|钟头|鐘頭|天|日|周|週|星期|礼拜|禮拜|个月|個月|月|年)(?:之?内|之?內)"
    ).unwrap()
});

/// Chinese time unit within parser
pub struct ZHTimeUnitWithinParser;

impl ZHTimeUnitWithinParser {
    pub fn new() -> Self {
        Self
    }

    fn parse_number(s: &str) -> i32 {
        parse_number_pattern(s) as i32
    }
}

impl Parser for ZHTimeUnitWithinParser {
    fn name(&self) -> &'static str {
        "ZHTimeUnitWithinParser"
    }

    fn should_apply(&self, context: &ParsingContext) -> bool {
        context.text.contains('内') || context.text.contains('內')
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();
        let ref_date = context.reference.instant;

        let mut start = 0;
        while start < context.text.len() {
            let search_text = &context.text[start..];

            if let Ok(Some(caps)) = PATTERN.captures(search_text) {
                let full_match = caps.get(0).unwrap();
                let match_start = start + full_match.start();
                let match_end = start + full_match.end();

                let number = caps
                    .name("number")
                    .map(|m| Self::parse_number(m.as_str()))
                    .unwrap_or(1);
                let unit_str = caps.name("unit").map(|m| m.as_str()).unwrap_or("");

                let unit = get_time_unit(unit_str);

                if let Some(time_unit) = unit {
                    let target_date = match time_unit {
                        TimeUnit::Second => ref_date + Duration::seconds(number as i64),
                        TimeUnit::Minute => ref_date + Duration::minutes(number as i64),
                        TimeUnit::Hour => ref_date + Duration::hours(number as i64),
                        TimeUnit::Day => ref_date + Duration::days(number as i64),
                        TimeUnit::Week => ref_date + Duration::weeks(number as i64),
                        TimeUnit::Month => {
                            let new_month = ref_date.month() as i32 + number;
                            let years_to_add = (new_month - 1) / 12;
                            let final_month = ((new_month - 1) % 12) + 1;
                            ref_date
                                .with_year(ref_date.year() + years_to_add)
                                .and_then(|d| d.with_month(final_month as u32))
                                .unwrap_or(ref_date)
                        }
                        TimeUnit::Year => ref_date
                            .with_year(ref_date.year() + number)
                            .unwrap_or(ref_date),
                        _ => ref_date + Duration::days(number as i64),
                    };

                    let mut components = context.create_components();
                    components.assign(Component::Year, target_date.year());
                    components.assign(Component::Month, target_date.month() as i32);
                    components.assign(Component::Day, target_date.day() as i32);

                    // For time units smaller than a day, also set time
                    match time_unit {
                        TimeUnit::Second | TimeUnit::Minute | TimeUnit::Hour => {
                            components.assign(Component::Hour, target_date.hour() as i32);
                            components.assign(Component::Minute, target_date.minute() as i32);
                            if time_unit == TimeUnit::Second {
                                components.assign(Component::Second, target_date.second() as i32);
                            }
                        }
                        _ => {}
                    }

                    results.push(context.create_result(match_start, match_end, components, None));
                }

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

impl Default for ZHTimeUnitWithinParser {
    fn default() -> Self {
        Self::new()
    }
}
