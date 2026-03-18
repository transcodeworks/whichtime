//! French weekday parser
//!
//! Handles French weekday expressions like:
//! - "lundi", "mardi", etc.
//! - "vendredi prochain"
//! - "vendredi dernier"
//! - "vendredi prochain à 15h"

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::fr as dict;
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use crate::scanner::TokenType;
use chrono::{Datelike, Duration};
use fancy_regex::Regex;
use std::sync::LazyLock;

static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)(?P<weekday>lundi|mardi|mercredi|jeudi|vendredi|samedi|dimanche)(?:\s+(?P<modifier>prochain|dernier|passé|passe))?(?:\s+(?:à\s+)?(?P<hour>\d{1,2})(?:h(?P<minute>\d{2})?)?)?"
    ).unwrap()
});

/// French weekday parser
pub struct FRWeekdayParser;

impl FRWeekdayParser {
    pub fn new() -> Self {
        Self
    }
}

impl Parser for FRWeekdayParser {
    fn name(&self) -> &'static str {
        "FRWeekdayParser"
    }

    fn should_apply(&self, context: &ParsingContext) -> bool {
        context.has_token_type(TokenType::Weekday)
    }

    fn parse(&self, context: &ParsingContext) -> Result<Vec<ParsedResult>> {
        let mut results = Vec::new();
        let ref_date = context.reference.instant;

        let mut start = 0;
        while start < context.text.len() {
            let search_text = &context.text[start..];
            let mat = match PATTERN.find(search_text) {
                Ok(Some(m)) => m,
                Ok(None) => break,
                Err(_) => break,
            };

            let matched_text = mat.as_str();
            let index = start + mat.start();

            let caps = match PATTERN.captures(matched_text) {
                Ok(Some(c)) => c,
                Ok(None) => {
                    start = index + 1;
                    continue;
                }
                Err(_) => {
                    start = index + 1;
                    continue;
                }
            };

            let weekday_str = caps
                .name("weekday")
                .map(|m| m.as_str().to_lowercase())
                .unwrap_or_default();

            let weekday = match dict::get_weekday(&weekday_str) {
                Some(w) => w,
                None => {
                    start = index + 1;
                    continue;
                }
            };

            let modifier = caps.name("modifier").map(|m| m.as_str().to_lowercase());

            // Calculate target weekday (using Monday as start of week for French)
            let ref_weekday = ref_date.weekday().num_days_from_monday() as i32;
            // Convert our Weekday enum (Sunday=0) to Monday-based (Monday=0, Sunday=6)
            let target_weekday = match weekday as i32 {
                0 => 6,     // Sunday
                n => n - 1, // Monday=0, Tuesday=1, etc.
            };

            let days_diff = match modifier.as_deref() {
                Some("prochain") => {
                    // Next occurrence, always in the future (at least 1 day ahead)
                    let diff = target_weekday - ref_weekday;
                    if diff <= 0 { diff + 7 } else { diff }
                }
                Some("dernier") | Some("passé") | Some("passe") => {
                    // Last occurrence, always in the past
                    let diff = target_weekday - ref_weekday;
                    if diff >= 0 { diff - 7 } else { diff }
                }
                None | Some(_) => {
                    // Default: find the weekday in the current week
                    // This means the difference is simply target - reference
                    // Monday of this week could be in the past, Sunday could be in the future
                    target_weekday - ref_weekday
                }
            };

            let target_date = ref_date + Duration::days(days_diff as i64);

            let mut components = context.create_components();
            components.assign(Component::Year, target_date.year());
            components.assign(Component::Month, target_date.month() as i32);
            components.assign(Component::Day, target_date.day() as i32);
            components.assign(Component::Weekday, weekday as i32);

            // Handle time if present
            if let Some(hour_match) = caps.name("hour") {
                let hour: i32 = hour_match.as_str().parse().unwrap_or(0);
                let minute: i32 = caps
                    .name("minute")
                    .and_then(|m| m.as_str().parse().ok())
                    .unwrap_or(0);
                components.assign(Component::Hour, hour);
                components.assign(Component::Minute, minute);
            }

            results.push(context.create_result(
                index,
                index + matched_text.len(),
                components,
                None,
            ));

            start = index + matched_text.len();
        }

        Ok(results)
    }
}

impl Default for FRWeekdayParser {
    fn default() -> Self {
        Self::new()
    }
}
