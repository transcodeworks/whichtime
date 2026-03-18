//! Overlap removal refiner - removes overlapping results, keeping longer matches

use crate::context::ParsingContext;
use crate::refiners::Refiner;
use crate::results::ParsedResult;

/// Refiner that removes overlapping matches and prefers the longest span.
pub struct OverlapRemovalRefiner;

impl Refiner for OverlapRemovalRefiner {
    fn refine(
        &self,
        _context: &ParsingContext,
        mut results: Vec<ParsedResult>,
    ) -> Vec<ParsedResult> {
        if results.len() < 2 {
            return results;
        }

        // Sort by start index, then by end index (longer first)
        results.sort_by(|a, b| {
            a.index
                .cmp(&b.index)
                .then_with(|| b.end_index.cmp(&a.end_index))
        });

        let mut filtered = Vec::with_capacity(results.len());
        let mut last_end = 0;

        for result in results {
            // If this result starts after (or at) the end of the previous, keep it
            if result.index >= last_end {
                last_end = result.end_index;
                filtered.push(result);
            } else if result.end_index > last_end {
                // This result overlaps but extends further - check if we should replace
                let last = filtered.last();
                if let Some(prev) = last {
                    let prev_len = prev.end_index - prev.index;
                    let curr_len = result.end_index - result.index;
                    if curr_len > prev_len {
                        filtered.pop();
                        last_end = result.end_index;
                        filtered.push(result);
                    }
                }
            }
            // Otherwise, skip this result (it's fully contained in the previous)
        }

        filtered
    }
}
