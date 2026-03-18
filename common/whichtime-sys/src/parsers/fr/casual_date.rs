//! French casual date parser
//!
//! Handles French casual date expressions like:
//! - "maintenant", "aujourd'hui", "demain", "hier"
//! - "ce matin", "cet après-midi", "ce soir"
//! - "la veille", "après-demain", "avant-hier"
//! - Combined: "demain matin 11h", "hier à 18h"

use crate::components::Component;
use crate::context::ParsingContext;
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use crate::types::Meridiem;
use chrono::{Datelike, Duration, Timelike};
use fancy_regex::Regex;
use std::sync::LazyLock;

static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)(?<![a-zA-ZàâäéèêëïîôùûüÿçœæÀÂÄÉÈÊËÏÎÔÙÛÜŸÇŒÆ])(maintenant|aujourd'hui|aujourdhui|demain|hier|la\s+veille|après-demain|apres-demain|après\s+demain|apres\s+demain|avant-hier|avant\s+hier|ce\s+matin|cet\s+après-midi|cet\s+apres-midi|cet\s+aprem|ce\s+soir)(?:\s+(matin|après-midi|apres-midi|soir|nuit))?(?:\s+(?:à\s+)?(midi|minuit|(\d{1,2})(?:[:h](\d{2})?)?)?)?(?=\W|$)"
    ).unwrap()
});

const DATE_GROUP: usize = 1;
const TIME_PERIOD_GROUP: usize = 2;
const CASUAL_TIME_GROUP: usize = 3;
const HOUR_GROUP: usize = 4;
const MINUTE_GROUP: usize = 5;

/// French casual date parser
pub struct FRCasualDateParser;

impl FRCasualDateParser {
    pub fn new() -> Self {
        Self
    }
}

impl Parser for FRCasualDateParser {
    fn name(&self) -> &'static str {
        "FRCasualDateParser"
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
                .map(|m| m.as_str().to_lowercase())
                .unwrap_or_default();

            let time_period = captures
                .get(TIME_PERIOD_GROUP)
                .map(|m| m.as_str().to_lowercase());

            let mut components = context.create_components();
            let target_date;

            // Process date keyword
            match date_keyword.as_str() {
                "maintenant" => {
                    components.assign(Component::Year, ref_date.year());
                    components.assign(Component::Month, ref_date.month() as i32);
                    components.assign(Component::Day, ref_date.day() as i32);
                    components.assign(Component::Hour, ref_date.hour() as i32);
                    components.assign(Component::Minute, ref_date.minute() as i32);
                    components.assign(Component::Second, ref_date.second() as i32);
                }
                "aujourd'hui" | "aujourdhui" => {
                    components.assign(Component::Year, ref_date.year());
                    components.assign(Component::Month, ref_date.month() as i32);
                    components.assign(Component::Day, ref_date.day() as i32);
                }
                "demain" => {
                    target_date = ref_date + Duration::days(1);
                    components.assign(Component::Year, target_date.year());
                    components.assign(Component::Month, target_date.month() as i32);
                    components.assign(Component::Day, target_date.day() as i32);
                }
                "hier" => {
                    target_date = ref_date - Duration::days(1);
                    components.assign(Component::Year, target_date.year());
                    components.assign(Component::Month, target_date.month() as i32);
                    components.assign(Component::Day, target_date.day() as i32);
                }
                _ if date_keyword.contains("veille") => {
                    // "la veille" = the day before
                    target_date = ref_date - Duration::days(1);
                    components.assign(Component::Year, target_date.year());
                    components.assign(Component::Month, target_date.month() as i32);
                    components.assign(Component::Day, target_date.day() as i32);
                    components.imply(Component::Hour, 0);
                }
                _ if date_keyword.contains("après") && date_keyword.contains("demain")
                    || date_keyword.contains("apres") && date_keyword.contains("demain") =>
                {
                    // "après-demain" = day after tomorrow
                    target_date = ref_date + Duration::days(2);
                    components.assign(Component::Year, target_date.year());
                    components.assign(Component::Month, target_date.month() as i32);
                    components.assign(Component::Day, target_date.day() as i32);
                }
                _ if date_keyword.contains("avant") && date_keyword.contains("hier") => {
                    // "avant-hier" = day before yesterday
                    target_date = ref_date - Duration::days(2);
                    components.assign(Component::Year, target_date.year());
                    components.assign(Component::Month, target_date.month() as i32);
                    components.assign(Component::Day, target_date.day() as i32);
                }
                _ if date_keyword.contains("ce") && date_keyword.contains("matin") => {
                    // "ce matin" = this morning
                    components.assign(Component::Year, ref_date.year());
                    components.assign(Component::Month, ref_date.month() as i32);
                    components.assign(Component::Day, ref_date.day() as i32);
                    components.imply(Component::Hour, 8);
                    components.assign(Component::Meridiem, Meridiem::AM as i32);
                }
                _ if date_keyword.contains("cet")
                    && (date_keyword.contains("après-midi")
                        || date_keyword.contains("apres-midi")
                        || date_keyword.contains("aprem")) =>
                {
                    // "cet après-midi" or "cet aprem" = this afternoon
                    components.assign(Component::Year, ref_date.year());
                    components.assign(Component::Month, ref_date.month() as i32);
                    components.assign(Component::Day, ref_date.day() as i32);
                    components.imply(Component::Hour, 14);
                    components.assign(Component::Meridiem, Meridiem::PM as i32);
                }
                _ if date_keyword.contains("ce") && date_keyword.contains("soir") => {
                    // "ce soir" = this evening
                    components.assign(Component::Year, ref_date.year());
                    components.assign(Component::Month, ref_date.month() as i32);
                    components.assign(Component::Day, ref_date.day() as i32);
                    components.imply(Component::Hour, 18);
                    components.assign(Component::Meridiem, Meridiem::PM as i32);
                }
                _ => {
                    start = match_end;
                    continue;
                }
            }

