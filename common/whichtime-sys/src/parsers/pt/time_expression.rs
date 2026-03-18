//! Portuguese time expression parser
//!
//! Handles Portuguese time expressions like:
//! - "6.13 AM" (dot separator)
//! - "às 6:30pm"
//! - "de 6:30pm a 11:00pm" (ranges)
//! - "8:10 - 12.32" (ranges)

use crate::components::Component;
use crate::context::ParsingContext;
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use crate::types::Meridiem;
use fancy_regex::Regex;
use std::sync::LazyLock;

// Primary pattern: handles single times and "de X a Y" ranges
// Supports both : and . as separators
// Matches:
// - às 6.13 AM
// - 8:10
// - de 1pm a 3
static PRIMARY_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?ix)
        (?<!\d[:\.])(?<!\w)
        (?:de\s+)?
        (?:às|as|a|ao)?\s*
        (?P<hour1>\d{1,2})
        (?:[:\.](?P<minute1>\d{2}))?
        (?:[:\.](?P<second1>\d{2}))?
        (?:\s*(?P<meridiem1>a\.?m\.?|p\.?m\.?))?
        (?:
            \s*(?:a|às|as|[\-–~])\s*
            (?P<hour2>\d{1,2})
            (?:[:\.](?P<minute2>\d{2}))?
            (?:[:\.](?P<second2>\d{2}))?
            (?:\s*(?P<meridiem2>a\.?m\.?|p\.?m\.?))?
        )?
        (?=\W|$)
        ",
    )
    .unwrap()
});

/// Portuguese time expression parser
pub struct PTTimeExpressionParser;

impl PTTimeExpressionParser {
    pub fn new() -> Self {
        Self
    }

    fn parse_meridiem(s: &str) -> Option<Meridiem> {
        let lower = s.to_lowercase();
        if lower.starts_with('p') {
            Some(Meridiem::PM)
        } else if lower.starts_with('a') {
            Some(Meridiem::AM)
        } else {
            None
        }
    }

    fn adjust_hour(hour: i32, meridiem: Option<Meridiem>) -> i32 {
        match meridiem {
            Some(Meridiem::PM) => {
                if hour < 12 {
                    hour + 12
                } else {
                    hour
                }
            }
            Some(Meridiem::AM) => {
                if hour == 12 {
                    0
                } else {
                    hour
                }
            }
            None => hour,
        }
    }
}

