//! Japanese time expression parser
//!
//! Handles Japanese time formats like:
//! - "午前6時13分" (AM 6:13)
//! - "午後8時" (PM 8:00)
//! - "午後三時半五十九秒" (PM 3:30:59 with kanji numbers)
//! - "6時30分PM" (6:30 PM)
//! - Time ranges: "午前八時十分から午後11時32分"

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::ja::{ja_string_to_number, to_hankaku};
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use crate::types::Meridiem;
use fancy_regex::Regex;
use std::sync::LazyLock;

// Pattern for Japanese time: [午前/午後]H時[M分][S秒][AM/PM]
static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?P<meridiem1>午前|午後)?(?P<hour>[0-9０-９一二三四五六七八九十]+)時(?!間)(?P<minute>[0-9０-９一二三四五六七八九十]+)?(?:分)?(?P<half>半)?(?P<second>[0-9０-９一二三四五六七八九十]+秒)?(?P<meridiem2>AM|PM|am|pm)?"
    ).unwrap()
});

// Pattern for time range with から (from)
static RANGE_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?P<meridiem1>午前|午後)?(?P<hour1>[0-9０-９一二三四五六七八九十]+)時(?!間)(?P<minute1>[0-9０-９一二三四五六七八九十]+)?(?:分)?(?P<half1>半)?(?P<second1>[0-9０-９一二三四五六七八九十]+秒)?(?P<pm1>AM|PM|am|pm)?(?:から|[-~～ー])(?P<meridiem2>午前|午後)?(?P<hour2>[0-9０-９一二三四五六七八九十]+)時(?!間)(?P<minute2>[0-9０-９一二三四五六七八九十]+)?(?:分)?(?P<half2>半)?(?P<second2>[0-9０-９一二三四五六七八九十]+秒)?(?P<pm2>AM|PM|am|pm)?"
    ).unwrap()
});

/// Japanese time expression parser
pub struct JATimeExpressionParser;

impl JATimeExpressionParser {
    pub fn new() -> Self {
        Self
    }

    fn parse_number(s: &str) -> i32 {
        // First try as regular number (with hankaku conversion)
        let hankaku = to_hankaku(s);
        if let Ok(n) = hankaku.parse::<i32>() {
            return n;
        }
        // Try as Japanese kanji number
        ja_string_to_number(s) as i32
    }

    fn parse_minute(s: &str) -> i32 {
        // Handle "半" (half) = 30
        if s.contains('半') {
            return 30;
        }
        Self::parse_number(s)
    }

    fn apply_meridiem(
        hour: i32,
        meridiem: Option<&str>,
        pm_suffix: Option<&str>,
        fallback: Option<Meridiem>,
    ) -> Option<(i32, Option<Meridiem>)> {
        if !(0..=23).contains(&hour) {
            return None;
        }

        let suffix_upper = pm_suffix.map(|s| s.to_ascii_uppercase());
        let suffix_ref = suffix_upper.as_deref();

        let is_pm = matches!(meridiem, Some("午後")) || suffix_ref == Some("PM");
        let is_am = matches!(meridiem, Some("午前")) || suffix_ref == Some("AM");

        if is_pm {
            if hour > 12 {
                return None;
            }
            let adjusted_hour = if hour < 12 { hour + 12 } else { hour };
            return Some((adjusted_hour, Some(Meridiem::PM)));
        }

        if is_am {
            if hour > 12 {
                return None;
            }
            let adjusted_hour = if hour == 12 { 0 } else { hour };
            return Some((adjusted_hour, Some(Meridiem::AM)));
        }

        if let Some(fallback_mer) = fallback {
            if hour > 12 {
                return None;
            }
            let adjusted_hour = match fallback_mer {
                Meridiem::PM => {
                    if hour < 12 {
                        hour + 12
                    } else {
                        hour
                    }
                }
                Meridiem::AM => {
                    if hour == 12 {
                        0
                    } else {
                        hour
                    }
                }
            };
            return Some((adjusted_hour, Some(fallback_mer)));
        }

        Some((
            hour,
            if hour >= 12 {
                Some(Meridiem::PM)
            } else {
                Some(Meridiem::AM)
            },
        ))
    }
}