            // Apply time period modifier if present
            if let Some(ref period) = time_period {
                match period.as_str() {
                    "matin" => {
                        components.imply(Component::Hour, 8);
                        components.assign(Component::Meridiem, Meridiem::AM as i32);
                    }
                    "après-midi" | "apres-midi" => {
                        components.imply(Component::Hour, 14);
                        components.assign(Component::Meridiem, Meridiem::PM as i32);
                    }
                    "soir" => {
                        components.imply(Component::Hour, 18);
                        components.assign(Component::Meridiem, Meridiem::PM as i32);
                    }
                    "nuit" => {
                        components.imply(Component::Hour, 22);
                        components.assign(Component::Meridiem, Meridiem::PM as i32);
                    }
                    _ => {}
                }
            }

            // Handle casual time: "à midi" or "à minuit"
            if let Some(casual_time) = captures.get(CASUAL_TIME_GROUP) {
                let time_word = casual_time.as_str().to_lowercase();
                if time_word == "midi" {
                    components.assign(Component::Hour, 12);
                    components.assign(Component::Minute, 0);
                    components.assign(Component::Meridiem, Meridiem::PM as i32);
                } else if time_word == "minuit" {
                    components.assign(Component::Hour, 0);
                    components.assign(Component::Minute, 0);
                }
            }

            // Handle explicit time: "à 18h" or "17:00"
            if let Some(hour_match) = captures.get(HOUR_GROUP) {
                let hour: i32 = hour_match.as_str().parse().unwrap_or(0);
                let minute: i32 = captures
                    .get(MINUTE_GROUP)
                    .map(|m| m.as_str().parse().unwrap_or(0))
                    .unwrap_or(0);

                components.assign(Component::Hour, hour);
                components.assign(Component::Minute, minute);

                if hour >= 12 {
                    components.assign(Component::Meridiem, Meridiem::PM as i32);
                } else {
                    components.assign(Component::Meridiem, Meridiem::AM as i32);
                }
            }

            results.push(context.create_result(match_start, match_end, components, None));

            start = match_end;
        }

        Ok(results)
    }
}

impl Default for FRCasualDateParser {
    fn default() -> Self {
        Self::new()
    }
}
