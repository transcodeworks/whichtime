//! Merge weekday and date refiner - combines adjacent weekday and date results
//!
//! Handles patterns like:
//! - "Sonntag 7.12.2014" -> single result with weekday and date
//! - "Freitag 30.12.16" -> single result with weekday and date
//! - "Sonntag, den 7. Dezember 2014" -> single result

use crate::components::Component;
use crate::context::ParsingContext;
use crate::refiners::Refiner;
use crate::results::ParsedResult;

/// Refiner that merges weekday matches with adjacent explicit dates.
pub struct MergeWeekdayDateRefiner;

impl MergeWeekdayDateRefiner {
    fn has_explicit_date(result: &ParsedResult) -> bool {
        result.start.is_certain(Component::Month) && result.start.is_certain(Component::Day)
    }

    fn merge_results(
        context: &ParsingContext,
        first: &ParsedResult,
        second: &ParsedResult,
        weekday_source: &ParsedResult,
        date_source: &ParsedResult,
    ) -> ParsedResult {
        let mut merged_components = date_source.start;

        if let Some(weekday) = weekday_source.start.get(Component::Weekday) {
            merged_components.assign(Component::Weekday, weekday);
        }

        let merged_end = if date_source.end.is_some() {
            date_source.end
        } else {
            weekday_source.end
        };

        let range_start = first.index.min(second.index);
        let range_end = first.end_index.max(second.end_index);

        ParsedResult::new(
            context.reference,
            range_start,
            &context.text[range_start..range_end],
            merged_components,
            merged_end,
        )
    }
}

impl Refiner for MergeWeekdayDateRefiner {
    fn refine(&self, context: &ParsingContext, results: Vec<ParsedResult>) -> Vec<ParsedResult> {
        if results.len() < 2 {
            return results;
        }

        let mut merged = Vec::with_capacity(results.len());
        let mut i = 0;

        while i < results.len() {
            let current = &results[i];

            // Check if next result exists and could be merged
            if i + 1 < results.len() {
                let next = &results[i + 1];

                // Check if they're adjacent (with optional whitespace/connectors)
                let gap_start = current.end_index;
                let gap_end = next.index;

                if gap_end >= gap_start {
                    let gap = if gap_end > gap_start {
                        &context.text[gap_start..gap_end]
                    } else {
                        ""
                    };
                    let gap_trimmed = gap.trim();

                    // Check for German connectors: ",", ", den", just whitespace
                    let is_connectable = gap_trimmed.is_empty()
                        || gap_trimmed == ","
                        || gap_trimmed == ", den"
                        || gap_trimmed == ",den"
                        || gap_trimmed == "den"
                        || gap_trimmed == "、"
                        || gap_trimmed == "，"
                        || gap_trimmed == "の";

                    if is_connectable {
                        let current_has_weekday = current.start.get(Component::Weekday).is_some();
                        let next_has_weekday = next.start.get(Component::Weekday).is_some();
                        let current_has_date = Self::has_explicit_date(current);
                        let next_has_date = Self::has_explicit_date(next);

                        if current_has_weekday && next_has_date && !next_has_weekday {
                            let merged_result =
                                Self::merge_results(context, current, next, current, next);
                            merged.push(merged_result);
                            i += 2;
                            continue;
                        }

                        if current_has_date && !current_has_weekday && next_has_weekday {
                            let merged_result =
                                Self::merge_results(context, current, next, next, current);
                            merged.push(merged_result);
                            i += 2;
                            continue;
                        }
                    }
                }
            }

            merged.push(results[i].clone());
            i += 1;
        }

        merged
    }
}
