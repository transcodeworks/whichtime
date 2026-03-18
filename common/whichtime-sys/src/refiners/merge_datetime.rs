//! Merge date and time refiner - combines adjacent date and time results

use crate::components::Component;
use crate::context::ParsingContext;
use crate::refiners::Refiner;
use crate::results::ParsedResult;

/// Refiner that combines adjacent date-only and time-only matches.
pub struct MergeDateTimeRefiner;

impl Refiner for MergeDateTimeRefiner {
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
                    let gap_lower = gap.trim().to_lowercase();
                    let is_connectable = gap_lower.is_empty()
                        || gap_lower == "at"
                        || gap_lower == ","
                        || gap_lower == "on"
                        || gap_lower == "um"  // German
                        || gap_lower == "a"   // Spanish/Italian
                        || gap_lower == "à"   // French
                        || gap_lower == "om"  // Dutch
                        || gap_lower == "в"   // Russian
                        || gap_lower == "に"  // Japanese
                        || gap_lower == "の"  // Japanese possessive
                        || gap_lower == "о"; // Ukrainian

                    if is_connectable {
                        // Check if one is date-only and other is time-only
                        let current_is_date = current.start.is_only_date();
                        let current_is_time = current.start.is_only_time();
                        let next_is_date = next.start.is_only_date();
                        let next_is_time = next.start.is_only_time();

                        if (current_is_date && next_is_time) || (current_is_time && next_is_date) {
                            // Merge them
                            let (date_result, time_result) = if current_is_date {
                                (current, next)
                            } else {
                                (next, current)
                            };

                            let mut merged_components = date_result.start;

                            // When merging date and time, make the year certain
                            // This prevents ForwardDateRefiner from moving it forward
                            if let Some(year) = merged_components.get(Component::Year) {
                                merged_components.assign(Component::Year, year);
                            }

                            // Copy time components
                            if let Some(hour) = time_result.start.get(Component::Hour) {
                                merged_components.assign(Component::Hour, hour);
                            }
                            if let Some(minute) = time_result.start.get(Component::Minute) {
                                merged_components.assign(Component::Minute, minute);
                            }
                            if let Some(second) = time_result.start.get(Component::Second) {
                                merged_components.assign(Component::Second, second);
                            }
                            if let Some(meridiem) = time_result.start.get(Component::Meridiem) {
                                merged_components.assign(Component::Meridiem, meridiem);
                            }

                            let merged_end = if let Some(mut end_comp) = time_result.end {
                                if let Some(year) = date_result.start.get(Component::Year) {
                                    end_comp.assign(Component::Year, year);
                                }
                                if let Some(month) = date_result.start.get(Component::Month) {
                                    end_comp.assign(Component::Month, month);
                                }
                                if let Some(day) = date_result.start.get(Component::Day) {
                                    end_comp.assign(Component::Day, day);
                                }
                                Some(end_comp)
                            } else {
                                date_result.end
                            };

                            let merged_result = ParsedResult::new(
                                context.reference,
                                current.index,
                                &context.text[current.index..next.end_index],
                                merged_components,
                                merged_end,
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
