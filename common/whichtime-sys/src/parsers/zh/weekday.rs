//! Chinese weekday parser
//!
//! Handles Chinese weekday expressions like:
//! - "星期四" (Thursday)
//! - "周一" (Monday)
//! - "下周三" (next Wednesday)
//! - "上週五" (last Friday)

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::zh::get_weekday;
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use crate::types::Weekday;
use chrono::{Datelike, Duration};
use fancy_regex::Regex;
use std::sync::LazyLock;

// Pattern for Chinese weekday
static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?P<modifier>这|這|这个|這個|本|今|下|下个|下個|上|上个|上個)?(?P<weekday>星期日|星期天|周日|週日|礼拜日|禮拜日|礼拜天|禮拜天|星期一|周一|週一|礼拜一|禮拜一|星期二|周二|週二|礼拜二|禮拜二|星期三|周三|週三|礼拜三|禮拜三|星期四|周四|週四|礼拜四|禮拜四|星期五|周五|週五|礼拜五|禮拜五|星期六|周六|週六|礼拜六|禮拜六)"
    ).unwrap()
});

/// Chinese weekday parser
pub struct ZHWeekdayParser;

impl ZHWeekdayParser {
    pub fn new() -> Self {
        Self
    }
}

impl Parser for ZHWeekdayParser {
    fn name(&self) -> &'static str {
        "ZHWeekdayParser"
    }

    fn should_apply(&self, context: &ParsingContext) -> bool {
        context.text.contains("星期")
            || context.text.contains("周")
            || context.text.contains("週")
            || context.text.contains("礼拜")
            || context.text.contains("禮拜")
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

                let modifier = caps.name("modifier").map(|m| m.as_str());
                let weekday_str = caps.name("weekday").map(|m| m.as_str()).unwrap_or("");

                if let Some(weekday) = get_weekday(weekday_str) {
                    let target_weekday = match weekday {
                        Weekday::Sunday => chrono::Weekday::Sun,
                        Weekday::Monday => chrono::Weekday::Mon,
                        Weekday::Tuesday => chrono::Weekday::Tue,
                        Weekday::Wednesday => chrono::Weekday::Wed,
                        Weekday::Thursday => chrono::Weekday::Thu,
                        Weekday::Friday => chrono::Weekday::Fri,
                        Weekday::Saturday => chrono::Weekday::Sat,
                    };

                    let current_weekday = ref_date.weekday();
                    let current_num = current_weekday.num_days_from_sunday() as i64;
                    let target_num = target_weekday.num_days_from_sunday() as i64;

                    // Calculate days offset based on modifier
                    let days_offset = match modifier {
                        Some("下") | Some("下个") | Some("下個") => {
                            // Next week
                            let diff = target_num - current_num;
                            (if diff <= 0 { diff + 7 } else { diff }) + 7
                        }
                        Some("上") | Some("上个") | Some("上個") => {
                            // Last week
                            let diff = target_num - current_num;
                            (if diff >= 0 { diff - 7 } else { diff }) - 7
                        }
                        Some("这") | Some("這") | Some("这个") | Some("這個") | Some("本")
                        | Some("今") => {
                            // This week (closest)

                            target_num - current_num
                        }
                        None | Some(_) => {
                            // No modifier - find closest occurrence (prefer past for weekdays before today)
                            let diff = target_num - current_num;
                            if diff > 0 {
                                diff - 7 // Past occurrence
                            } else if diff < 0 {
                                diff // Past occurrence
                            } else {
                                0 // Today
                            }
                        }
                    };

                    let target_date = ref_date + Duration::days(days_offset);

                    let mut components = context.create_components();
                    components.assign(Component::Year, target_date.year());
                    components.assign(Component::Month, target_date.month() as i32);
                    components.assign(Component::Day, target_date.day() as i32);
                    components.assign(Component::Weekday, weekday as i32);

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

impl Default for ZHWeekdayParser {
    fn default() -> Self {
        Self::new()
    }
}
