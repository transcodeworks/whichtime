//! Forward date refiner - adjusts dates to be in the future when ambiguous

use crate::components::Component;
use crate::context::ParsingContext;
use crate::dictionaries::Locale;
use crate::refiners::Refiner;
use crate::results::ParsedResult;
use chrono::Datelike;

/// Refiner that pushes ambiguous calendar dates into the future when needed.
pub struct ForwardDateRefiner;

impl Refiner for ForwardDateRefiner {
    fn refine(&self, context: &ParsingContext, results: Vec<ParsedResult>) -> Vec<ParsedResult> {
        if matches!(context.locale, Locale::Ja) {
            return results;
        }

        let ref_date = context.reference.instant;

        results
            .into_iter()
            .map(|mut result| {
                // Only adjust if year is not certain
                if !result.start.is_certain(Component::Year)
                    && let (Some(month), Some(day)) = (
                        result.start.get(Component::Month),
                        result.start.get(Component::Day),
                    )
                {
                    let ref_month = ref_date.month() as i32;
                    let ref_day = ref_date.day() as i32;

                    // If the date is in the past this year, move to next year
                    if month < ref_month || (month == ref_month && day < ref_day) {
                        result.start.assign(Component::Year, ref_date.year() + 1);
                    }
                }
                result
            })
            .collect()
    }
}