impl Parser for PTTimeExpressionParser {
    fn name(&self) -> &'static str {
        "PTTimeExpressionParser"
    }

    fn should_apply(&self, context: &ParsingContext) -> bool {
        let text = context.text;
        text.bytes().any(|b| b.is_ascii_digit())
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();
        let ref_date = context.reference.instant;

        let mut start = 0;
        while start < context.text.len() {
            let search_text = &context.text[start..];
            let captures = match PRIMARY_PATTERN.captures(search_text) {
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
            let matched_text = full_match.as_str();

            let hour1: i32 = captures
                .name("hour1")
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);
            let minute1: i32 = captures
                .name("minute1")
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);
            let second1: i32 = captures
                .name("second1")
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);
            let meridiem1 = captures
                .name("meridiem1")
                .map(|m| m.as_str())
                .and_then(Self::parse_meridiem);

            // Validate hours
            if hour1 > 23 {
                start = match_end;
                continue;
            }

            let hour2_opt = captures.name("hour2").and_then(|m| m.as_str().parse().ok());
            let minute2: i32 = captures
                .name("minute2")
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);
            let second2: i32 = captures
                .name("second2")
                .and_then(|m| m.as_str().parse().ok())
                .unwrap_or(0);
            let meridiem2 = captures
                .name("meridiem2")
                .map(|m| m.as_str())
                .and_then(Self::parse_meridiem);

            // Heuristic: If no meridiem and no minutes/seconds, ensure context supports it (like "às", "de")
            // or regex handles it via `(?:de\s+)?...`. But regex allows loose matches.
            // If simple number "6", regex matches. We want to avoid false positives.
            let has_context_prefix = matched_text.to_lowercase().contains("às")
                || matched_text.to_lowercase().contains("as")
                || matched_text.to_lowercase().contains("de ")
                || matched_text.to_lowercase().starts_with("a "); // careful with "a"

            let has_time_separator = matched_text.contains(':')
                || (matched_text.contains('.')
                    && matched_text
                        .chars()
                        .any(|c| c == 'a' || c == 'p' || c == 'A' || c == 'P')); // Dot usually only with AM/PM or specific contexts?

            let has_meridiem = meridiem1.is_some() || meridiem2.is_some();

            if !has_time_separator && !has_meridiem && !has_context_prefix && hour2_opt.is_none() {
                // Just a number? e.g. "10" in "10 Agosto" might be matched?
                // Regex `(?<!\d)` helps but `Agosto` follows.
                // If we match "10" in "10 Agosto", we shouldn't treat it as time.
                // We should let date parser handle "10 Agosto".
                // But date parser runs separately.
                // OverlapRemovalRefiner should handle conflicts if ranges overlap.
                // But simpler is to skip if not confident.
                start = match_end;
                continue;
            }

            // Adjust hours
            let adj_hour1 = Self::adjust_hour(hour1, meridiem1);

            let mut components = context.create_components();
            components.assign(Component::Hour, adj_hour1);
            components.assign(Component::Minute, minute1);
            components.assign(Component::Second, second1);

            if let Some(m) = meridiem1 {
                components.assign(Component::Meridiem, m as i32);
            } else {
                // If hours > 12, imply PM (already handled by 24h input, but set flag?)
                if hour1 >= 12 {
                    components.assign(Component::Meridiem, Meridiem::PM as i32);
                } else if hour1 < 12 {
                    // Ambiguous. Leave unset.
                }
            }

            // Handle Range
            let end_components = if let Some(hour2) = hour2_opt {
                if hour2 > 23 {
                    None
                } else {
                    // Inherit meridiem from end to start if start missing?
                    // e.g. "1 to 3 pm" -> 1pm to 3pm.
                    // "de 1pm a 3" -> 1pm to 3pm?

                    let final_meridiem2 = meridiem2.or(meridiem1);

                    // Also check start time inheritance?
                    // "5 - 7pm" -> 5pm - 7pm.

                    // Adjust start hour if needed (logic from `en` parser)
                    // If start is ambiguous (no meridiem, < 12) and end is PM, maybe start is PM?
                    // Or if end is AM, start is AM.
                    // But here we construct end components separately.

                    let mut end_comp = context.create_components();
                    let adj_hour2 = Self::adjust_hour(hour2, final_meridiem2);

                    end_comp.assign(Component::Hour, adj_hour2);
                    end_comp.assign(Component::Minute, minute2);
                    end_comp.assign(Component::Second, second2);
                    if let Some(m) = final_meridiem2 {
                        end_comp.assign(Component::Meridiem, m as i32);
                    } else if hour2 >= 12 {
                        end_comp.assign(Component::Meridiem, Meridiem::PM as i32);
                    }

                    // Copy date from reference
                    use chrono::Datelike;
                    end_comp.imply(Component::Year, ref_date.year());
                    end_comp.imply(Component::Month, ref_date.month() as i32);
                    end_comp.imply(Component::Day, ref_date.day() as i32);

                    Some(end_comp)
                }
            } else {
                None
            };

            // Re-adjust start hour if we learned from end meridiem?
            // "de 1pm a 3" -> 1pm to 3pm (test case). Here mer1=PM, mer2=None.
            // My logic: `effective_meridiem2 = meridiem2.or(meridiem1)`. So mer2 becomes PM. Correct.

            // What about "5 - 7pm"? mer1=None, mer2=PM.
            // effective_meridiem2 = PM.
            // Does start inherit PM?
            // If hour1 < 12 and hour2 < 12 and mer2=PM -> likely start is PM too?
            // e.g. 5 - 7pm -> 17:00 - 19:00.
            // e.g. 10 - 2pm -> 10am - 2pm? Or 10pm - 2am? Probably 10am.
            // Only imply if start < end?
            // Standard whichtime logic usually implies start meridiem from end if sensible.
            // But let's stick to basics unless test fails.

            results.push(context.create_result(match_start, match_end, components, end_components));

            start = match_end;
        }

        Ok(results)
    }
}

impl Default for PTTimeExpressionParser {
    fn default() -> Self {
        Self::new()
    }
}
