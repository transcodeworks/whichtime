//! Merge date range refiner - combines date results separated by range connectors
//!
//! Handles patterns like:
//! - "heute - nächsten Freitag" -> single result with start and end
//! - "10. August - 22. August" -> single result with start and end

use crate::context::ParsingContext;
use crate::refiners::Refiner;
use crate::results::ParsedResult;

/// Refiner that converts adjacent date matches into a range result.
pub struct MergeDateRangeRefiner;

impl Refiner for MergeDateRangeRefiner {
    fn refine(&self, context: &ParsingContext, results: Vec<ParsedResult>) -> Vec<ParsedResult> {
        if results.len() < 2 {
            return results;
        }

        let mut merged = Vec::with_capacity(results.len());
        let mut i = 0;

        while i < results.len() {
            let current = &results[i];

            // Check if next result exists and could be merged as a range
            if i + 1 < results.len() {
                let next = &results[i + 1];

                // Check if they're separated by a range connector
                let gap_start = current.end_index;
                let gap_end = next.index;

                if gap_end > gap_start {
                    let gap = &context.text[gap_start..gap_end];
                    let gap_trimmed = gap.trim();

                    // Check for range connectors: "-", "–", "bis"
                    let is_range_connector = gap_trimmed == "-"
                        || gap_trimmed == "–"
                        || gap_trimmed.eq_ignore_ascii_case("bis");

                    if is_range_connector {
                        // Both should be date-like results (not pure time)
                        let current_has_date = current.start.get(crate::Component::Day).is_some()
                            || current.start.get(crate::Component::Month).is_some();
                        let next_has_date = next.start.get(crate::Component::Day).is_some()
                            || next.start.get(crate::Component::Month).is_some();

                        if current_has_date && next_has_date {
                            // Merge them as a range
                            let merged_result = ParsedResult::new(
                                context.reference,
                                current.index,
                                &context.text[current.index..next.end_index],
                                current.start,
                                Some(next.start),
                            );

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
