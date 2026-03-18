//! French time unit relative parser
//!
//! Handles French relative time expressions like:
//! - "la semaine prochaine"
//! - "le mois dernier"
//! - "les 2 prochaines semaines"
//! - "les trois prochaines semaines"

use crate::components::Component;
use crate::context::ParsingContext;
use crate::error::Result;
use crate::parsers::Parser;
use crate::results::ParsedResult;
use chrono::{Datelike, Duration, Timelike};
use fancy_regex::Regex;
use std::sync::LazyLock;

// Pattern for "la semaine prochaine", "le mois dernier", "les 2 prochaines semaines"
static PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)(?:l[ae]s?)\s+(?:(\d+|deux|trois|quatre|cinq|six|sept|huit|neuf|dix|une?)\s+)?(?:(prochaine?s?|dernière?s?|dernier|passée?s?|passe)\s+)?(semaines?|mois|ans?|années?|annees?)(?:\s+(prochaine?|dernière?|dernier|passée?|passe))?\b"
    ).unwrap()
});

/// French time unit relative parser
pub struct FRTimeUnitRelativeParser;

impl FRTimeUnitRelativeParser {
    pub fn new() -> Self {
        Self
    }

    fn parse_number(s: &str) -> i32 {
        match s.to_lowercase().as_str() {
            "un" | "une" => 1,
            "deux" => 2,
            "trois" => 3,
            "quatre" => 4,
            "cinq" => 5,
            "six" => 6,
            "sept" => 7,
            "huit" => 8,
            "neuf" => 9,
            "dix" => 10,
            _ => s.parse().unwrap_or(1),
        }
    }
}

impl Parser for FRTimeUnitRelativeParser {
    fn name(&self) -> &'static str {
        "FRTimeUnitRelativeParser"
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

            // Get number (default 1)
            let num = caps
                .get(1)
                .map(|m| Self::parse_number(m.as_str()))
                .unwrap_or(1);

            // Get modifier (prochaine/dernier before or after unit)
            let modifier_before = caps.get(2).map(|m| m.as_str().to_lowercase());
            let modifier_after = caps.get(4).map(|m| m.as_str().to_lowercase());
            let modifier = modifier_before.or(modifier_after).unwrap_or_default();

            // Get time unit
            let unit = caps
                .get(3)
                .map(|m| m.as_str().to_lowercase())
                .unwrap_or_default();

            // Determine direction
            let is_future = modifier.contains("prochain");
            let multiplier = if is_future { 1 } else { -1 };
            let amount = num * multiplier;

            // Calculate target date
            let target_date = match unit.as_str() {
                "semaine" | "semaines" => ref_date + Duration::weeks(amount as i64),
                "mois" => {
                    let new_month = ref_date.month() as i32 + amount;
                    let (year_offset, month) = if new_month <= 0 {
                        ((new_month - 12) / 12, ((new_month - 1) % 12 + 13) as u32)
                    } else if new_month > 12 {
                        ((new_month - 1) / 12, ((new_month - 1) % 12 + 1) as u32)
                    } else {
                        (0, new_month as u32)
                    };
                    let new_year = ref_date.year() + year_offset;
                    ref_date
                        .with_year(new_year)
                        .and_then(|d| d.with_month(month))
                        .unwrap_or(ref_date)
                }
                "an" | "année" | "annee" | "ans" | "années" | "annees" => {
                    let new_year = ref_date.year() + amount;
                    ref_date.with_year(new_year).unwrap_or(ref_date)
                }
                _ => {
                    start = index + matched_text.len();
                    continue;
                }
            };

            let mut components = context.create_components();
            components.assign(Component::Year, target_date.year());
            components.assign(Component::Month, target_date.month() as i32);
            components.assign(Component::Day, target_date.day() as i32);

            // Preserve time from reference
            components.imply(Component::Hour, ref_date.hour() as i32);
            components.imply(Component::Minute, ref_date.minute() as i32);
            components.imply(Component::Second, ref_date.second() as i32);

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

impl Default for FRTimeUnitRelativeParser {
    fn default() -> Self {
        Self::new()
    }
}