impl Parser for JATimeExpressionParser {
    fn name(&self) -> &'static str {
        "JATimeExpressionParser"
    }

    fn should_apply(&self, context: &ParsingContext) -> bool {
        context.text.contains('時')
            || context.text.contains("午前")
            || context.text.contains("午後")
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();

        let mut start = 0;
        while start < context.text.len() {
            let search_text = &context.text[start..];

            // Try range pattern first
            if let Ok(Some(caps)) = RANGE_PATTERN.captures(search_text) {
                let full_match = caps.get(0).unwrap();
                let match_start = start + full_match.start();
                let match_end = start + full_match.end();

                let hour1 = caps
                    .name("hour1")
                    .map(|m| Self::parse_number(m.as_str()))
                    .unwrap_or(0);
                let mut minute1 = caps
                    .name("minute1")
                    .map(|m| Self::parse_minute(m.as_str()))
                    .unwrap_or(0);
                let second1 = caps
                    .name("second1")
                    .map(|m| Self::parse_number(m.as_str().trim_end_matches('秒')))
                    .unwrap_or(0);
                let meridiem1 = caps.name("meridiem1").map(|m| m.as_str());
                let pm1 = caps.name("pm1").map(|m| m.as_str());

                let hour2 = caps
                    .name("hour2")
                    .map(|m| Self::parse_number(m.as_str()))
                    .unwrap_or(0);
                let mut minute2 = caps
                    .name("minute2")
                    .map(|m| Self::parse_minute(m.as_str()))
                    .unwrap_or(0);
                let second2 = caps
                    .name("second2")
                    .map(|m| Self::parse_number(m.as_str().trim_end_matches('秒')))
                    .unwrap_or(0);
                let meridiem2 = caps.name("meridiem2").map(|m| m.as_str());
                let pm2 = caps.name("pm2").map(|m| m.as_str());

                if caps.name("half1").is_some() {
                    minute1 = if minute1 == 0 { 30 } else { minute1 + 30 };
                }
                if caps.name("half2").is_some() {
                    minute2 = if minute2 == 0 { 30 } else { minute2 + 30 };
                }

                if minute1 >= 60 || minute2 >= 60 || second1 >= 60 || second2 >= 60 {
                    start = match_end;
                    continue;
                }

                let Some((adj_hour1, mer1)) = Self::apply_meridiem(hour1, meridiem1, pm1, None)
                else {
                    start = match_end;
                    continue;
                };

                let fallback_meridiem = if meridiem2.is_none() && pm2.is_none() {
                    mer1
                } else {
                    None
                };

                let Some((adj_hour2, mer2)) =
                    Self::apply_meridiem(hour2, meridiem2, pm2, fallback_meridiem)
                else {
                    start = match_end;
                    continue;
                };

                let mut components = context.create_components();
                components.assign(Component::Hour, adj_hour1);
                components.assign(Component::Minute, minute1);
                if second1 > 0 {
                    components.assign(Component::Second, second1);
                }
                if let Some(m) = mer1 {
                    components.assign(Component::Meridiem, m as i32);
                }

                let mut end_comp = context.create_components();
                end_comp.assign(Component::Hour, adj_hour2);
                end_comp.assign(Component::Minute, minute2);
                if second2 > 0 {
                    end_comp.assign(Component::Second, second2);
                }
                if let Some(m) = mer2 {
                    end_comp.assign(Component::Meridiem, m as i32);
                }

                results.push(context.create_result(
                    match_start,
                    match_end,
                    components,
                    Some(end_comp),
                ));
                start = match_end;
                continue;
            }

            // Try single time pattern
            if let Ok(Some(caps)) = PATTERN.captures(search_text) {
                let full_match = caps.get(0).unwrap();
                let match_start = start + full_match.start();
                let match_end = start + full_match.end();

                let hour = caps
                    .name("hour")
                    .map(|m| Self::parse_number(m.as_str()))
                    .unwrap_or(0);
                let mut minute = caps
                    .name("minute")
                    .map(|m| Self::parse_minute(m.as_str()))
                    .unwrap_or(0);
                let second = caps
                    .name("second")
                    .map(|m| Self::parse_number(m.as_str().trim_end_matches('秒')))
                    .unwrap_or(0);
                let meridiem = caps.name("meridiem1").map(|m| m.as_str());
                let pm_suffix = caps.name("meridiem2").map(|m| m.as_str());

                if caps.name("half").is_some() {
                    minute = if minute == 0 { 30 } else { minute + 30 };
                }

                if minute >= 60 || second >= 60 {
                    start = match_end;
                    continue;
                }

                let Some((adj_hour, mer)) = Self::apply_meridiem(hour, meridiem, pm_suffix, None)
                else {
                    start = match_end;
                    continue;
                };

                let mut components = context.create_components();
                components.assign(Component::Hour, adj_hour);
                components.assign(Component::Minute, minute);
                if second > 0 {
                    components.assign(Component::Second, second);
                }
                if let Some(m) = mer {
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

impl Default for JATimeExpressionParser {
    fn default() -> Self {
        Self::new()
    }
}
